mod swig_foreign_types_map {}

foreign_typemap!(
    (r_type) ::std::os::raw::c_char;
    (f_type) "char";
);

foreign_typemap!(
    (r_type) ::std::os::raw::c_int;
    (f_type) "int";
);

foreign_typemap!(
    (r_type) f32;
    (f_type) "float";
);

foreign_typemap!(
    (r_type) f64;
    (f_type) "double";
);

foreign_typemap!(
    (r_type) ();
    (f_type) "void";
);

foreign_typemap!(
    (r_type) i8;
    (f_type, req_modules = ["<stdint.h>"]) "int8_t";
);

foreign_typemap!(
    (r_type) u8;
    (f_type, req_modules = ["<stdint.h>"]) "uint8_t";
);

foreign_typemap!(
    (r_type) i16;
    (f_type, req_modules = ["<stdint.h>"]) "int16_t";
);

foreign_typemap!(
    (r_type) u16;
    (f_type, req_modules = ["<stdint.h>"]) "uint16_t";
);

foreign_typemap!(
    (r_type) i32;
    (f_type, req_modules = ["<stdint.h>"]) "int32_t";
);

foreign_typemap!(
    (r_type) u32;
    (f_type, req_modules = ["<stdint.h>"]) "uint32_t";
);

foreign_typemap!(
    (r_type) i64;
    (f_type, req_modules = ["<stdint.h>"]) "int64_t";
);

foreign_typemap!(
    (r_type) u64;
    (f_type, req_modules = ["<stdint.h>"]) "uint64_t";
);

foreign_typemap!(
    (r_type) usize;
    (f_type, req_modules = ["<stdint.h>"]) "uintptr_t";
);

foreign_typemap!(
    (r_type) isize;
    (f_type, req_modules = ["<stdint.h>"]) "intptr_t";
);

#[allow(unused_macros)]
macro_rules! swig_c_str {
    ($lit:expr) => {
        concat!($lit, "\0").as_ptr() as *const ::std::os::raw::c_char
    };
}

#[allow(dead_code)]
pub trait SwigForeignClass {
    fn c_class_name() -> *const ::std::os::raw::c_char;
    fn box_object(x: Self) -> *mut ::std::os::raw::c_void;
    fn unbox_object(p: *mut ::std::os::raw::c_void) -> Self;
}

#[allow(dead_code)]
pub trait SwigForeignEnum {
    fn as_u32(&self) -> u32;
    fn from_u32(_: u32) -> Self;
}

#[allow(dead_code)]
#[swig_code = "let mut {to_var}: {to_var_type} = {from_var}.swig_into();"]
trait SwigInto<T> {
    fn swig_into(self) -> T;
}

#[allow(dead_code)]
#[swig_code = "let mut {to_var}: {to_var_type} = <{to_var_type}>::swig_from({from_var});"]
trait SwigFrom<T> {
    fn swig_from(_: T) -> Self;
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRustStrView {
    data: *const ::std::os::raw::c_char,
    len: usize,
}

#[allow(dead_code)]
impl CRustStrView {
    fn from_str(s: &str) -> CRustStrView {
        CRustStrView {
            data: s.as_ptr() as *const ::std::os::raw::c_char,
            len: s.len(),
        }
    }
}

foreign_typemap!(
    ($p:r_type) <T> Arc<Mutex<T>> => &Mutex<T> {
        $out = & $p;
    };
);

foreign_typemap!(
    ($p:r_type) <T> &Mutex<T> => MutexGuard<T> {
        $out = $p.lock().unwrap();
    };
);

foreign_typemap!(
    ($p:r_type) <T> MutexGuard<T> => &T {
        $out = & $p;
    };
);

foreign_typemap!(
    ($p:r_type) <T> MutexGuard<T> => &mut T {
        $out = &mut $p;
    };
);

foreign_typemap!(
    ($p:r_type) <T> Rc<T> => &T {
        $out = & $p;
    };
);

foreign_typemap!(
    ($p:r_type) <T> &Rc<T> => &T {
        $out = & $p;
    };
);

foreign_typemap!(
    ($p:r_type) <T> &RefCell<T> => Ref<T> {
        $out = $p.borrow();
    };
);

foreign_typemap!(
    ($p:r_type) <T> &RefCell<T> => RefMut<T> {
        $out = $p.borrow_mut();
    };
);

foreign_typemap!(
    ($p:r_type) <T> Ref<T> => &T {
        $out = & $p;
    };
);

foreign_typemap!(
    ($p:r_type) <T> RefMut<T> => &mut T {
        $out = &mut $p;
    };
);

// &str -> &Path
impl<'a> SwigInto<&'a Path> for &'a str {
    fn swig_into(self) -> &'a Path {
        Path::new(self)
    }
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CRustString {
    data: *const ::std::os::raw::c_char,
    len: usize,
    capacity: usize,
}

#[allow(dead_code)]
impl CRustString {
    pub fn from_string(s: String) -> CRustString {
        let data = s.as_ptr() as *const ::std::os::raw::c_char;
        let len = s.len();
        let capacity = s.capacity();
        ::std::mem::forget(s);
        CRustString {
            data,
            len,
            capacity,
        }
    }
}

impl<'a> SwigInto<String> for &'a str {
    fn swig_into(self) -> String {
        self.into()
    }
}

impl<T: SwigForeignEnum> SwigFrom<T> for u32 {
    fn swig_from(x: T) -> u32 {
        x.as_u32()
    }
}

impl<T: SwigForeignEnum> SwigFrom<u32> for T {
    fn swig_from(x: u32) -> T {
        T::from_u32(x)
    }
}

foreign_typemap!(
    (r_type) * mut ::std::os::raw::c_void;
    (f_type) "void *";
);

foreign_typemap!(
    (r_type) * const ::std::os::raw::c_void;
    (f_type) "const void *";
);

foreign_typemap!(
    (r_type) <T: SwigTypeIsReprC> *const T;
    (f_type) "const swig_f_type!(T) *";
);

foreign_typemap!(
    (r_type) <T: SwigTypeIsReprC> *mut T;
    (f_type) "swig_f_type!(T) *";
);

foreign_typemap!(
    generic_alias!(CFnOneArgPtr = swig_concat_idents!(c_fn_, swig_i_type!(T), _t));
    foreign_code!(
        module = "CFnOneArgPtr!().h";
        r##"
typedef void (*CFnOneArgPtr!())(swig_f_type!(T));
"##
    );
    (r_type) <T: SwigTypeIsReprC> Option<extern "C" fn(T)>;
    (f_type, req_modules = ["\"CFnOneArgPtr!().h\""]) "CFnOneArgPtr!()";
);

foreign_typemap!(
    generic_alias!(CFnTwoArgsPtr = swig_concat_idents!(c_fn_, swig_i_type!(T1), swig_i_type!(T2), _t));
    foreign_code!(
        module = "CFnTwoArgsPtr!().h";
        r##"
typedef void (*CFnTwoArgsPtr!())(swig_f_type!(T1), swig_f_type!(T2));
"##
    );
    (r_type) <T1: SwigTypeIsReprC, T2: SwigTypeIsReprC> extern "C" fn(T1, T2);
    (f_type, req_modules = ["\"CFnTwoArgsPtr!().h\""]) "CFnTwoArgsPtr!()";
);

foreign_typemap!(
   generic_alias!(CRustPair = swig_concat_idents!(CRustPair, swig_i_type!(T1), swig_i_type!(T2)));
   generic_alias!(CRustPairModule = swig_concat_idents!(rust_tuple, swig_i_type!(T1), swig_i_type!(T2)));
   define_c_type!(
        module = "CRustPairModule!().h";
       #[repr(C)]
       #[derive(Clone, Copy)]
       pub struct CRustPair!() {
           first: swig_i_type!(T1),
           second: swig_i_type!(T2),
       }
   );
   ($p:r_type) <T1, T2> (T1, T2) => CRustPair!() {
       swig_from_rust_to_i_type!(T1, $p.0, p0)
       swig_from_rust_to_i_type!(T2, $p.1, p1)
       $out = CRustPair!() {
           first: p0,
           second: p1,
       };
   };
   ($p:r_type) <T1, T2> (T1, T2) <= CRustPair!() {
       swig_from_i_type_to_rust!(T1, $p.first, p0)
       swig_from_i_type_to_rust!(T2, $p.second, p1)
       $out = (p0, p1);
   };
   ($p:f_type, req_modules = ["\"CRustPairModule!().h\"", "<utility>"]) => "std::pair<swig_f_type!(T1), swig_f_type!(T2)>"
           "std::make_pair(swig_foreign_from_i_type!(T1, $p.first), swig_foreign_from_i_type!(T2, $p.second))";
   ($p:f_type, req_modules = ["\"CRustPairModule!().h\"", "<utility>"]) <= "std::pair<swig_f_type!(T1), swig_f_type!(T2)>"
       "swig_f_type!(CRustPair!()) { swig_foreign_to_i_type!(T1, $p.first), swig_foreign_to_i_type!(T2, $p.second) }";
);

foreign_typemap!(
   generic_alias!(CRustTuple3 = swig_concat_idents!(CRustTuple3, swig_i_type!(T1), swig_i_type!(T2), swig_i_type!(T3)));
   generic_alias!(CRustTuple3Module = swig_concat_idents!(rust_tuple, swig_i_type!(T1), swig_i_type!(T2), swig_i_type!(T3)));
   define_c_type!(
        module = "CRustTuple3Module!().h";
       #[repr(C)]
       #[derive(Clone, Copy)]
       pub struct CRustTuple3!() {
           first: swig_i_type!(T1),
           second: swig_i_type!(T2),
           third: swig_i_type!(T3),
       }
   );
   ($p:r_type) <T1, T2, T3> (T1, T2, T3) => CRustTuple3!() {
       swig_from_rust_to_i_type!(T1, $p.0, p0)
       swig_from_rust_to_i_type!(T2, $p.1, p1)
       swig_from_rust_to_i_type!(T3, $p.2, p2)
       $out = CRustTuple3!() {
           first: p0,
           second: p1,
           third: p2,
       };
   };
   ($p:r_type) <T1, T2, T3> (T1, T2, T3) <= CRustTuple3!() {
       swig_from_i_type_to_rust!(T1, $p.first, p0)
       swig_from_i_type_to_rust!(T2, $p.second, p1)
       swig_from_i_type_to_rust!(T3, $p.third, p2)
       $out = (p0, p1, p2);
   };
   ($p:f_type, req_modules = ["\"CRustTuple3Module!().h\"", "<tuple>"]) => "std::tuple<swig_f_type!(T1), swig_f_type!(T2), swig_f_type!(T3)>"
       r#"std::make_tuple(swig_foreign_from_i_type!(T1, $p.first),
                           swig_foreign_from_i_type!(T2, $p.second),
                           swig_foreign_from_i_type!(T3, $p.third))"#;
   ($p:f_type, req_modules = ["\"CRustTuple3Module!().h\"", "<tuple>"]) <= "std::tuple<swig_f_type!(T1), swig_f_type!(T2), swig_f_type!(T3)>"
       r#"swig_f_type!(CRustTuple3!()) {
               swig_foreign_to_i_type!(T1, $p.first),
               swig_foreign_to_i_type!(T2, $p.second),
               swig_foreign_to_i_type!(T3, $p.third),
        }"#;
);

foreign_typemap!(
    ($pin:r_type) bool => ::std::os::raw::c_char {
        $out = if $pin  { 1 } else { 0 };
    };
    ($pin:f_type) => "bool" "($pin != 0)";
    ($pin:r_type) bool <= ::std::os::raw::c_char {
        $out = $pin != 0;
    };
    ($pin:f_type) <= "bool" "$pin ? 1 : 0";
);

foreign_typemap!(
    define_c_type!(module = "rust_str.h";
        #[repr(C)]
        #[derive(Clone, Copy)]
        pub struct CRustStrView {
            data: *const ::std::os::raw::c_char,
            len: usize,
        }
    );
    ($p:r_type) &str => CRustStrView {
        $out = CRustStrView::from_str($p);
    };
    ($p:r_type) &str <= CRustStrView {
        $out = unsafe {
            let slice: &[u8] = ::std::slice::from_raw_parts($p.data as *const u8, $p.len);
            ::std::str::from_utf8_unchecked(slice)
        };
    };

    ($p:f_type, option = "CppStrView::Boost", req_modules = ["\"rust_str.h\"", "<boost/utility/string_view.hpp>"]) => "boost::string_view"
        "boost::string_view{ $p.data, $p.len }";
    ($p:f_type, option = "CppStrView::Boost", req_modules = ["\"rust_str.h\"", "<boost/utility/string_view.hpp>"]) <= "boost::string_view"
        "CRustStrView{ $p.data(), $p.size() }";

    ($p:f_type, option = "CppStrView::Std17", req_modules = ["\"rust_str.h\"", "<string_view>"]) => "std::string_view"
        "std::string_view{ $p.data, $p.len }";
    ($p:f_type, option = "CppStrView::Std17", req_modules = ["\"rust_str.h\"", "<string_view>"]) <= "std::string_view"
        "CRustStrView{ $p.data(), $p.size() }";
);

foreign_typemap!(
    define_c_type!(module = "rust_str.h";
        #[repr(C)]
        #[derive(Clone, Copy)]
        struct CRustString {
            data: *const ::std::os::raw::c_char,
            len: usize,
            capacity: usize,
        }

        #[no_mangle]
        pub extern "C" fn crust_string_free(x: CRustString) {
            let s = unsafe { String::from_raw_parts(x.data as *mut u8, x.len, x.capacity) };
            drop(s);
        }

        #[no_mangle]
        pub extern "C" fn crust_string_clone(x: CRustString) -> CRustString {
            let s = unsafe { String::from_raw_parts(x.data as *mut u8, x.len, x.capacity) };
            let ret = CRustString::from_string(s.clone());
            ::std::mem::forget(s);
            ret
        }
    );
    foreign_code!(module = "rust_str.h";
                    r##"
#ifdef __cplusplus

#include <string>
"##
    );
    foreign_code!(module = "rust_str.h";
                    option = "CppStrView::Std17";
                    r##"
#include <string_view>
"##);
    foreign_code!(module = "rust_str.h";
                    option = "CppStrView::Boost";
                    r##"
#include <boost/utility/string_view.hpp>
"##);
    foreign_code!(module = "rust_str.h";
                    r##"
namespace $RUST_SWIG_USER_NAMESPACE {
class RustString final : private CRustString {
public:
    explicit RustString(const CRustString &o) noexcept
    {
        data = o.data;
        len = o.len;
        capacity = o.capacity;
    }
    RustString() noexcept { reset(*this); }
    RustString(const RustString &o) noexcept
        : RustString(crust_string_clone(o))
    {
    }
    RustString &operator=(const RustString &o) noexcept
    {
        if (this != &o) {
            free_mem();
            auto copy = crust_string_clone(o);
            data = copy.data;
            len = copy.len;
            capacity = copy.capacity;
        }
        return *this;
    }
    RustString(RustString &&o) noexcept
    {
        data = o.data;
        len = o.len;
        capacity = o.capacity;

        reset(o);
    }
    RustString &operator=(RustString &&o) noexcept
    {
        free_mem();
        data = o.data;
        len = o.len;
        capacity = o.capacity;

        reset(o);
        return *this;
    }
    ~RustString() noexcept { free_mem(); }
    std::string to_std_string() const { return std::string(data, len); }
    size_t size() const noexcept { return this->len; }
    bool empty() const noexcept { return this->len == 0; }
"##);
    foreign_code!(module = "rust_str.h";
                    option = "CppStrView::Std17";
                    r##"
    std::string_view to_string_view() const { return std::string_view(data, len); }
"##);
    foreign_code!(module = "rust_str.h";
                    option = "CppStrView::Boost";
                    r#"
    boost::string_view to_boost_string_view() const { return boost::string_view{ data, len }; }
"#);
    foreign_code!(module = "rust_str.h";
                    r##"
private:
    void free_mem() noexcept
    {
        if (data != nullptr) {
            crust_string_free(*this);
            reset(*this);
        }
    }
    static void reset(RustString &o) noexcept
    {
        o.data = nullptr;
        o.len = 0;
        o.capacity = 0;
    }
};
} // namespace $RUST_SWIG_USER_NAMESPACE
#endif // __cplusplus
"##
                    );
    ($pin:r_type) String => CRustString {
        $out = CRustString::from_string($pin);
    };
    ($pin:f_type, req_modules = ["\"rust_str.h\""]) => "RustString" "RustString{$pin}";
);

foreign_typemap!(
    generic_alias!(CRustClassOpt = swig_concat_idents!(CRustClassOpt, swig_f_type!(T)));
    define_c_type!(
         module = "rust_option.h";
         #[repr(C)]
         #[derive(Clone, Copy)]
         pub struct CRustClassOpt!() {
             p: *const ::std::os::raw::c_void,
         }
    );
    ($p:r_type) <T: SwigForeignClass> Option<&T> <= CRustClassOpt!() {
        $out = if !$p.p.is_null() {
            swig_from_i_type_to_rust!(&T, $p.p, obj)
            Some(obj)
        } else {
            None
        };
    };
    ($p:f_type, unique_prefix = "/*opt ref*/", req_modules = ["\"rust_option.h\""]) <= "/*opt ref*/const swig_f_type!(T) *" r#"
        $out = CRustClassOpt!() { ($p != nullptr) ? static_cast<swig_f_type!(T)Opaque *>(* $p) : nullptr };
"#;
);

foreign_typemap!(
    generic_alias!(CRustClassOptMut = swig_concat_idents!(CRustClassOptMut, swig_f_type!(T)));
    define_c_type!(
         module = "rust_option.h";
         #[repr(C)]
         #[derive(Clone, Copy)]
         pub struct CRustClassOptMut!() {
             p: *mut ::std::os::raw::c_void,
         }
    );
    ($p:r_type) <T: SwigForeignClass> Option<&mut T> <= CRustClassOptMut!() {
        $out = if !$p.p.is_null() {
            swig_from_i_type_to_rust!(&mut T, $p.p, obj)
            Some(obj)
        } else {
            None
        };
    };
    ($p:f_type, unique_prefix = "/*opt mut ref*/", req_modules = ["\"rust_option.h\""]) <= "/*opt mut ref*/swig_f_type!(T) *" r#"
        $out = CRustClassOptMut!() { ($p != nullptr) ? static_cast<swig_f_type!(T)Opaque *>(* $p) : nullptr };
"#;
);

foreign_typemap!(
    generic_alias!(CRustOpt = swig_concat_idents!(CRustOption, swig_i_type!(T)));
    generic_alias!(CRustOptUnion = swig_concat_idents!(CRustOptionUnion, swig_i_type!(T)));
    define_c_type!(
         module = "rust_option.h";
         #[repr(C)]
         #[derive(Clone, Copy)]
         pub union CRustOptUnion!() {
             data: swig_i_type!(T),
             uninit: u8,
         }

         #[repr(C)]
         #[derive(Clone, Copy)]
         pub struct CRustOpt!() {
             val: CRustOptUnion!(),
             is_some: u8,
         }
     );
    ($p:r_type) <T> Option<T> => CRustOpt!() {
        $out = match $p {
            Some(mut x) => {
                swig_from_rust_to_i_type!(T, x, data)
                CRustOpt!() {
                    val: CRustOptUnion!() { data },
                    is_some: 1,
                }
            }
            None => CRustOpt!() {
                    val: CRustOptUnion!() { uninit: 0 },
                    is_some: 0,
            },
        };
    };
    ($p:r_type) <T> Option<T> <= CRustOpt!() {
        $out = if $p.is_some != 0 {
            swig_from_i_type_to_rust!(T, unsafe { $p.val.data }, ret)
            Some(ret)
        } else {
            None
        };
    };

    ($p:f_type, option = "CppOptional::Boost", req_modules = ["\"rust_option.h\"", "<boost/optional.hpp>"]) => "boost::optional<swig_f_type!(T)>"
        "($p.is_some != 0) ? boost::optional<swig_f_type!(T)>(swig_foreign_from_i_type!(T, $p.val.data)) : boost::optional<swig_f_type!(T)>()";
    ($p:f_type, option = "CppOptional::Boost", req_modules = ["\"rust_option.h\"", "<boost/optional.hpp>"]) <= "boost::optional<swig_f_type!(T)>"
        r#"        $out;
        if (!!$p) {
            $out.val.data = swig_foreign_to_i_type!(T, (*$p));
            $out.is_some = 1;
        } else {
            $out.is_some = 0;
        }"#;

    ($p:f_type, option = "CppOptional::Std17", req_modules = ["\"rust_option.h\"", "<optional>"]) => "std::optional<swig_f_type!(T)>"
        "($p.is_some != 0) ? std::optional<swig_f_type!(T)>(swig_foreign_from_i_type!(T, $p.val.data)) : std::optional<swig_f_type!(T)>()";
    ($p:f_type, option = "CppOptional::Std17", req_modules = ["\"rust_option.h\"", "<optional>"]) <= "std::optional<swig_f_type!(T)>"
        r#"        $out;
        if (!!$p) {
            $out.val.data = swig_foreign_to_i_type!(T, (*$p));
            $out.is_some = 1;
        } else {
            $out.is_some = 0;
        }"#;
);

#[allow(dead_code)]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRustObjectSlice {
    data: *const ::std::os::raw::c_void,
    len: usize,
    step: usize,
}

foreign_typemap!(
    define_c_type!(
        module = "rust_slice.h";
        #[repr(C)]
        #[derive(Clone, Copy)]
        pub struct CRustObjectSlice {
            data: *const ::std::os::raw::c_void,
            len: usize,
            step: usize,
        });
    foreign_code!(module = "rust_slice.h";
                    r##"
#ifdef __cplusplus
#include "rust_foreign_slice_impl.hpp"

namespace $RUST_SWIG_USER_NAMESPACE {
template<typename T>
using RustForeignSliceConst = RustForeignSlice<T, CRustObjectSlice>;
}
#endif
"##);
    (r_type) CRustObjectSlice;
    (f_type) "CRustObjectSlice";
);

#[allow(dead_code)]
#[repr(C)]
pub struct CRustObjectMutSlice {
    data: *mut ::std::os::raw::c_void,
    len: usize,
    step: usize,
}

foreign_typemap!(
    define_c_type!(
        module = "rust_slice_mut.h";
        #[repr(C)]
        #[derive(Clone, Copy)]
        pub struct CRustObjectMutSlice {
            data: *mut ::std::os::raw::c_void,
            len: usize,
            step: usize,
        });
    foreign_code!(module = "rust_slice_mut.h";
                    r##"
#ifdef __cplusplus
#include "rust_foreign_slice_impl.hpp"

namespace $RUST_SWIG_USER_NAMESPACE {
template<typename T>
using RustForeignSliceMut = RustForeignSlice<T, CRustObjectMutSlice>;
}
#endif
"##);
    (r_type) CRustObjectMutSlice;
    (f_type) "CRustObjectMutSlice";
);

foreign_typemap!(
    ($p:r_type) <T: SwigForeignClass> &[T] => CRustObjectSlice {
        $out = CRustObjectSlice {
            data: $p.as_ptr() as *const ::std::os::raw::c_void,
            len: $p.len(),
            step: ::std::mem::size_of::<swig_subst_type!(T)>(),
        };
    };
    ($p:r_type) <T: SwigForeignClass> &[T] <= CRustObjectSlice {
        $out = unsafe { ::std::slice::from_raw_parts($p.data as *const swig_subst_type!(T), $p.len) };
    };
    ($p:f_type, req_modules = ["\"rust_slice.h\""]) => "RustForeignSliceConst<swig_f_type!(&T)>"
        "RustForeignSliceConst<swig_f_type!(&T)>{$p}";
    ($p:f_type, req_modules = ["\"rust_slice.h\""]) <= "RustForeignSliceConst<swig_f_type!(&T, output)>"
        "$p";
);

foreign_typemap!(
    ($p:r_type) <T: SwigForeignClass> &mut [T] => CRustObjectMutSlice {
        $out = CRustObjectMutSlice {
            data: $p.as_ptr() as *const ::std::os::raw::c_void,
            len: $p.len(),
            step: ::std::mem::size_of::<swig_subst_type!(T)>(),
        };
    };
    ($p:r_type) <T: SwigForeignClass> &mut [T] <= CRustObjectMutSlice {
        $out = unsafe { ::std::slice::from_raw_parts_mut($p.data as *mut swig_subst_type!(T), $p.len) };
    };
    ($p:f_type, req_modules = ["\"rust_slice_mut.h\""]) => "RustForeignSliceMut<swig_f_type!(&T)>"
        "RustForeignSliceMut<swig_f_type!(&T)>{$p}";
    ($p:f_type, req_modules = ["\"rust_slice_mut.h\""]) <= "RustForeignSliceMut<swig_f_type!(&T, output)>"
        "$p";
);

foreign_typemap!(
    generic_alias!(CRustSlice = swig_concat_idents!(CRustSlice, swig_i_type!(T)));
    define_c_type!(
        module = "CRustSlice!().h";
        #[repr(C)]
        #[derive(Clone, Copy)]
        pub struct CRustSlice!() {
            data: *const swig_i_type!(T),
            len: usize,
        }
    );
    foreign_code!(module = "CRustSlice!().h";
                    r##"
#ifdef __cplusplus
#include "rust_slice_tmpl.hpp"
#endif
"##);
    ($p:r_type) <T: SwigTypeIsReprC> &[T] => CRustSlice!() {
        $out =  CRustSlice!() {
            data: $p.as_ptr(),
            len: $p.len(),
        };
    };
    ($p:r_type) <T: SwigTypeIsReprC> &[T] <= CRustSlice!() {
        assert!($p.len == 0 || !$p.data.is_null());
        $out = unsafe { ::std::slice::from_raw_parts($p.data, $p.len) };
    };
    ($p:f_type, req_modules = ["\"CRustSlice!().h\""]) => "RustSlice<const swig_f_type!(T)>"
        "RustSlice<const swig_f_type!(T)>{$p.data, $p.len}";
    ($p:f_type, req_modules = ["\"CRustSlice!().h\""]) <= "RustSlice<const swig_f_type!(T)>"
        "$p.as_c<swig_f_type!(CRustSlice!())>()";
);

foreign_typemap!(
    generic_alias!(CRustSliceMut = swig_concat_idents!(CRustSliceMut, swig_i_type!(T)));
    define_c_type!(
        module = "CRustSliceMut!().h";
        #[repr(C)]
        #[derive(Clone, Copy)]
        pub struct CRustSliceMut!() {
            data: *mut swig_i_type!(T),
            len: usize,
        }
    );
    foreign_code!(module = "CRustSliceMut!().h";
                    r##"
#ifdef __cplusplus
#include "rust_slice_tmpl.hpp"
#endif
"##);
    ($p:r_type) <T: SwigTypeIsReprC> &mut [T] => CRustSliceMut!() {
        $out =  CRustSliceMut!() {
            data: $p.as_mut_ptr(),
            len: $p.len(),
        };
    };
    ($p:r_type) <T: SwigTypeIsReprC> &mut [T] <= CRustSliceMut!() {
        assert!($p.len == 0 || !$p.data.is_null());
        $out = unsafe { ::std::slice::from_raw_parts_mut($p.data, $p.len) };
    };
    ($p:f_type, req_modules = ["\"CRustSliceMut!().h\""]) => "RustSlice<swig_f_type!(T)>"
        "RustSlice<swig_f_type!(T)>{$p.data, $p.len}";
    ($p:f_type, req_modules = ["\"CRustSliceMut!().h\""]) <= "RustSlice<swig_f_type!(T)>"
        "$p.as_c<swig_f_type!(CRustSliceMut!())>()";
);

foreign_typemap!(
    generic_alias!(CRustVec = swig_concat_idents!(CRustVec, swig_i_type!(T)));
    generic_alias!(CRustVecModule = swig_concat_idents!(rust_vec_, swig_i_type!(T)));
    generic_alias!(CRustVecFree = swig_concat_idents!(CRustVec, swig_i_type!(T), _free));
    generic_alias!(CppRustVec = swig_concat_idents!(RustVec, swig_i_type!(T)));
    define_c_type!(
        module = "CRustVecModule!().h";
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct CRustVec!() {
            data: *const swig_subst_type!(T),
            len: usize,
            capacity: usize,
        }

        #[no_mangle]
        pub extern "C" fn CRustVecFree!()(v: CRustVec!()) {
            let v = unsafe { Vec::from_raw_parts(v.data as *mut swig_subst_type!(T), v.len, v.capacity) };
            drop(v);
        }
    );
    foreign_code!(module = "CRustVecModule!().h";
                    r##"
#ifdef __cplusplus

#include "rust_vec_impl.hpp"

namespace $RUST_SWIG_USER_NAMESPACE {
using CppRustVec!() = RustVec<CRustVec!(), CRustVecFree!()>;
}

#endif
"##);
    ($p:r_type) <T: SwigTypeIsReprC> Vec<T> => CRustVec!() {
        let p = $p.as_mut_ptr();
        let len = $p.len();
        let cap = $p.capacity();
        ::std::mem::forget($p);
        $out = CRustVec!() {
            data: p,
            len: len,
            capacity: cap,
        };
    };
    ($p:f_type, req_modules = ["\"CRustVecModule!().h\""]) => "CppRustVec!()"
        "CppRustVec!(){$p}";
);

#[allow(dead_code)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CRustForeignVec {
    data: *const ::std::os::raw::c_void,
    len: usize,
    capacity: usize,
    step: usize,
}

#[allow(dead_code)]
impl CRustForeignVec {
    pub fn from_vec<T: SwigForeignClass>(mut v: Vec<T>) -> CRustForeignVec {
        let data = v.as_mut_ptr() as *const ::std::os::raw::c_void;
        let len = v.len();
        let capacity = v.capacity();
        ::std::mem::forget(v);
        CRustForeignVec {
            data,
            len,
            capacity,
            step: ::std::mem::size_of::<T>(),
        }
    }
}

#[allow(dead_code)]
#[inline]
fn push_foreign_class_to_vec<T: SwigForeignClass>(
    vec: *mut CRustForeignVec,
    elem: *mut ::std::os::raw::c_void,
) {
    assert!(!vec.is_null());
    let vec: &mut CRustForeignVec = unsafe { &mut *vec };
    assert!(vec.len == 0 || ::std::mem::size_of::<T>() == vec.step);
    vec.step = ::std::mem::size_of::<T>();
    let mut v = unsafe { Vec::from_raw_parts(vec.data as *mut T, vec.len, vec.capacity) };
    v.push(T::unbox_object(elem));
    vec.data = v.as_mut_ptr() as *const ::std::os::raw::c_void;
    vec.len = v.len();
    vec.capacity = v.capacity();
    ::std::mem::forget(v);
}

#[allow(dead_code)]
#[inline]
fn remove_foreign_class_from_vec<T: SwigForeignClass>(
    vec: *mut CRustForeignVec,
    index: usize,
) -> *mut ::std::os::raw::c_void {
    assert!(!vec.is_null());
    let vec: &mut CRustForeignVec = unsafe { &mut *vec };
    assert_eq!(::std::mem::size_of::<T>(), vec.step);
    let mut v = unsafe { Vec::from_raw_parts(vec.data as *mut T, vec.len, vec.capacity) };
    let elem: T = v.remove(index);
    vec.data = v.as_mut_ptr() as *const ::std::os::raw::c_void;
    vec.len = v.len();
    vec.capacity = v.capacity();
    ::std::mem::forget(v);
    T::box_object(elem)
}

#[allow(dead_code)]
#[inline]
fn drop_foreign_class_vec<T: SwigForeignClass>(v: CRustForeignVec) {
    assert_eq!(::std::mem::size_of::<T>(), v.step);
    let v = unsafe { Vec::from_raw_parts(v.data as *mut T, v.len, v.capacity) };
    drop(v);
}

foreign_typemap!(
    define_c_type!(
        module = "rust_vec.h";
        #[repr(C)]
        #[derive(Clone, Copy)]
        pub struct CRustForeignVec {
            data: *const ::std::os::raw::c_void,
            len: usize,
            capacity: usize,
            step: usize,
        });
    (r_type) CRustForeignVec;
    (f_type) "CRustForeignVec";
);

foreign_typemap!(
    generic_alias!(CForeignVecModule = swig_concat_idents!(RustForeignVec, swig_f_type!(T)));
    generic_alias!(CForeignVecFree = swig_concat_idents!(RustForeignVec, swig_f_type!(T), _free));
    generic_alias!(CForeignVecPush = swig_concat_idents!(RustForeignVec, swig_f_type!(T), _push));
    generic_alias!(CForeignVecRemove = swig_concat_idents!(RustForeignVec, swig_f_type!(T), _remove));

    define_c_type!(
        module = "CForeignVecModule!().h";
        #[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
        #[no_mangle]
        pub extern "C" fn CForeignVecFree!()(v: CRustForeignVec) {
            drop_foreign_class_vec::<swig_subst_type!(T)>(v);
        }

        #[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
        #[no_mangle]
        pub extern "C" fn CForeignVecPush!()(v: *mut CRustForeignVec, e: *mut ::std::os::raw::c_void) {
            push_foreign_class_to_vec::<swig_subst_type!(T)>(v, e);
        }

        #[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
        #[no_mangle]
        pub extern "C" fn CForeignVecRemove!()(v: *mut CRustForeignVec, idx: usize) -> *mut ::std::os::raw::c_void {
            remove_foreign_class_from_vec::<swig_subst_type!(T)>(v, idx)
        }
    );

    foreign_code!(module = "CForeignVecModule!().h";
                    r##"
#ifdef __cplusplus

#include "rust_foreign_vec_impl.hpp"

namespace $RUST_SWIG_USER_NAMESPACE {
using CForeignVecModule!() = RustForeignVec<swig_f_type!(&T, output), CRustForeignVec, CForeignVecFree!(), CForeignVecPush!(), CForeignVecRemove!()>;
}
#endif
"##);

    ($p:r_type) <T: SwigForeignClass> Vec<T> => CRustForeignVec {
        $out = CRustForeignVec::from_vec($p);
    };
    ($p:r_type) <T: SwigForeignClass> Vec<T> <= CRustForeignVec {
        $out = unsafe { Vec::from_raw_parts($p.data as *mut swig_subst_type!(T), $p.len, $p.capacity) };
    };
    ($p:f_type, req_modules = ["\"CForeignVecModule!().h\""]) => "CForeignVecModule!()"
        "CForeignVecModule!(){$p}";
    ($p:f_type, req_modules = ["\"CForeignVecModule!().h\""]) <= "CForeignVecModule!()"
        "$p.release()";
);

// order is important!!!
// we map () to void, but C++ can not handle std::variant<void,..>
foreign_typemap!(
   generic_alias!(CRustRes = swig_concat_idents!(CRustVoidOkResult, swig_i_type!(T)));
   generic_alias!(CRustResUnion = swig_concat_idents!(CRustVoidOkResultUnion, swig_i_type!(T)));
   generic_alias!(CRustResModule = swig_concat_idents!(rust_void_ok_result, swig_i_type!(T)));
   define_c_type!(
       module = "CRustResModule!().h";
       #[repr(C)]
       #[derive(Clone, Copy)]
       pub union CRustResUnion!() {
           ok: u8,
           err: swig_i_type!(T),
       }

       #[repr(C)]
       #[derive(Clone, Copy)]
       pub struct CRustRes!() {
           data: CRustResUnion!(),
           is_ok: u8,
       }
   );
   ($p:r_type) <T> Result<(), T> => CRustRes!() {
       $out = match $p {
           Ok(()) => {
               CRustRes!() {
                   data: CRustResUnion!() { ok: 0 },
                   is_ok: 1,
               }
           }
           Err(err) => {
               swig_from_rust_to_i_type!(T, err, err)
               CRustRes!() {
                   data: CRustResUnion!() { err },
                   is_ok: 0,
               }
           }
       };
   };

    ($p:f_type, option = "CppVariant::Boost",
     req_modules = ["\"CRustResModule!().h\"", "<boost/optional.hpp>"],
     unique_prefix = "/*res_empty*/") => "/*res_empty*/boost::optional<swig_f_type!(T)>"
       r#"$p.is_ok != 0 ?
            boost::none :
            boost::optional<swig_f_type!(T)> { swig_foreign_from_i_type!(T, $p.data.err) }"#;

    ($p:f_type, option = "CppVariant::Std17",
     req_modules = ["\"CRustResModule!().h\"", "<optional>"],
     unique_prefix = "/*res_empty*/") => "/*res_empty*/std::optional<swig_f_type!(T)>"
       r#"$p.is_ok != 0 ?
            std::nullopt :
            std::optional<swig_f_type!(T)> { swig_foreign_from_i_type!(T, $p.data.err) }"#;

    ($p:r_type) <T> Result<(), T> <= CRustRes!() {
       $out = unsafe {
           if $p.is_ok != 0 {
               Ok(())
           } else {
               swig_from_i_type_to_rust!(T, $p.data.err, x)
               Err(x)
           }
       };
   };

   ($p:f_type, option = "CppVariant::Boost",
    req_modules = ["\"CRustResModule!().h\"", "<boost/optional.hpp>",
                   "<cassert>", "<utility>"],
    unique_prefix = "/*res_empty*/")
       <= "/*res_empty*/boost::optional<swig_f_type!(T)>"
       r#"[](boost::optional<swig_f_type!(T)> p) -> CRustRes!() {
            CRustRes!() out;
            if (!p) {
                out.is_ok = 1;
            } else {
                swig_f_type!(T) tmp = std::move(*p);
                out.data.err = swig_foreign_to_i_type!(T, tmp);
                out.is_ok = 0;
            }
            return out;
           }(std::move($p))"#;

   ($p:f_type, option = "CppVariant::Std17",
    req_modules = ["\"CRustResModule!().h\"", "<optional>"],
    unique_prefix = "/*res_empty*/")
       <= "/*res_empty*/std::optional<swig_f_type!(T)>"
       r#"[](std::optional<swig_f_type!(T)> p) -> CRustRes!() {
            CRustRes!() out;
            if (!p) {
                out.is_ok = 1;
            } else {
                swig_f_type!(T) tmp = std::move(*p);
                out.data.err = swig_foreign_to_i_type!(T, tmp);
                out.is_ok = 0;
            }
            return out;
          }(std::move($p))"#;
);

foreign_typemap!(
   generic_alias!(CRustRes = swig_concat_idents!(CRustResult, swig_i_type!(T1), swig_i_type!(T2)));
   generic_alias!(CRustResUnion = swig_concat_idents!(CRustResultUnion, swig_i_type!(T1), swig_i_type!(T2)));
   generic_alias!(CRustResModule = swig_concat_idents!(rust_result, swig_i_type!(T1), swig_i_type!(T2)));
   define_c_type!(
        module = "CRustResModule!().h";
       #[repr(C)]
       #[derive(Clone, Copy)]
       pub union CRustResUnion!() {
           ok: swig_i_type!(T1),
           err: swig_i_type!(T2),
       }

       #[repr(C)]
       #[derive(Clone, Copy)]
       pub struct CRustRes!() {
           data: CRustResUnion!(),
           is_ok: u8,
       }
   );
   ($p:r_type) <T1, T2> Result<T1, T2> => CRustRes!() {
       $out = match $p {
           Ok(mut x) => {
               swig_from_rust_to_i_type!(T1, x, ok)
               CRustRes!() {
                   data: CRustResUnion!() { ok },
                   is_ok: 1,
               }
           }
           Err(err) => {
               swig_from_rust_to_i_type!(T2, err, err)
               CRustRes!() {
                   data: CRustResUnion!() { err },
                   is_ok: 0,
               }
           }
       };
   };

   ($p:f_type, option = "CppVariant::Boost", req_modules = ["\"CRustResModule!().h\"", "<boost/variant.hpp>"]) => "boost::variant<swig_f_type!(T1), swig_f_type!(T2)>"
       r#"$p.is_ok != 0 ?
              boost::variant<swig_f_type!(T1), swig_f_type!(T2)> { swig_foreign_from_i_type!(T1, $p.data.ok) } :
              boost::variant<swig_f_type!(T1), swig_f_type!(T2)> { swig_foreign_from_i_type!(T2, $p.data.err) }"#;

   ($p:f_type, option = "CppVariant::Std17", req_modules = ["\"CRustResModule!().h\"", "<variant>"]) => "std::variant<swig_f_type!(T1), swig_f_type!(T2)>"
       r#"$p.is_ok != 0 ?
              std::variant<swig_f_type!(T1), swig_f_type!(T2)> { swig_foreign_from_i_type!(T1, $p.data.ok) } :
              std::variant<swig_f_type!(T1), swig_f_type!(T2)> { swig_foreign_from_i_type!(T2, $p.data.err) }"#;

   ($p:r_type) <T1, T2> Result<T1, T2> <= CRustRes!() {
       $out = unsafe {
           if $p.is_ok != 0 {
               swig_from_i_type_to_rust!(T1, $p.data.ok, x)
               Ok(x)
           } else {
               swig_from_i_type_to_rust!(T2, $p.data.err, x)
               Err(x)
           }
       };
   };

   ($p:f_type, option = "CppVariant::Boost",
    req_modules = ["\"CRustResModule!().h\"", "<boost/variant.hpp>", "<cassert>"])
       <= "boost::variant<swig_f_type!(T1), swig_f_type!(T2)>"
       r#"[](boost::variant<swig_f_type!(T1), swig_f_type!(T2)> p) -> CRustRes!() {
            CRustRes!() out;
            if (boost::get<swig_f_type!(T1)>(&p) != nullptr) {
                swig_f_type!(T1) tmp = boost::get<swig_f_type!(T1)>(std::move(p));
                out.data.ok = swig_foreign_to_i_type!(T1, tmp);
                out.is_ok = 1;
            } else {
                assert(boost::get<swig_f_type!(T2)>(&p) != nullptr);
                swig_f_type!(T2) tmp = boost::get<swig_f_type!(T2)>(std::move(p));
                out.data.err = swig_foreign_to_i_type!(T2, tmp);
                out.is_ok = 0;
            }
            return out;
         }(std::move($p))"#;

   ($p:f_type, option = "CppVariant::Std17",
    req_modules = ["\"CRustResModule!().h\"", "<variant>"])
       <= "std::variant<swig_f_type!(T1), swig_f_type!(T2)>"
       r#"[](std::variant<swig_f_type!(T1), swig_f_type!(T2)> p) -> CRustRes!() {
            CRustRes!() out;
            if (std::get_if<swig_f_type!(T1)>(&p) != nullptr) {
                swig_f_type!(T1) tmp = std::get<swig_f_type!(T1)>(std::move(p));
                out.data.ok = swig_foreign_to_i_type!(T1, tmp);
                out.is_ok = 1;
            } else {
                assert(std::get_if<swig_f_type!(T2)>(&p) != nullptr);
                swig_f_type!(T2) tmp = std::get<swig_f_type!(T2)>(std::move(p));
                out.data.err = swig_foreign_to_i_type!(T2, tmp);
                out.is_ok = 0;
            }
            return out;
          }(std::move($p))"#;
);
