mod classes;
mod map_type;

use super::*;
// use cpp::{fclass, CppContext};
use ast::TyParamsSubstList;
use error::{ResultDiagnostic, ResultSynDiagnostic, SourceIdSpan};
use file_cache::FileWriteCache;
use itertools::Itertools;
use map_type::{DotNetForeignMethodSignature, NameGenerator};
use petgraph::Direction;
use quote::quote;
use rustc_hash::FxHashSet;
use smol_str::SmolStr;
use std::{
    collections::{HashSet, HashMap},
    fs::{self, File},
    rc::Rc,
};
use syn::{parse_str, Ident, Type};
use typemap::{
    ast,
    ty::{ForeignTypeS, RustType},
    utils::{self, ForeignMethodSignature, ForeignTypeInfoT},
    ForeignTypeInfo, MapToForeignFlag, FROM_VAR_TEMPLATE, TO_VAR_TEMPLATE,
};
use types::{FnArg, ForeignerClassInfo, ForeignerMethod, MethodVariant, SelfTypeVariant};

pub struct DotNetGenerator<'a> {
    config: &'a DotNetConfig,
    conv_map: &'a mut TypeMap,
    rust_code: Vec<TokenStream>,
    cs_file: FileWriteCache,
    additional_cs_code_for_types: HashMap<SmolStr, String>,
    known_c_items_modules: HashSet<SmolStr>,
}

impl<'a> DotNetGenerator<'a> {
    fn new(config: &'a DotNetConfig, conv_map: &'a mut TypeMap) -> Result<Self> {
        let mut generated_files_registry = FxHashSet::default();
        let cs_file = Self::create_cs_project(config, &mut generated_files_registry)?;

        Ok(Self {
            config,
            conv_map,
            rust_code: Vec::new(),
            cs_file,
            additional_cs_code_for_types: HashMap::new(),
            known_c_items_modules: HashSet::new()
        })
    }

    fn generate(mut self, items: Vec<ItemToExpand>) -> Result<Vec<TokenStream>> {
        // let void_type = self.conv_map.find_or_alloc_rust_type_no_src_id(&parse_type!(*mut std::os::raw::c_void));
        // self.conv_map.add_foreign_rust_ty_idx(foreign_name, correspoding_rty)
        // self.conv_map.add_foreign(, foreign_name);

        for item in items {
            match item {
                ItemToExpand::Class(fclass) => {
                    self.generate_class(&fclass)?;
                }
                _ => unimplemented!(), // ItemToExpand::Enum(fenum) => fenum::generate_enum(&mut ctx, &fenum)?,
                                       // ItemToExpand::Interface(finterface) => {
                                       //     finterface::generate_interface(&mut ctx, &finterface)?
                                       // }
            }
        }

        self.finish()?;
        self.cs_file.update_file_if_necessary()?;
        Ok(self.rust_code)
    }

    fn create_cs_project(
        config: &'a DotNetConfig,
        generated_files_registry: &mut FxHashSet<PathBuf>,
    ) -> Result<FileWriteCache> {
        fs::create_dir_all(&config.managed_lib_name).expect("Can't create managed lib directory");

        let mut csproj = File::create(format!("{0}/{0}.csproj", config.managed_lib_name))
            .with_note("Can't create csproj file")?;

        write!(
            csproj,
            r#"
<Project Sdk="Microsoft.NET.Sdk">

<PropertyGroup>
    <TargetFramework>netstandard2.0</TargetFramework>
</PropertyGroup>

</Project>
"#,
        )
        .with_note("Can't write to csproj file")?;

        let cs_file_name = config.managed_lib_name.clone() + ".cs";
        let mut cs_file = FileWriteCache::new(
            PathBuf::from(&config.managed_lib_name).join(cs_file_name),
            generated_files_registry,
        );

        write!(
            cs_file,
            r#"
// Generated by rust_swig. Do not edit.

using System;
using System.Runtime.InteropServices;

namespace {managed_lib_name}
{{

    internal static class RustInterop {{
        [DllImport("{native_lib_name}", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void String_delete(IntPtr c_char_ptr);
    }}
"#,
            managed_lib_name = config.managed_lib_name,
            native_lib_name = config.native_lib_name,
        )
        .with_note("Write to memory failed")?;

        Ok(cs_file)
    }

    fn generate_class(&mut self, class: &ForeignerClassInfo) -> Result<()> {
        //self.conv_map.register_foreigner_class(fclass);
        classes::register_class(self.conv_map, class)?;
        self.generate_swig_trait_for_class(class)?;
        self.generate_rust_destructor(class)?;
        self.generate_dotnet_class_code(class)?;

        for method in &class.methods {
            self.generate_method(&class, method)?;
        }

        writeln!(self.cs_file, "}} // class").with_note("Write to memory failed")?;

        Ok(())
    }

    fn class_storage_type(&self, class: &ForeignerClassInfo) -> Option<RustType> {
        Some(
            self.conv_map
                .ty_to_rust_type(&class.self_desc.as_ref()?.constructor_ret_type),
        )
    }

    fn generate_swig_trait_for_class(&mut self, class: &ForeignerClassInfo) -> Result<()> {
        if let Some(self_description) = class.self_desc.as_ref() {
            let class_ty = &self_description.self_type;
            let fclass_impl_code: TokenStream = quote! {
                impl SwigForeignClassStorage for Box<#class_ty> {
                    type BaseType = #class_ty;

                    fn swig_as_ref(&self) -> &Self::BaseType {
                        self.as_ref()
                    }
                    fn swig_as_mut(&mut self) -> &mut Self::BaseType {
                        self.as_mut()
                    }
                    fn swig_cloned(&self) -> Self::BaseType {
                        self.as_ref().clone()
                    }
                    fn swig_leak_into_raw(mut self) -> *mut Self {
                        // Yes. We need to wrap it into one more Box, for the returning pointer to remain valid.
                        Box::into_raw(Box::new(self))
                    }
                    fn swig_drop_raw(raw_ptr: *mut Self) {
                        unsafe { ::std::mem::drop(Box::from_raw(raw_ptr)) };
                    }
                }

                impl SwigForeignClass for #class_ty {
                    type StorageType = Box<#class_ty>;

                    fn swig_into_storage_type(self) -> Self::StorageType {
                        Self::StorageType::new(self)
                    }
                }
            };
            self.rust_code.push(fclass_impl_code);
        }

        // if let Some(this_type) = self.class_storage_type(class) {
        //     let (_, code_box_this) =
        //         utils::convert_to_heap_pointer(self.conv_map, &this_type, "this");
        //     let lifetimes = ast::list_lifetimes(&this_type.ty);
        //     let unpack_code = utils::unpack_from_heap_pointer(&this_type, TO_VAR_TEMPLATE, true);
        //     let class_name = &this_type.ty;
        //     let unpack_code = unpack_code.replace(TO_VAR_TEMPLATE, "p");
        //     let unpack_code: TokenStream = syn::parse_str(&unpack_code).unwrap_or_else(|err| {
        //         error::panic_on_syn_error(
        //             "internal/c++ foreign class unpack code",
        //             unpack_code,
        //             err,
        //         )
        //     });
        //     let this_type_ty = this_type.to_type_without_lifetimes();
        //     let fclass_impl_code: TokenStream = quote! {
        //         impl<#(#lifetimes),*> SwigForeignClass for #class_name {
        //             // fn c_class_name() -> *const ::std::os::raw::c_char {
        //             //     swig_c_str!(stringify!(#class_name))
        //             // }
        //             fn box_object(this: Self) -> *mut ::std::os::raw::c_void {
        //                 #code_box_this
        //                 this as *mut ::std::os::raw::c_void
        //             }
        //             fn unbox_object(p: *mut ::std::os::raw::c_void) -> Self {
        //                 let p = p as *mut #this_type_ty;
        //                 #unpack_code
        //                 p
        //             }
        //         }
        //     };
        //     self.rust_code.push(fclass_impl_code);
        // }
        Ok(())
    }

    fn generate_rust_destructor(&mut self, class: &ForeignerClassInfo) -> Result<()> {
        // Do not generate destructor for static classes.
        if let Some(self_desc) = class.self_desc.as_ref() {
            let class_name = &class.name;
            let self_ty = &self_desc.self_type;
            //let storage_ty = &self_desc.constructor_ret_type;
            // let storage_ty = 
            let destructor_name = parse_str::<Ident>(&format!("{}_delete", class_name)).unwrap();

            let destructor_code = quote! {
                #[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
                #[no_mangle]
                pub extern "C" fn #destructor_name(this: *mut <#self_ty as SwigForeignClass>::StorageType) {
                    <#self_ty as SwigForeignClass>::StorageType::swig_drop_raw(this);
                }
            };
            self.rust_code.push(destructor_code);
        }
        Ok(())
    }

    fn generate_dotnet_class_code(&mut self, class: &ForeignerClassInfo) -> Result<()> {
        let class_name = class.name.to_string();

        if let Some(_) = class.self_desc {
            let rust_destructor_name = class_name.clone() + "_delete";

            write!(
                self.cs_file,
                r#"public class {class_name}: IDisposable {{
        internal IntPtr nativePtr;

        internal {class_name}(IntPtr nativePtr) {{
            this.nativePtr = nativePtr;
        }}

        public void Dispose() {{
            DoDispose();
            GC.SuppressFinalize(this);
        }}

        private void DoDispose() {{
            if (nativePtr != IntPtr.Zero) {{
                {rust_destructor_name}(nativePtr);
                nativePtr = IntPtr.Zero;
            }}
        }}

        [DllImport("{native_lib_name}", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void {rust_destructor_name}(IntPtr __this);

        ~{class_name}() {{
            DoDispose();
        }}
"#,
                class_name = class_name,
                rust_destructor_name = rust_destructor_name,
                native_lib_name = self.config.native_lib_name,
            )
            .with_note("Write to memory failed")?;
        } else {
            writeln!(
                self.cs_file,
                "public static class {class_name} {{",
                class_name = class_name,
            )
            .with_note("Write to memory failed")?;
        }

        Ok(())
    }

    fn generate_method(
        &mut self,
        class: &ForeignerClassInfo,
        method: &ForeignerMethod,
    ) -> Result<()> {
        let foreign_method_signature = map_type::make_foreign_method_signature(
            self,
            class,
            method,
        )?;

        self.write_rust_glue_code(class, &foreign_method_signature)?;
        self.write_pinvoke_function_signature(class, &foreign_method_signature)?;
        self.write_dotnet_wrapper_function(class, &foreign_method_signature)?;

        Ok(())
    }

    fn write_rust_glue_code(
        &mut self,
        class: &ForeignerClassInfo,
        foreign_method_signature: &DotNetForeignMethodSignature,
    ) -> Result<()> {
        let method_name = &foreign_method_signature.name;
        let full_method_name = format!("{}_{}", class.name, method_name);

        let convert_input_code = itertools::process_results(
            foreign_method_signature
                .input
                .iter()
                .map(|arg| -> Result<String> { 
                    let (mut deps, conversion) = arg.rust_conversion_code(self.conv_map)?;
                    self.rust_code.append(&mut deps);
                    Ok(conversion)
                }),
            |mut iter| iter.join(""),
        )?;

        let rust_func_args_str = foreign_method_signature
            .input
            .iter()
            .map(|arg_info| {
                format!(
                    "{}: {}",
                    arg_info.arg_name.rust_variable_name(),
                    arg_info.type_info.rust_intermediate_type.typename()
                )
            })
            .join(", ");

        let (mut deps, convert_output_code) = foreign_method_signature
            .output
            .rust_conversion_code(self.conv_map)?;
        self.rust_code.append(&mut deps);

        let rust_code_str = format!(
            r#"
    #[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
    #[no_mangle]
    pub extern "C" fn {func_name}({func_args}) -> {return_type} {{
        {convert_input_code}
        let mut {ret_name} = {call};
        {convert_output_code}
        {ret_name}
    }}
"#,
            func_name = full_method_name,
            func_args = rust_func_args_str,
            return_type = foreign_method_signature
                .output
                .type_info
                .rust_intermediate_type,
            convert_input_code = convert_input_code,
            ret_name = foreign_method_signature
                .output
                .arg_name
                .rust_variable_name(),
            convert_output_code = convert_output_code,
            call = foreign_method_signature.rust_function_call,
        );
        self.rust_code
            .push(syn::parse_str(&rust_code_str).with_syn_src_id(class.src_id)?);
        Ok(())
    }

    fn write_pinvoke_function_signature(
        &mut self,
        class: &ForeignerClassInfo,
        foreign_method_signature: &DotNetForeignMethodSignature,
    ) -> Result<()> {
        let method_name = &foreign_method_signature.name;
        let full_method_name = format!("{}_{}", class.name, method_name);
        let pinvoke_args_str = foreign_method_signature
            .input
            .iter()
            .map(|a| {
                format!(
                    "{} {}",
                    a.type_info.dotnet_intermediate_type,
                    a.arg_name.dotnet_variable_name()
                )
            })
            .join(", ");
        write!(
            self.cs_file,
            r#"
        //[SuppressUnmanagedCodeSecurity]
        [DllImport("{native_lib_name}", CallingConvention = CallingConvention.Cdecl)]
        internal static extern {return_type} {method_name}({args});
"#,
            native_lib_name = self.config.native_lib_name,
            return_type = foreign_method_signature
                .output
                .type_info
                .dotnet_intermediate_type,
            method_name = full_method_name,
            args = pinvoke_args_str,
        )
        .with_note("Write to memory failed")?;

        Ok(())
    }

    fn write_dotnet_wrapper_function(
        &mut self,
        class: &ForeignerClassInfo,
        // method: &ForeignerMethod,
        foreign_method_signature: &DotNetForeignMethodSignature,
    ) -> Result<()> {
        let mut name_generator = NameGenerator::new();
        let maybe_static_str = if foreign_method_signature.variant == MethodVariant::StaticMethod {
            "static"
        } else {
            ""
        };
        let is_constructor = foreign_method_signature.variant == MethodVariant::Constructor;
        let full_method_name = format!("{}_{}", class.name, foreign_method_signature.name);
        let method_name = if is_constructor {
            ""
        } else {
            &foreign_method_signature.name
        };
        let args_to_skip = if let MethodVariant::Method(_) = foreign_method_signature.variant {
            1
        } else {
            0
        };
        let dotnet_args_str = foreign_method_signature
            .input
            .iter()
            .skip(args_to_skip)
            .map(|arg| {
                format!(
                    "{} {}",
                    arg.type_info.dotnet_type,
                    NameGenerator::first_variant(arg.arg_name.dotnet_variable_name())
                )
            })
            .join(", ");

        let this_input_conversion =
            if let MethodVariant::Method(_) = foreign_method_signature.variant {
                "var __this_0 = this.nativePtr;\n"
            } else {
                ""
            };

        let dotnet_input_conversion = this_input_conversion.to_owned()
            + &foreign_method_signature
                .input
                .iter()
                .skip(args_to_skip)
                .map(|arg| arg.dotnet_conversion_code(&mut name_generator))
                .join("\n            ");

        let returns_something =
            foreign_method_signature.output.type_info.dotnet_type != "void" && !is_constructor;
        let maybe_return_bind = if returns_something {
            "var __ret_0 = "
        } else if is_constructor {
            "this.nativePtr = "
        } else {
            ""
        };
        let maybe_dotnet_output_conversion = if returns_something {
            foreign_method_signature
                .output
                .dotnet_conversion_code(&mut name_generator)
        } else {
            String::new()
        };
        let maybe_return = if returns_something {
            format!("return {};", name_generator.last_variant("__ret"))
        } else {
            String::new()
        };

        let finalizers = foreign_method_signature
            .input
            .iter()
            .filter(|arg| arg.has_finalizer())
            .map(|arg| arg.dotnet_finalizer(&mut name_generator))
            .join("\n            ");

        let pinvoke_call_args = foreign_method_signature
            .input
            .iter()
            .map(|arg| name_generator.last_variant(arg.arg_name.dotnet_variable_name()))
            .join(", ");
        write!(
            self.cs_file,
            r#"
        public {maybe_static} {dotnet_return_type} {method_name}({dotnet_args}) {{
            {dotnet_input_conversion}
            {maybe_return_bind}{full_method_name}({pinvoke_call_args});
            {maybe_dotnet_output_conversion}
            {finalizers}
            {maybe_return}
        }}
"#,
            maybe_static = maybe_static_str,
            dotnet_return_type = foreign_method_signature.output.type_info.dotnet_type,
            method_name = method_name,
            dotnet_args = dotnet_args_str,
            dotnet_input_conversion = dotnet_input_conversion,
            maybe_return_bind = maybe_return_bind,
            full_method_name = full_method_name,
            pinvoke_call_args = pinvoke_call_args,
            maybe_dotnet_output_conversion = maybe_dotnet_output_conversion,
            finalizers = finalizers,
            maybe_return = maybe_return,
        )
        .with_note("Write to memory failed")?;

        Ok(())
    }

    fn finish(&mut self) -> Result<()> {
        for (_, cs_code) in self.additional_cs_code_for_types.drain() {
            write!(self.cs_file, "{}", cs_code)?;
        }
        writeln!(self.cs_file, "}} // namespace",)?;
        Ok(())
    }
}

impl LanguageGenerator for DotNetConfig {
    fn expand_items(
        &self,
        conv_map: &mut TypeMap,
        _target_pointer_width: usize,
        _code: &[SourceCode],
        items: Vec<ItemToExpand>,
        _remove_not_generated_files: bool,
    ) -> Result<Vec<TokenStream>> {
        DotNetGenerator::new(&self, conv_map)?.generate(items)
    }
}
