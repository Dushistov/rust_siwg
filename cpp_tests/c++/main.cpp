#define _USE_MATH_DEFINES // for C++
#include <atomic>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstring>
#include <array>
#include <functional>
#include <limits>
#include <iostream>
#include <sstream>
#include <thread>
#include <chrono>
#include <mutex>
#include <gtest/gtest.h>

#include "rust_interface/CheckPrimitiveTypesClass.hpp"
#include "rust_interface/Foo.hpp"
#include "rust_interface/SomeObserver.hpp"
#include "rust_interface/ClassCooperationTest.hpp"
#include "rust_interface/TestObjectLifetime.hpp"
#include "rust_interface/RustForeignVecFoo.h"
#include "rust_interface/TestWorkWithVec.hpp"
#include "rust_interface/TestEnumClass.hpp"
#include "rust_interface/TestPassPathAsParam.hpp"
#include "rust_interface/TestOptional.hpp"
#include "rust_interface/TestError.hpp"
#include "rust_interface/TestResult.hpp"
#include "rust_interface/Position.hpp"
#include "rust_interface/LocationService.hpp"
#include "rust_interface/TestReferences.hpp"
#include "rust_interface/TestOnlyStaticMethods.hpp"
#include "rust_interface/Interface.hpp"
#include "rust_interface/TestPassInterface.hpp"
#include "rust_interface/RecursiveStruct_fwd.hpp"
#include "rust_interface/RecursiveStruct.hpp"
#include "rust_interface/Boo.hpp"
#include "rust_interface/TestPair.hpp"
#include "rust_interface/TestCopy.hpp"
#include "rust_interface/GetSetStrTest.hpp"
#include "rust_interface/TestWorkWithReprC.hpp"
#include "rust_interface/TestFnInline.hpp"
#include "rust_interface/TestFuture.hpp"
#include "rust_interface/ThreadSafeObserver.hpp"
#include "rust_interface/TestMultiThreadCallback.hpp"
#include "rust_interface/Session.hpp"

using namespace rust;

static std::atomic<uint32_t> c_simple_cb_counter{ 0 };
static std::atomic<uint32_t> c_simple_cb_counter_without_args{ 0 };

static void c_delete_int(void *opaque)
{
    printf("clear\n");
    auto self = static_cast<int *>(opaque);
    ASSERT_EQ(17, *self);
    delete self;
}

static void c_simple_cb(int32_t a, char b, void *opaque)
{
    assert(opaque != nullptr);
    const int tag = *static_cast<int *>(opaque);
    ASSERT_EQ(17, tag);
    printf("!!! a %d, b: %d, tag %d\n", static_cast<int>(a), b, tag);
    ++c_simple_cb_counter;
}

static void c_simple_cb_without_args(void *opaque)
{
    assert(opaque != nullptr);
    const int tag = *static_cast<int *>(opaque);
    ASSERT_EQ(17, tag);
    ++c_simple_cb_counter_without_args;
}

static char c_simple_cb_is_odd(int32_t x, void *opaque)
{
    assert(opaque != nullptr);
    const int tag = *static_cast<int *>(opaque);
    EXPECT_EQ(17, tag);
    return (x % 2) == 1;
}

TEST(c_Foo, Simple)
{
    auto foo = Foo_new(1, CRustStrView{ "a", 1 });
    ASSERT_TRUE(foo != nullptr);

    EXPECT_EQ(3, Foo_f(foo, 1, 1));
    auto name = Foo_getName(foo);
    EXPECT_EQ(std::string("a"), std::string(name.data, name.len));

    Foo_set_field(foo, 5);
    EXPECT_EQ(7, Foo_f(foo, 1, 1));
    const C_SomeObserver obs = {
        new int(17), c_delete_int, c_simple_cb, c_simple_cb_without_args, c_simple_cb_is_odd,
    };
    c_simple_cb_counter = 0;
    c_simple_cb_counter_without_args = 0;
    Foo_call_me(&obs);
    EXPECT_EQ(1u, c_simple_cb_counter.load());
    EXPECT_EQ(1u, c_simple_cb_counter_without_args.load());
    Foo_delete(foo);
}

TEST(Foo, Simple)
{
    Foo foo(1, "b");
    EXPECT_EQ(3, foo.f(1, 1));
    {
        auto name = foo.getName();
        EXPECT_EQ(std::string("b"), std::string(name.data(), name.size()));
    }
    EXPECT_NEAR(std::hypot(1., 1.) + 1., foo.f_double(1., 1.), 1e-10);
    EXPECT_NEAR(std::hypot(1.0, 1.0), Foo::fHypot(1.0, 1.0), 1e-10);
    foo.set_field(5);
    EXPECT_EQ(7, foo.f(1, 1));
    const C_SomeObserver obs = {
        new int(17), c_delete_int, c_simple_cb, c_simple_cb_without_args, c_simple_cb_is_odd,
    };
    c_simple_cb_counter = 0;
    c_simple_cb_counter_without_args = 0;
    Foo_call_me(&obs);
    EXPECT_EQ(1u, c_simple_cb_counter.load());
    EXPECT_EQ(1u, c_simple_cb_counter_without_args.load());

    EXPECT_NEAR(7.5, foo.one_and_half(), 1e-16);
    {
        Foo f2(17, "");
        EXPECT_EQ(19, f2.f(1, 1));
        auto name = f2.getName();
        EXPECT_EQ(std::string(""), std::string(name.data(), name.size()));
    }

    EXPECT_EQ(4u, foo.cpp_func(std::string("abcd")));
}

namespace {
struct MySomeObserver final : public SomeObserver {
    static size_t f1_call;
    static size_t f2_call;
    static size_t deleted;

    MySomeObserver()
    {
        f1_call = 0;
        f2_call = 0;
        deleted = 0;
    }
    ~MySomeObserver() { ++deleted; }
    void onStateChanged(int32_t a, bool b) noexcept override
    {
        std::cout << "onStateChanged: a: " << a << ", b: " << b << "\n";
        ASSERT_EQ(2, a);
        ASSERT_FALSE(!!b);
        ++f1_call;
    }
    void onStateChangedWithoutArgs(void) noexcept override
    {
        std::cout << "onStateChangedWithoutArgs\n";
        ++f2_call;
    }
    bool isOdd(int32_t num) noexcept override { return num % 2 == 1; }
};

size_t MySomeObserver::f1_call = 0;
size_t MySomeObserver::f2_call = 0;
size_t MySomeObserver::deleted = 0;
} // namespace

TEST(Foo, CppSomeObserver)
{
    Foo foo(17, "CppSomeObserver");
    std::unique_ptr<MySomeObserver> obs{ new MySomeObserver };
    ASSERT_EQ(0u, obs->f1_call);
    ASSERT_EQ(0u, obs->f2_call);
    Foo::call_me(std::move(obs));
    EXPECT_EQ(1u, MySomeObserver::f1_call);
    EXPECT_EQ(1u, MySomeObserver::f2_call);
    EXPECT_EQ(1u, MySomeObserver::deleted);
}

TEST(CheckPrimitiveTypesClass, smokeTest)
{
    CheckPrimitiveTypesClass x;

    EXPECT_TRUE(CheckPrimitiveTypesClass::invert(false));
    EXPECT_FALSE(CheckPrimitiveTypesClass::invert(true));

    EXPECT_TRUE(CheckPrimitiveTypesClass::Xor(true, false));
    EXPECT_TRUE(CheckPrimitiveTypesClass::Xor(false, true));
    EXPECT_FALSE(CheckPrimitiveTypesClass::Xor(true, true));
    EXPECT_FALSE(CheckPrimitiveTypesClass::Xor(false, false));

    EXPECT_NEAR(static_cast<float>(M_E), x.test(true), std::numeric_limits<float>::epsilon());
    EXPECT_NEAR(static_cast<float>(M_PI), x.test(false), std::numeric_limits<float>::epsilon());
    EXPECT_EQ(255, CheckPrimitiveTypesClass::test_u8(254u));
    EXPECT_EQ(0, CheckPrimitiveTypesClass::test_i8(-1));
    EXPECT_EQ(65535u, CheckPrimitiveTypesClass::test_u16(65534u));
    EXPECT_EQ(0, CheckPrimitiveTypesClass::test_i16(-1));
    EXPECT_EQ(4294967295u, CheckPrimitiveTypesClass::test_u32(4294967295u - 1));
    EXPECT_EQ(0, CheckPrimitiveTypesClass::test_i32(-1));
    EXPECT_EQ(18446744073709551615ull,
              CheckPrimitiveTypesClass::test_u64(18446744073709551615ull - 1));
    EXPECT_EQ(0, CheckPrimitiveTypesClass::test_i64(-1));
    EXPECT_NEAR(2.1f, CheckPrimitiveTypesClass::test_f32(1.1f), 1e-12f);
    EXPECT_NEAR(0., CheckPrimitiveTypesClass::test_f64(-1.0), 1e-12);
}

TEST(ClassCooperationTest, smokeTest)
{
    ClassCooperationTest x;
    auto f1 = x.get(0);
    EXPECT_EQ(std::string("5"), f1.getName());
    EXPECT_EQ(5, f1.f(0, 0));
    auto f2 = x.get(1);
    EXPECT_EQ(std::string("7"), f2.getName());
    EXPECT_EQ(6, f2.f(0, 0));

    Foo new_f2{ 437, "437" };
    x.set(1, std::move(new_f2));
    f2 = x.get(1);
    EXPECT_EQ(std::string("437"), f2.getName());
    EXPECT_EQ(437, f2.f(0, 0));
}

TEST(TestObjectLifetime, smokeTest)
{
    TestObjectLifetime x;
    EXPECT_EQ(5, x.get_data());
    x.set_data(1, 2, 3, 4., 5.);
    EXPECT_EQ(15, x.get_data());
}

static void validate_create_foo_vec(size_t n, const RustForeignVecFoo &vec)
{
    ASSERT_EQ(n, vec.size());

    size_t i = 0;
    std::stringstream fmt;
    for (auto &&elem : vec) {
        ASSERT_EQ(static_cast<int32_t>(i), elem.f(0, 0));
        fmt << i;
        ASSERT_EQ(fmt.str(), elem.getName());
        fmt.str(std::string());
        fmt.clear();
        ++i;
    }
    ASSERT_EQ(n, i);
}

TEST(TestWorkWithVec, smokeTest)
{
    const char tag[] = "Test data";
    const size_t tag_len = std::strlen(tag);
    TestWorkWithVec t{ tag };
    for (uint32_t n : { 0, 1, 2, 3, 5, 10, 100, 1000 }) {
        auto vec = t.get_bytes(n);
        EXPECT_TRUE(n == 0 || !vec.empty());
        EXPECT_EQ(tag_len * n, vec.size());
        for (size_t i = 0; i < vec.size(); i += std::strlen(tag)) {
            EXPECT_TRUE(i + tag_len <= vec.size());
            EXPECT_EQ(std::string(tag),
                      std::string(reinterpret_cast<const char *>(&vec[i]), tag_len));
        }
        vec = t.get_bytes(n);
        EXPECT_EQ(tag_len * n, vec.size());
        for (size_t i = 0; i < vec.size(); i += std::strlen(tag)) {
            EXPECT_TRUE(i + tag_len <= vec.size());
            EXPECT_EQ(std::string(tag),
                      std::string(reinterpret_cast<const char *>(&vec[i]), tag_len));
        }
    }

    auto sp = t.get_u32_slice();
    ASSERT_EQ(tag_len + 1, sp.size());
    for (size_t i = 0; i < tag_len; ++i) {
        EXPECT_EQ(i, sp[i]);
    }
    EXPECT_EQ(uint32_t(1) << 30, sp[tag_len]);

    static_assert(std::is_same<RustVecu32::value_type, uint32_t>::value,
                  "RustVecu32::value_type should be uint32_t");
    RustVecu32 vec_u32{ t.get_vec_u32() };
    ASSERT_EQ(tag_len + 1, vec_u32.size());
    for (size_t i = 0; i < tag_len; ++i) {
        EXPECT_EQ(i, vec_u32[i]);
    }
    EXPECT_EQ(uint32_t(1) << 30, vec_u32[tag_len]);

    RustVecf32 vec_f32{ t.get_vec_f32() };
    ASSERT_EQ(2u, vec_f32.size());
    EXPECT_NEAR(static_cast<float>(M_E), vec_f32[0], std::numeric_limits<float>::epsilon());
    EXPECT_NEAR(static_cast<float>(M_PI), vec_f32[1], std::numeric_limits<float>::epsilon());

    RustVecf64 vec_f64{ t.get_vec_f64() };
    ASSERT_EQ(2u, vec_f64.size());
    EXPECT_NEAR(static_cast<double>(M_E), vec_f64[0], std::numeric_limits<double>::epsilon());
    EXPECT_NEAR(static_cast<double>(M_PI), vec_f64[1], std::numeric_limits<double>::epsilon());

    RustForeignVecFoo vec_foo = t.get_vec_foo();
    ASSERT_EQ(tag_len, vec_foo.size());
    for (size_t i = 0; i < vec_foo.size(); ++i) {
        EXPECT_EQ(std::string(tag), vec_foo[i].getName());
        EXPECT_TRUE(vec_foo[i].f(0, 0) >= 0);
        EXPECT_EQ(i, size_t(vec_foo[i].f(0, 0)));
    }
    vec_foo.push(Foo{ 57, "boo" });
    ASSERT_EQ(tag_len + 1, vec_foo.size());
    EXPECT_EQ(57, vec_foo[vec_foo.size() - 1].f(0, 0));
    EXPECT_EQ(std::string("boo"), vec_foo[vec_foo.size() - 1].getName());
    {
        auto elem = vec_foo.remove(tag_len);
        EXPECT_EQ(tag_len, vec_foo.size());
        EXPECT_EQ(57, elem.f(0, 0));
        EXPECT_EQ(std::string("boo"), elem.getName());
        for (size_t i = 0; i < vec_foo.size(); ++i) {
            EXPECT_EQ(std::string(tag), vec_foo[i].getName());
            EXPECT_TRUE(vec_foo[i].f(0, 0) >= 0);
            EXPECT_EQ(i, size_t(vec_foo[i].f(0, 0)));
        }
    }
    {
        auto v = TestWorkWithVec::create_foo_vec(30);
        auto v1 = TestWorkWithVec::create_foo_vec(0);
        while (!v.empty()) {
            v1.push(v.remove(v.size() - 1));
        }
        size_t i = v1.size() - 1;
        for (const auto &elem : v1) {
            EXPECT_EQ(i, static_cast<size_t>(elem.f(0, 0)));
            --i;
        }
        TestWorkWithVec::sort_foo_slice(v1.as_slice_mut());
        i = 0;
        for (const auto &elem : v1) {
            EXPECT_EQ(i, static_cast<size_t>(elem.f(0, 0)));
            ++i;
        }
    }

    {
        auto sl = t.return_usize_slice();
        ASSERT_EQ(2u, sl.size());
        EXPECT_EQ(17u, sl[0]);
        EXPECT_EQ(18u, sl[1]);

        auto v = t.return_usize_vec();
        ASSERT_EQ(2u, v.size());
        EXPECT_EQ(17u, v[0]);
        EXPECT_EQ(18u, v[1]);
    }
    {
        auto v = TestWorkWithVec::create_foo_vec(30);
        validate_create_foo_vec(30, v);
        auto v1 = TestWorkWithVec::clone_foo_slice(v.as_slice());
        validate_create_foo_vec(30, v1);
    }
    {
        const std::array<int32_t, 5> a{ { -(int32_t(1) << 29), -10, 0, 17, int32_t(1) << 30 } };
        auto v = TestWorkWithVec::test_i32_slice(RustSlice<const int32_t>{ &a[0], a.size() });
        ASSERT_EQ(a.size(), v.size());
        for (size_t i = 0; i < a.size(); ++i) {
            EXPECT_EQ(a[i] + 1, v[i]);
        }
    }
    {
        auto v = TestWorkWithVec::create_foo_vec(30);
        validate_create_foo_vec(30, v);
        TestWorkWithVec tester{ tag };
        tester.set_vec_foo(std::move(v));
        auto v2 = tester.get_vec_foo();
        validate_create_foo_vec(30, v2);
    }

    {
        auto v = TestWorkWithVec::test_lifetime_objs(1000);
        uint32_t i = 0;
        for (const auto &&e : v) {
            ASSERT_EQ(static_cast<int32_t>(i), e.get_data());
            ++i;
        }
    }

    {
        std::array<int32_t, 5> a{ { -17, 60, 5, 33, 18 } };
        auto a2 = a;
        TestWorkWithVec::sort_i32_slice(RustSlice<int32_t>{ &a[0], a.size() });
        std::sort(a2.begin(), a2.end());
        EXPECT_EQ(a2, a);
    }
}

TEST(TestWorkWithVec, assign)
{
    const size_t N = 2000;
    auto vec = TestWorkWithVec::create_foo_vec(N);
    validate_create_foo_vec(N, vec);

    RustForeignVecFoo vec2;
    vec2 = TestWorkWithVec::create_foo_vec(100);
    validate_create_foo_vec(100, vec2);
    vec2 = TestWorkWithVec::create_foo_vec(100);
    validate_create_foo_vec(100, vec2);

    RustForeignVecFoo vec3 = TestWorkWithVec::create_foo_vec(200);
    validate_create_foo_vec(200, vec3);
    vec3 = std::move(vec2);
    validate_create_foo_vec(100, vec3);
    EXPECT_TRUE(vec2.empty());
}

TEST(TestWorkWithVec, iterator)
{
    const char tag[] = "Test data";
    const size_t tag_len = std::strlen(tag);
    TestWorkWithVec t{ tag };
    auto slice_foo = t.get_slice_foo();
    ASSERT_EQ(tag_len, slice_foo.size());
    for (size_t i = 0; i < slice_foo.size(); ++i) {
        EXPECT_EQ(std::string(tag), slice_foo[i].getName());
        EXPECT_TRUE(slice_foo[i].f(0, 0) >= 0);
        EXPECT_EQ(i, size_t(slice_foo[i].f(0, 0)));
    }
    size_t i = 0;
    for (auto foo : slice_foo) {
        EXPECT_EQ(std::string(tag), foo.getName());
        EXPECT_TRUE(foo.f(0, 0) >= 0);
        EXPECT_EQ(i, size_t(foo.f(0, 0)));
        ++i;
    }

    {
        auto it = std::find_if(slice_foo.begin(), slice_foo.end(), [tag_len](const FooRef &item) {
            return static_cast<size_t>(item.f(0, 0)) == tag_len / 2;
        });
        ASSERT_NE(slice_foo.end(), it);
        ASSERT_EQ(tag_len / 2, static_cast<size_t>((*it).f(0, 0)));
        ASSERT_EQ(tag_len / 2, size_t(it - slice_foo.begin()));
    }
    {
        RustForeignVecFoo empty_vec;
        auto it = std::find_if(empty_vec.begin(), empty_vec.end(), [tag_len](const FooRef &item) {
            return static_cast<size_t>(item.f(0, 0)) == tag_len / 2;
        });
        ASSERT_EQ(empty_vec.end(), it);
    }
}

TEST(TestEnumClass, smokeTest)
{
    TestEnumClass x;
    ASSERT_EQ(-5, x.f1(ITEM1));
    ASSERT_EQ(-5, x.f1(ITEM3));
    ASSERT_EQ(17, x.f1(ITEM2));
    ASSERT_EQ(ITEM2, TestEnumClass::next_enum(ITEM1));
    ASSERT_EQ(ITEM3, TestEnumClass::next_enum(ITEM2));
    ASSERT_EQ(ITEM1, TestEnumClass::next_enum(ITEM3));
}

TEST(TestPassPathAsParam, smokeTest)
{
    TestPassPathAsParam x;
    x.set_path("/tmp/a.txt");
    ASSERT_EQ("\"/tmp/a.txt\"", x.path());
}

TEST(TestRustStringReturn, smokeTest)
{
    auto try_ret = [](const char *word) {
        Foo foo(1, word);
        EXPECT_EQ(std::string(word), foo.getName());
        EXPECT_EQ(std::string(word), foo.ret_string().to_std_string());
    };
    try_ret("Word");
    try_ret("");
    for (size_t i = 0; i < 100; ++i) {
        std::string expect(i, 'A');
        try_ret(expect.c_str());
    }
}

#if (defined(HAS_STDCXX_17) && !defined(NO_HAVE_STD17_OPTIONAL)) || defined(USE_BOOST)
TEST(TestOptional, smokeTest)
{
    TestOptional x;
    {
        auto foo = x.f1(true);
        ASSERT_TRUE(!!foo);
        EXPECT_EQ(17, foo->f(0, 0));
        EXPECT_EQ(std::string("17"), foo->getName());
    }
    {
        auto foo = x.f1(false);
        EXPECT_FALSE(!!foo);
    }
    {
        auto val = x.f2(true);
        ASSERT_TRUE(!!val);
        EXPECT_NEAR(static_cast<double>(M_E), *val, std::numeric_limits<double>::epsilon());
        auto val2 = x.f2(false);
        ASSERT_FALSE(!!val2);
    }
    {
        auto val = x.f3(true);
        ASSERT_TRUE(!!val);
        EXPECT_EQ(17u, *val);
        auto val2 = x.f3(false);
        ASSERT_FALSE(!!val2);
    }

    EXPECT_NEAR(10., x.f4({ 5. }), std::numeric_limits<double>::epsilon());
    EXPECT_NEAR(-1., x.f4({}), std::numeric_limits<double>::epsilon());

    {
        auto val = x.f5(true);
        ASSERT_TRUE(!!val);
        FooRef foo = std::move(*val);
        EXPECT_EQ(5, foo.f(0, 0));
        EXPECT_EQ(std::string("aaa"), foo.getName());
    }
    {
        auto foo = x.f5(false);
        EXPECT_FALSE(!!foo);
    }

    {
        Foo foo(17, "17");
        x.f6({ std::move(foo) });
        auto val = x.f5(true);
        ASSERT_TRUE(!!val);
        FooRef foor = std::move(*val);
        EXPECT_EQ(17, foor.f(0, 0));
        EXPECT_EQ(std::string("17"), foor.getName());
        x.f6({});
    }

    {
        auto val = x.f7();
        ASSERT_TRUE(!!val);
        EXPECT_EQ(ITEM1, *val);
    }
    {
        auto val = x.f9({ { "aaa" } });
        EXPECT_EQ(std::string("your name is aaa"), val.to_std_string());
        auto val2 = x.f9({});
        EXPECT_EQ(std::string("None"), val2.to_std_string());
    }
    {
        TestOptional y;
        auto val = y.f10(true);
        ASSERT_TRUE(!!val);
        EXPECT_EQ(std::string("aaa"), *val);
        auto val2 = y.f10(false);
        EXPECT_TRUE(!val2);
    }
    {
        TestOptional y;
        auto val = y.f11(17, true);
        ASSERT_TRUE(!!val);
        EXPECT_EQ(17, *val);
        auto val2 = y.f11(0, false);
        EXPECT_TRUE(!val2);
        val = y.f11(-17, true);
        ASSERT_TRUE(!!val);
        EXPECT_EQ(-17, *val);
    }
    {
        TestOptional y;
        auto val = y.f12(17.5, true);
        ASSERT_TRUE(!!val);
        EXPECT_NEAR(17.5, *val, 1e-5);
        auto val2 = y.f12(0., false);
        EXPECT_TRUE(!val2);
        val = y.f12(-17.434, true);
        ASSERT_TRUE(!!val);
        EXPECT_NEAR(-17.434, *val, 1e-5);
    }

    {
        TestOptional y;
        auto val = y.f13(true);
        ASSERT_TRUE(!!val);
        const std::string expect(R"(There was a Young Lady whose nose,
Was so long that it reached to her toes;
So she hired an Old Lady,
Whose conduct was steady,
To carry that wonderful nose.)");
        EXPECT_EQ(expect, val->to_std_string());
        auto val2 = y.f13(false);
        EXPECT_TRUE(!val2);
    }

    {
        TestOptional y;
        auto val = y.f14(true);
        ASSERT_TRUE(!!val);
        EXPECT_TRUE(*val);
        auto val2 = y.f14(false);
        EXPECT_TRUE(!val2);
    }

    {
        Foo foo(17, "17");
        auto val = TestOptional::f15(&foo);
        ASSERT_TRUE(!!val);
        EXPECT_EQ("17", val->to_std_string());

        auto val_none = TestOptional::f15(nullptr);
        EXPECT_TRUE(!val_none);
    }

    {
        Foo foo(17, "17");
        TestOptional::f16(&foo, 5);
        EXPECT_EQ(5, foo.f(0, 0));

        TestOptional::f16(nullptr, 17);
    }
}

TEST(TestResult, smokeTest)
{
#if defined(HAS_STDCXX_17) && !defined(NO_HAVE_STD17_VARIANT)
    std::variant<TestResult, RustString> res = TestResult::new_with_err();
    EXPECT_TRUE(nullptr == std::get_if<TestResult>(&res));
    EXPECT_TRUE(nullptr != std::get_if<RustString>(&res));
    EXPECT_EQ(std::string_view("this is error"), std::get<RustString>(res).to_string_view());
    auto res2 = TestResult::f(true);
    EXPECT_EQ(std::nullopt, res2);
    auto res3 = TestResult::f(false);
    ASSERT_TRUE(!!res3);
    EXPECT_EQ(std::string_view("Not ok"), res3->to_string_view());

    auto res_vec = TestResult::f_vec(true);
    EXPECT_TRUE(nullptr == std::get_if<RustString>(&res_vec));
    EXPECT_TRUE(nullptr != std::get_if<RustForeignVecFoo>(&res_vec));
    auto vec = std::get<RustForeignVecFoo>(std::move(res_vec));
    ASSERT_EQ(2u, vec.size());
    EXPECT_EQ(std::string("15"), vec[0].getName());
    EXPECT_EQ(15, vec[0].f(0, 0));
    EXPECT_EQ(std::string("13"), vec[1].getName());
    EXPECT_EQ(13, vec[1].f(0, 0));
    res_vec = TestResult::f_vec(false);
    EXPECT_TRUE(nullptr != std::get_if<RustString>(&res_vec));
    EXPECT_TRUE(nullptr == std::get_if<RustForeignVecFoo>(&res_vec));
    EXPECT_EQ(std::string_view("Not ok"), std::get<RustString>(res_vec).to_string_view());

    auto f2_ok = TestResult::f2(true);
    EXPECT_TRUE(nullptr != std::get_if<Foo>(&f2_ok));
    EXPECT_TRUE(nullptr == std::get_if<TestError>(&f2_ok));
    Foo f2_ok_ret = std::get<Foo>(std::move(f2_ok));
    ASSERT_EQ(17, f2_ok_ret.f(0, 0));
    ASSERT_EQ(std::string_view("ok"), f2_ok_ret.getName());

    auto f2_err = TestResult::f2(false);
    EXPECT_TRUE(nullptr == std::get_if<Foo>(&f2_err));
    EXPECT_TRUE(nullptr != std::get_if<TestError>(&f2_err));
    TestError f2_err_ret = std::get<TestError>(std::move(f2_err));
    ASSERT_EQ(std::string_view("Not ok"), RustString{ f2_err_ret.to_string() }.to_string_view());

    auto f3_ok = TestResult::f3(true);
    EXPECT_TRUE(nullptr != std::get_if<RustForeignVecFoo>(&f3_ok));
    EXPECT_TRUE(nullptr == std::get_if<TestError>(&f3_ok));
    auto f3_vec = std::get<RustForeignVecFoo>(std::move(f3_ok));

    ASSERT_EQ(2u, f3_vec.size());
    EXPECT_EQ(std::string_view("40"), f3_vec[0].getName());
    EXPECT_EQ(40, f3_vec[0].f(0, 0));
    EXPECT_EQ(std::string_view(""), f3_vec[1].getName());
    EXPECT_EQ(60, f3_vec[1].f(0, 0));

    auto f3_err = TestResult::f3(false);
    EXPECT_TRUE(nullptr == std::get_if<RustForeignVecFoo>(&f3_err));
    EXPECT_TRUE(nullptr != std::get_if<TestError>(&f3_err));
    TestError f3_err_ret = std::get<TestError>(std::move(f3_err));
    ASSERT_EQ(std::string_view("Not ok"), RustString{ f3_err_ret.to_string() }.to_string_view());

    {
        auto f4_ok = TestResult::f4(true);
        EXPECT_TRUE(nullptr != std::get_if<RustVecu8>(&f4_ok));
        EXPECT_TRUE(nullptr == std::get_if<TestError>(&f4_ok));
        auto f4_vec = std::get<RustVecu8>(std::move(f4_ok));
        ASSERT_EQ(2u, f4_vec.size());
        EXPECT_EQ(17, f4_vec[0]);
        EXPECT_EQ(18, f4_vec[1]);
    }
    {
        auto f5_ok = TestResult::f5(true);
        EXPECT_TRUE(nullptr != std::get_if<Foo>(&f5_ok));
        EXPECT_TRUE(nullptr == std::get_if<ErrorEnum>(&f5_ok));
        auto f5_ok_ret = std::get<Foo>(std::move(f5_ok));
        ASSERT_EQ(17, f5_ok_ret.f(0, 0));
        ASSERT_EQ(std::string_view("ok"), f5_ok_ret.getName());
        auto f5_err = TestResult::f5(false);
        EXPECT_TRUE(nullptr == std::get_if<Foo>(&f5_err));
        EXPECT_TRUE(nullptr != std::get_if<ErrorEnum>(&f5_err));
        auto f5_err_ret = std::get<ErrorEnum>(std::move(f5_err));
        EXPECT_EQ(eeB, f5_err_ret);
    }
    {
        auto ok = TestResult::f6(true, 1000 * 1000);
        EXPECT_TRUE(nullptr != std::get_if<int64_t>(&ok));
        EXPECT_TRUE(nullptr == std::get_if<TestError>(&ok));
        auto ok_ret = std::get<int64_t>(std::move(ok));
        ASSERT_EQ((1000 * 1000 + 1), ok_ret);

        auto err = TestResult::f6(false, -1);
        EXPECT_TRUE(nullptr == std::get_if<int64_t>(&err));
        EXPECT_TRUE(nullptr != std::get_if<TestError>(&err));
        TestError err_ret = std::get<TestError>(std::move(err));
        ASSERT_EQ(std::string_view("Not ok"), RustString{ err_ret.to_string() }.to_string_view());
    }
#endif // HAS_STDCXX_17
#ifdef USE_BOOST
    {
        boost::variant<TestResult, RustString> res = TestResult::new_with_err();
        EXPECT_EQ(nullptr, boost::get<TestResult>(&res));
        EXPECT_NE(nullptr, boost::get<RustString>(&res));
#ifdef HAS_BOOST_STRING_VIEW_HPP
        EXPECT_EQ(boost::string_view("this is error"),
                  boost::get<RustString>(std::move(res)).to_boost_string_view());
#else
        EXPECT_EQ(std::string("this is error"),
                  boost::get<RustString>(std::move(res)).to_std_string());
#endif
        auto res2 = TestResult::f(true);
        EXPECT_EQ(boost::none, res2);
        auto res3 = TestResult::f(false);
        ASSERT_TRUE(!!res3);
#ifdef HAS_BOOST_STRING_VIEW_HPP
        EXPECT_EQ(boost::string_view("Not ok"), res3->to_boost_string_view());
#endif
        EXPECT_EQ(std::string("Not ok"), res3->to_std_string());
    }
    {
        auto res_vec = TestResult::f_vec(true);
        EXPECT_EQ(nullptr, boost::get<RustString>(&res_vec));
        EXPECT_NE(nullptr, boost::get<RustForeignVecFoo>(&res_vec));
        auto vec = std::move(boost::get<RustForeignVecFoo>(std::move(res_vec)));
        ASSERT_EQ(2u, vec.size());

        EXPECT_EQ(std::string("15"), vec[0].getName());
        EXPECT_EQ(std::string("13"), vec[1].getName());

        EXPECT_EQ(15, vec[0].f(0, 0));
        EXPECT_EQ(13, vec[1].f(0, 0));
    }
    auto res_vec = TestResult::f_vec(false);
    EXPECT_NE(nullptr, boost::get<RustString>(&res_vec));
    EXPECT_EQ(nullptr, boost::get<RustForeignVecFoo>(&res_vec));
#ifdef HAS_BOOST_STRING_VIEW_HPP
    EXPECT_EQ(boost::string_view("Not ok"), boost::get<RustString>(res_vec).to_boost_string_view());
#else
    EXPECT_EQ(std::string("Not ok"), boost::get<RustString>(res_vec).to_std_string());
#endif

    {
        auto ok = TestResult::f6(true, 1000 * 1000);
        EXPECT_NE(nullptr, boost::get<int64_t>(&ok));
        EXPECT_EQ(nullptr, boost::get<TestError>(&ok));
        auto ok_ret = boost::get<int64_t>(std::move(ok));
        ASSERT_EQ((1000 * 1000 + 1), ok_ret);

        auto err = TestResult::f6(false, -1);
        EXPECT_EQ(nullptr, boost::get<int64_t>(&err));
        EXPECT_NE(nullptr, boost::get<TestError>(&err));
        TestError err_ret = boost::get<TestError>(std::move(err));
        ASSERT_EQ(std::string("Not ok"), err_ret.to_string().to_std_string());
    }
#endif // USE_BOOST
}

#if defined(__GNUC__) && defined(__GNUG__) && !defined(__clang__)
#if __GNUC_PREREQ(9, 0)
#define SKIP_TEST_RESULT_COMPOUND
#else
#undef SKIP_TEST_RESULT_COMPOUND
#endif
#else
#undef SKIP_TEST_RESULT_COMPOUND
#endif

#ifndef SKIP_TEST_RESULT_COMPOUND
// there is some issue with optional/may-be-unused and valgrind
// see https://gcc.gnu.org/bugzilla/show_bug.cgi?id=80635
TEST(TestResult, Compound)
{
#if defined(HAS_STDCXX_17) && !defined(NO_HAVE_STD17_VARIANT)
    {
        auto res1 = TestResult::f_res_opt(0);
        ASSERT_TRUE(nullptr != std::get_if<std::optional<Foo>>(&res1));
        auto res1_ok = std::get<std::optional<Foo>>(std::move(res1));
        ASSERT_TRUE(!!res1_ok);
        EXPECT_EQ("17", res1_ok->getName());
        EXPECT_EQ((17 + 1), res1_ok->f(0, 1));

        auto res2 = TestResult::f_res_opt(1);
        ASSERT_TRUE(nullptr != std::get_if<std::optional<Foo>>(&res2));
        auto res2_ok = std::get<std::optional<Foo>>(std::move(res2));
        EXPECT_TRUE(!res2_ok);

        auto res3 = TestResult::f_res_opt(2);
        ASSERT_TRUE(nullptr != std::get_if<RustString>(&res3));
        auto res3_ok = std::get<RustString>(std::move(res3));
        EXPECT_EQ("this is bad", res3_ok.to_string_view());
    }
#endif
#ifdef USE_BOOST
    {
        auto res1 = TestResult::f_res_opt(0);
        ASSERT_TRUE(nullptr != boost::get<boost::optional<Foo>>(&res1));
        auto res1_ok = boost::get<boost::optional<Foo>>(boost::move(res1));
        ASSERT_TRUE(!!res1_ok);
        EXPECT_EQ("17", res1_ok->getName());
        EXPECT_EQ((17 + 1), res1_ok->f(0, 1));

        auto res2 = TestResult::f_res_opt(1);
        ASSERT_TRUE(nullptr != boost::get<boost::optional<Foo>>(&res2));
        auto res2_ok = boost::get<boost::optional<Foo>>(boost::move(res2));
        EXPECT_TRUE(!res2_ok);

        auto res3 = TestResult::f_res_opt(2);
        ASSERT_TRUE(nullptr != boost::get<RustString>(&res3));
        auto res3_ok = boost::get<RustString>(boost::move(res3));
        EXPECT_EQ("this is bad", res3_ok.to_boost_string_view());
    }
#endif
}
#endif // SKIP_TEST_RESULT_COMPOUND
#endif

TEST(TestReferences, smokeTest)
{
    TestReferences tr(500, "bugaga");
    auto foo = tr.get_foo_ref();
    EXPECT_EQ(502, foo.f(1, 1));
    EXPECT_EQ(std::string("bugaga"), foo.getName());

    Foo new_foo(100, "100");
    tr.update_foo(new_foo);
    foo = tr.get_foo_ref();
    EXPECT_EQ(102, foo.f(1, 1));
    EXPECT_EQ(std::string("100"), foo.getName());

    Foo foo2(200, "200");
    tr.update_mut_foo(foo2);
    foo = tr.get_foo_ref();
    EXPECT_EQ(202, foo.f(1, 1));
    EXPECT_EQ(std::string("200A"), foo2.getName());

    CheckPrimitiveTypesClass p1;
    p1.setA(15);
    EXPECT_EQ(15, TestReferences::check_rc_pass1(p1));
    EXPECT_EQ(15 + 42, TestReferences::check_rc_pass2(p1));
    EXPECT_EQ(15 + 42, p1.getA());
}

TEST(TestOnlyStaticMethods, smokeTest) { EXPECT_EQ(4, TestOnlyStaticMethods::add_func(2, 2)); }

#if (defined(HAS_STDCXX_17) && !defined(NO_HAVE_STD17_VARIANT)) || defined(USE_BOOST)
TEST(TestDummyConstructor, smokeTest)
{
    auto res = LocationService::position();
#ifdef HAS_STDCXX_17
    ASSERT_TRUE(nullptr != std::get_if<Position>(&res));
    auto pos = std::get<Position>(std::move(res));
#endif // HAS_STDCXX_17
#ifdef USE_BOOST
    ASSERT_TRUE(nullptr != boost::get<Position>(&res));
    auto pos = boost::get<Position>(std::move(res));
#endif // USE_BOOST
    EXPECT_NEAR(0.1, pos.latitude(), 1e-10);
}
#endif

TEST(Interface, smokeTest)
{
    Interface x;
    EXPECT_EQ(18, x.f(1));
    x.set(0);
    EXPECT_EQ(1, x.f(1));
    EXPECT_EQ(17, TestPassInterface::use_interface(std::move(x), 17));
}

TEST(RecursiveStruct, smokeTest)
{
    RecursiveStruct s{ "aaa", "aaa/bbb", "aaa/ccc" };

    EXPECT_EQ(std::string{ "aaa" }, s.tag());
    ASSERT_EQ(2u, s.childs().size());
    EXPECT_EQ(std::string{ "aaa/bbb" }, s.childs()[0].tag());
    EXPECT_EQ(std::string{ "aaa/ccc" }, s.childs()[1].tag());
}

TEST(TestPair, smokeTest)
{
    auto pair = TestPair::return_pair_obj1();
    EXPECT_EQ(17, pair.second.f());
    EXPECT_EQ(5, pair.first.f(0, 0));
    EXPECT_EQ(std::string("FooName"), pair.first.getName());

    const auto pair2 = TestPair::return_pair_obj2();
    EXPECT_EQ(17, pair2.first.f());
    EXPECT_EQ(5, pair2.second.f(0, 0));
    EXPECT_EQ(std::string("FooName"), pair2.second.getName());

    const auto pair3 = TestPair::return_pair_f32_i32();
    EXPECT_NEAR(42.42f, pair3.first, std::numeric_limits<float>::epsilon());
    EXPECT_EQ(17, pair3.second);

    pair.first.set_field(11);
    const auto pair4 = TestPair::swap_foo_boo(std::move(pair));
    EXPECT_EQ(17, pair4.first.f());
    EXPECT_EQ(11, pair4.second.f(0, 0));
    EXPECT_EQ(std::string("FooName"), pair4.second.getName());

    {
        bool p1 = false;
        int32_t p2 = -1;
        RustString p3;
        std::tie(p1, p2, p3) = TestPair::return_tuple3();
        EXPECT_TRUE(p1);
        EXPECT_EQ(17, p2);
        EXPECT_EQ("tuple3", p3.to_std_string());
    }
}

TEST(TestCopy, smokeTest)
{
    TestCopy tst1{ "aaaaB" };
    EXPECT_EQ(std::string("aaaaB"), tst1.get());
    TestCopy tst2(tst1);

    EXPECT_EQ(std::string("aaaaB"), tst1.get());
    EXPECT_EQ(std::string("aaaaB"), tst2.get());

    TestCopy tst3{ "Chuvava" };
    EXPECT_EQ(std::string("aaaaB"), tst1.get());
    EXPECT_EQ(std::string("aaaaB"), tst2.get());
    EXPECT_EQ(std::string("Chuvava"), tst3.get());

    tst2 = tst3;

    EXPECT_EQ(std::string("aaaaB"), tst1.get());
    EXPECT_EQ(std::string("Chuvava"), tst2.get());
    EXPECT_EQ(std::string("Chuvava"), tst3.get());
}

TEST(RustString, Copy)
{
    Foo foo(1, "AAAA");
    auto s = foo.ret_string();
    ASSERT_EQ("AAAA", s.to_std_string());
    RustString s2(s);
    ASSERT_EQ("AAAA", s.to_std_string());
    ASSERT_EQ("AAAA", s2.to_std_string());
    RustString s3;
    s3 = s2;
    s3 = s;
    ASSERT_EQ("AAAA", s.to_std_string());
    ASSERT_EQ("AAAA", s2.to_std_string());
    ASSERT_EQ("AAAA", s3.to_std_string());
}

TEST(GetSetStrTest, smokeTest)
{
    GetSetStrTest test;
    EXPECT_EQ("", test.get_str());
    test.set_str(test.get_str());
    test.set_str({ "hello" });
    EXPECT_EQ("hello", test.get_str());
}

TEST(TestWorkWithReprC, smokeTest)
{
    auto v = TestWorkWithReprC::inc_vec2({ 1.75f, 1e5f });

    EXPECT_NEAR(1.75f + 1.1f, v.x, std::numeric_limits<float>::epsilon());
    EXPECT_NEAR(1e5f + 1.f, v.y, std::numeric_limits<float>::epsilon());
}

TEST(TestFnInline, smokeTest)
{
    for (auto x : { 1, 100, -1, 1000 }) {
        auto s = TestFnInline::int_to_str(x);
        EXPECT_EQ(std::to_string(x), s.to_std_string());
    }
}

TEST(TestFuture, smokeTest)
{
    {
        auto future = TestFuture::call_fn();
        future.wait();
        Foo foo = future.get();
        EXPECT_EQ(-1, foo.f(0, 0));
        EXPECT_EQ("from callback", foo.getName());
    }

    {
        auto future = TestFuture::call_fn2(true);
        future.wait();
        auto res = future.get();
#if defined(HAS_STDCXX_17) && !defined(NO_HAVE_STD17_VARIANT)
        ASSERT_TRUE(nullptr != std::get_if<RustForeignVecFoo>(&res));
        auto foo_vec = std::get<RustForeignVecFoo>(std::move(res));
        ASSERT_EQ(1u, foo_vec.size());
        auto foo = foo_vec.remove(0);

        EXPECT_EQ(-1, foo.f(0, 0));
        EXPECT_EQ("from callback", foo.getName());
#endif
#ifdef USE_BOOST
        ASSERT_TRUE(nullptr != boost::get<RustForeignVecFoo>(&res));
        auto foo_vec = boost::get<RustForeignVecFoo>(std::move(res));
        ASSERT_EQ(1u, foo_vec.size());
        auto foo = foo_vec.remove(0);

        EXPECT_EQ(-1, foo.f(0, 0));
        EXPECT_EQ("from callback", foo.getName());
#endif
    }

    {
        auto future = TestFuture::call_fn2(false);
        future.wait();
        auto res = future.get();
#if defined(HAS_STDCXX_17) && !defined(NO_HAVE_STD17_VARIANT)
        ASSERT_TRUE(nullptr != std::get_if<RustString>(&res));
        auto msg = std::get<RustString>(std::move(res));
        EXPECT_EQ("Err", msg.to_std_string());
#endif
#ifdef USE_BOOST
        ASSERT_TRUE(nullptr != boost::get<RustString>(&res));
        auto msg = boost::get<RustString>(std::move(res));
        EXPECT_EQ("Err", msg.to_std_string());
#endif
    }
}

namespace {
struct State {
    std::mutex lock;
    bool called = false;
    int32_t x;
    RustString s;
    State() = default;
};

class MyThreadSafeObserver final : public ThreadSafeObserver {
public:
    MyThreadSafeObserver(std::shared_ptr<State> state)
        : state_(std::move(state))
    {
        assert(state_ != nullptr);
        assert(!state_->called);
    }
    void onStateChanged(int32_t x, RustString s) noexcept override
    {
        std::lock_guard<std::mutex> guard(state_->lock);
        state_->called = true;
        state_->x = x;
        state_->s = std::move(s);
    }

private:
    std::shared_ptr<State> state_;
};
} // namespace

TEST(TestMultiThreadCallback, smokeTest)
{
    auto state = std::make_shared<State>();
    EXPECT_FALSE(state->called);

    TestMultiThreadCallback::f(
        std::unique_ptr<ThreadSafeObserver>{ new MyThreadSafeObserver(state) });
    std::this_thread::sleep_for(std::chrono::seconds(4));
    EXPECT_TRUE(state->called);
    EXPECT_EQ(42, state->x);
    EXPECT_EQ("15", state->s.to_std_string());
}

TEST(SmartPtrCopy, smokeTest)
{
    Session session{ "Session" };
    EXPECT_EQ("Session", session.name());
    Session session2{ session };
    EXPECT_EQ("Session", session.name());
    EXPECT_EQ("Session", session2.name());

    EXPECT_EQ(static_cast<const SessionOpaque *>(session2),
              static_cast<const SessionOpaque *>(session));
    Session session3{ "AAAA" };
    EXPECT_EQ("Session", session.name());
    EXPECT_EQ("Session", session2.name());
    EXPECT_EQ("AAAA", session3.name());
    session2 = session3;
    EXPECT_EQ("Session", session.name());
    EXPECT_EQ("AAAA", session2.name());
    EXPECT_EQ("AAAA", session3.name());
    EXPECT_NE(static_cast<const SessionOpaque *>(session2),
              static_cast<const SessionOpaque *>(session));
    EXPECT_EQ(static_cast<const SessionOpaque *>(session2),
              static_cast<const SessionOpaque *>(session3));

    auto s = Session::test_copy(session);
    EXPECT_EQ("Session", session.name());
    EXPECT_EQ("Session", s.to_std_string());
}

int main(int argc, char *argv[])
{
    ::testing::InitGoogleTest(&argc, argv);
    ::testing::GTEST_FLAG(throw_on_failure) = true;
    return RUN_ALL_TESTS();
}
