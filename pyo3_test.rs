#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3_decimal_macro::make_decimal;
use gdb_breakpoint::breakpoint;
use once_cell::sync::Lazy;
use pyo3::class::basic::CompareOp;
use pyo3::conversion::AsPyPointer;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyFloat, PyInt, PyList, PyString, PyTuple};
use pyo3::PyNativeType;
use pyo3::{exceptions, PyResult};
use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use rust_decimal::prelude::Decimal as RustDecimal;
use rust_decimal::prelude::ToPrimitive;
use std::fmt;
use std::ptr;
use fxhash::hash;
use std::hash::Hash;
struct VersionInfo {
    rustc: String,
    os: String,
    family: String,
    env: String,
    endian: String,
    arch: String,
    target: String,
    rust_decimal: String,
    pyo3: String,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::hash::Hash for VersionInfo {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {
            VersionInfo {
                rustc: ref __self_0_0,
                os: ref __self_0_1,
                family: ref __self_0_2,
                env: ref __self_0_3,
                endian: ref __self_0_4,
                arch: ref __self_0_5,
                target: ref __self_0_6,
                rust_decimal: ref __self_0_7,
                pyo3: ref __self_0_8,
            } => {
                ::core::hash::Hash::hash(&(*__self_0_0), state);
                ::core::hash::Hash::hash(&(*__self_0_1), state);
                ::core::hash::Hash::hash(&(*__self_0_2), state);
                ::core::hash::Hash::hash(&(*__self_0_3), state);
                ::core::hash::Hash::hash(&(*__self_0_4), state);
                ::core::hash::Hash::hash(&(*__self_0_5), state);
                ::core::hash::Hash::hash(&(*__self_0_6), state);
                ::core::hash::Hash::hash(&(*__self_0_7), state);
                ::core::hash::Hash::hash(&(*__self_0_8), state)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for VersionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            VersionInfo {
                rustc: ref __self_0_0,
                os: ref __self_0_1,
                family: ref __self_0_2,
                env: ref __self_0_3,
                endian: ref __self_0_4,
                arch: ref __self_0_5,
                target: ref __self_0_6,
                rust_decimal: ref __self_0_7,
                pyo3: ref __self_0_8,
            } => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f, "VersionInfo");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "rustc", &&(*__self_0_0));
                let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, "os", &&(*__self_0_1));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "family", &&(*__self_0_2));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "env", &&(*__self_0_3));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "endian", &&(*__self_0_4));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "arch", &&(*__self_0_5));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "target", &&(*__self_0_6));
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "rust_decimal",
                    &&(*__self_0_7),
                );
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "pyo3", &&(*__self_0_8));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for VersionInfo {
    #[inline]
    fn clone(&self) -> VersionInfo {
        match *self {
            VersionInfo {
                rustc: ref __self_0_0,
                os: ref __self_0_1,
                family: ref __self_0_2,
                env: ref __self_0_3,
                endian: ref __self_0_4,
                arch: ref __self_0_5,
                target: ref __self_0_6,
                rust_decimal: ref __self_0_7,
                pyo3: ref __self_0_8,
            } => VersionInfo {
                rustc: ::core::clone::Clone::clone(&(*__self_0_0)),
                os: ::core::clone::Clone::clone(&(*__self_0_1)),
                family: ::core::clone::Clone::clone(&(*__self_0_2)),
                env: ::core::clone::Clone::clone(&(*__self_0_3)),
                endian: ::core::clone::Clone::clone(&(*__self_0_4)),
                arch: ::core::clone::Clone::clone(&(*__self_0_5)),
                target: ::core::clone::Clone::clone(&(*__self_0_6)),
                rust_decimal: ::core::clone::Clone::clone(&(*__self_0_7)),
                pyo3: ::core::clone::Clone::clone(&(*__self_0_8)),
            },
        }
    }
}
pub mod built_info {
    ///The Continuous Integration platform detected during compilation.
    #[allow(dead_code)]
    pub const CI_PLATFORM: Option<&str> = None;
    ///The full version.
    #[allow(dead_code)]
    pub const PKG_VERSION: &str = r"0.1.0";
    ///The major version.
    #[allow(dead_code)]
    pub const PKG_VERSION_MAJOR: &str = r"0";
    ///The minor version.
    #[allow(dead_code)]
    pub const PKG_VERSION_MINOR: &str = r"1";
    ///The patch version.
    #[allow(dead_code)]
    pub const PKG_VERSION_PATCH: &str = r"0";
    ///The pre-release version.
    #[allow(dead_code)]
    pub const PKG_VERSION_PRE: &str = r"";
    ///A colon-separated list of authors.
    #[allow(dead_code)]
    pub const PKG_AUTHORS: &str = r"fx kirin <fx.kirin@gmail.com>";
    ///The name of the package.
    #[allow(dead_code)]
    pub const PKG_NAME: &str = r"pyo3-decimal";
    ///The description.
    #[allow(dead_code)]
    pub const PKG_DESCRIPTION: &str = r"";
    ///The homepage.
    #[allow(dead_code)]
    pub const PKG_HOMEPAGE: &str = r"";
    ///The license.
    #[allow(dead_code)]
    pub const PKG_LICENSE: &str = r"";
    ///The source repository as advertised in Cargo.toml.
    #[allow(dead_code)]
    pub const PKG_REPOSITORY: &str = r"";
    ///The target triple that was being compiled for.
    #[allow(dead_code)]
    pub const TARGET: &str = r"x86_64-unknown-linux-gnu";
    ///The host triple of the rust compiler.
    #[allow(dead_code)]
    pub const HOST: &str = r"x86_64-unknown-linux-gnu";
    ///`release` for release builds, `debug` for other builds.
    #[allow(dead_code)]
    pub const PROFILE: &str = r"debug";
    ///The compiler that cargo resolved to use.
    #[allow(dead_code)]
    pub const RUSTC: &str = r"rustc";
    ///The documentation generator that cargo resolved to use.
    #[allow(dead_code)]
    pub const RUSTDOC: &str = r"rustdoc";
    ///Value of OPT_LEVEL for the profile used during compilation.
    #[allow(dead_code)]
    pub const OPT_LEVEL: &str = r"0";
    ///The parallelism that was specified during compilation.
    #[allow(dead_code)]
    pub const NUM_JOBS: u32 = 8;
    ///Value of DEBUG for the profile used during compilation.
    #[allow(dead_code)]
    pub const DEBUG: bool = true;
    ///The features that were enabled during compilation.
    #[allow(dead_code)]
    pub const FEATURES: [&str; 0] = [];
    ///The features as a comma-separated string.
    #[allow(dead_code)]
    pub const FEATURES_STR: &str = r"";
    ///The output of `rustc -V`
    #[allow(dead_code)]
    pub const RUSTC_VERSION: &str = r"rustc 1.59.0-nightly (51e8031e1 2021-12-25)";
    ///The output of `rustdoc -V`
    #[allow(dead_code)]
    pub const RUSTDOC_VERSION: &str = r"rustdoc 1.59.0-nightly (51e8031e1 2021-12-25)";
    ///An array of effective dependencies as documented by `Cargo.lock`.
    #[allow(dead_code)]
    pub const DEPENDENCIES: [(&str, &str); 56] = [
        ("arrayvec", "0.7.2"),
        ("autocfg", "1.1.0"),
        ("bitflags", "1.3.2"),
        ("built", "0.5.1"),
        ("byteorder", "1.4.3"),
        ("cargo-lock", "7.0.1"),
        ("cc", "1.0.73"),
        ("cfg-if", "0.1.10"),
        ("cfg-if", "1.0.0"),
        ("form_urlencoded", "1.0.1"),
        ("fxhash", "0.2.1"),
        ("gdb_breakpoint", "0.1.6"),
        ("idna", "0.2.3"),
        ("indoc", "1.0.4"),
        ("libc", "0.2.124"),
        ("lock_api", "0.4.7"),
        ("matches", "0.1.9"),
        ("nix", "0.13.1"),
        ("num-traits", "0.2.14"),
        ("once_cell", "1.10.0"),
        ("parking_lot", "0.12.0"),
        ("parking_lot_core", "0.9.2"),
        ("percent-encoding", "2.1.0"),
        ("proc-macro2", "1.0.37"),
        ("pyo3", "0.16.4"),
        ("pyo3-build-config", "0.16.4"),
        ("pyo3-decimal", "0.1.0"),
        ("pyo3-ffi", "0.16.4"),
        ("pyo3-macros", "0.16.4"),
        ("pyo3-macros-backend", "0.16.4"),
        ("pyo3_decimal_macro", "0.1.0"),
        ("quote", "1.0.18"),
        ("redox_syscall", "0.2.13"),
        ("rust_decimal", "1.23.1"),
        ("scopeguard", "1.1.0"),
        ("semver", "1.0.7"),
        ("serde", "1.0.136"),
        ("serde_derive", "1.0.136"),
        ("smallvec", "1.8.0"),
        ("syn", "1.0.91"),
        ("target-lexicon", "0.12.3"),
        ("tinyvec", "1.5.1"),
        ("tinyvec_macros", "0.1.0"),
        ("toml", "0.5.9"),
        ("unicode-bidi", "0.3.7"),
        ("unicode-normalization", "0.1.19"),
        ("unicode-xid", "0.2.2"),
        ("unindent", "0.1.8"),
        ("url", "2.2.2"),
        ("void", "1.0.2"),
        ("windows-sys", "0.34.0"),
        ("windows_aarch64_msvc", "0.34.0"),
        ("windows_i686_gnu", "0.34.0"),
        ("windows_i686_msvc", "0.34.0"),
        ("windows_x86_64_gnu", "0.34.0"),
        ("windows_x86_64_msvc", "0.34.0"),
    ];
    ///The effective dependencies as a comma-separated string.
    #[allow(dead_code)]
    pub const DEPENDENCIES_STR: &str = r"arrayvec 0.7.2, autocfg 1.1.0, bitflags 1.3.2, built 0.5.1, byteorder 1.4.3, cargo-lock 7.0.1, cc 1.0.73, cfg-if 0.1.10, cfg-if 1.0.0, form_urlencoded 1.0.1, fxhash 0.2.1, gdb_breakpoint 0.1.6, idna 0.2.3, indoc 1.0.4, libc 0.2.124, lock_api 0.4.7, matches 0.1.9, nix 0.13.1, num-traits 0.2.14, once_cell 1.10.0, parking_lot 0.12.0, parking_lot_core 0.9.2, percent-encoding 2.1.0, proc-macro2 1.0.37, pyo3 0.16.4, pyo3-build-config 0.16.4, pyo3-decimal 0.1.0, pyo3-ffi 0.16.4, pyo3-macros 0.16.4, pyo3-macros-backend 0.16.4, pyo3_decimal_macro 0.1.0, quote 1.0.18, redox_syscall 0.2.13, rust_decimal 1.23.1, scopeguard 1.1.0, semver 1.0.7, serde 1.0.136, serde_derive 1.0.136, smallvec 1.8.0, syn 1.0.91, target-lexicon 0.12.3, tinyvec 1.5.1, tinyvec_macros 0.1.0, toml 0.5.9, unicode-bidi 0.3.7, unicode-normalization 0.1.19, unicode-xid 0.2.2, unindent 0.1.8, url 2.2.2, void 1.0.2, windows-sys 0.34.0, windows_aarch64_msvc 0.34.0, windows_i686_gnu 0.34.0, windows_i686_msvc 0.34.0, windows_x86_64_gnu 0.34.0, windows_x86_64_msvc 0.34.0";
    ///The target architecture, given by `CARGO_CFG_TARGET_ARCH`.
    #[allow(dead_code)]
    pub const CFG_TARGET_ARCH: &str = r"x86_64";
    ///The endianness, given by `CARGO_CFG_TARGET_ENDIAN`.
    #[allow(dead_code)]
    pub const CFG_ENDIAN: &str = r"little";
    ///The toolchain-environment, given by `CARGO_CFG_TARGET_ENV`.
    #[allow(dead_code)]
    pub const CFG_ENV: &str = r"gnu";
    ///The OS-family, given by `CARGO_CFG_TARGET_FAMILY`.
    #[allow(dead_code)]
    pub const CFG_FAMILY: &str = r"unix";
    ///The operating system, given by `CARGO_CFG_TARGET_OS`.
    #[allow(dead_code)]
    pub const CFG_OS: &str = r"linux";
    ///The pointer width, given by `CARGO_CFG_TARGET_POINTER_WIDTH`.
    #[allow(dead_code)]
    pub const CFG_POINTER_WIDTH: &str = r"64";
}
fn make_decimal_version_info() -> VersionInfo {
    let rustc = built_info::RUSTC_VERSION.to_string();
    let os = built_info::CFG_OS.to_string();
    let family = built_info::CFG_FAMILY.to_string();
    let env = built_info::CFG_ENV.to_string();
    let endian = built_info::CFG_ENDIAN.to_string();
    let arch = built_info::CFG_TARGET_ARCH.to_string();
    let target = built_info::TARGET.to_string();
    let rust_decimal = built_info::DEPENDENCIES
        .iter()
        .filter(|&dep| dep.0 == "rust_decimal")
        .map(|&dep| dep.1)
        .next()
        .expect("dependency of rust_decimal was not found")
        .to_string();
    let pyo3 = built_info::DEPENDENCIES
        .iter()
        .filter(|&dep| dep.0 == "pyo3")
        .map(|&dep| dep.1)
        .next()
        .expect("dependency of rust_decimal was not found")
        .to_string();
    let ver_info = VersionInfo {
        rustc,
        os,
        family,
        env,
        endian,
        arch,
        target,
        rust_decimal,
        pyo3,
    };
    ver_info
}
fn make_decimal_version_hash() -> usize {
    let ver_info = make_decimal_version_info();
    hash(&ver_info)
}
static DECIMAL_VERSION_HASH: Lazy<usize> = Lazy::new(|| make_decimal_version_hash());
static DECIMAL_VERSION_INFO: Lazy<VersionInfo> = Lazy::new(|| make_decimal_version_info());
fn get_decimal_version_info<'p>(input: Decimal, _py: Python<'p>) -> PyResult<String> {
    Ok({
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &[""],
            &match (&*DECIMAL_VERSION_INFO,) {
                _args => [::core::fmt::ArgumentV1::new(
                    _args.0,
                    ::core::fmt::Debug::fmt,
                )],
            },
        ));
        res
    }
    .to_string())
}
unsafe extern "C" fn __pyfunction_get_decimal_version_info(
    _slf: *mut ::pyo3::ffi::PyObject,
    _args: *const *mut ::pyo3::ffi::PyObject,
    _nargs: ::pyo3::ffi::Py_ssize_t,
    _kwnames: *mut ::pyo3::ffi::PyObject,
) -> *mut ::pyo3::ffi::PyObject {
    use :: pyo3 as _pyo3;
    let gil = _pyo3::GILPool::new();
    let _py = gil.python();
    _pyo3::callback::panic_result_into_callback_output(
        _py,
        ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
            const DESCRIPTION: _pyo3::impl_::extract_argument::FunctionDescription =
                _pyo3::impl_::extract_argument::FunctionDescription {
                    cls_name: ::std::option::Option::None,
                    func_name: "get_decimal_version_info",
                    positional_parameter_names: &["input"],
                    positional_only_parameters: 0usize,
                    required_positional_parameters: 1usize,
                    keyword_only_parameters: &[],
                };
            let mut output = [::std::option::Option::None; 1usize];
            let (_args , _kwargs) = DESCRIPTION . extract_arguments_fastcall :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _nargs , _kwnames , & mut output) ? ;
            let arg0 = _pyo3::impl_::extract_argument::extract_argument(
                _pyo3::impl_::extract_argument::unwrap_required_argument(output[0usize]),
                "input",
            )?;
            let arg1 = _py;
            _pyo3::callback::convert(_py, get_decimal_version_info(arg0, arg1))
        }),
    )
}
#[doc(hidden)]
mod get_decimal_version_info {
    use :: pyo3 as _pyo3;
    pub(crate) struct PyO3Def;
    pub use _pyo3::impl_::pyfunction::wrap_pyfunction as wrap;
    pub const DEF: _pyo3::PyMethodDef = <PyO3Def as _pyo3::impl_::pyfunction::PyFunctionDef>::DEF;
}
const _: () = {
    use :: pyo3 as _pyo3;
    impl _pyo3::impl_::pyfunction::PyFunctionDef for get_decimal_version_info::PyO3Def {
        const DEF: _pyo3::PyMethodDef =
            _pyo3::impl_::pymethods::PyMethodDef::fastcall_cfunction_with_keywords(
                "get_decimal_version_info\u{0}",
                _pyo3::impl_::pymethods::PyCFunctionFastWithKeywords(
                    __pyfunction_get_decimal_version_info,
                ),
                "\u{0}",
            );
    }
};
pub struct Decimal(RustDecimal, usize);
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Decimal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Decimal(ref __self_0_0, ref __self_0_1) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Decimal");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_1));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
const _: () = {
    use :: pyo3 as _pyo3;
    unsafe impl _pyo3::type_object::PyTypeInfo for Decimal {
        type AsRefTarget = _pyo3::PyCell<Self>;
        const NAME: &'static str = "Decimal";
        const MODULE: ::std::option::Option<&'static str> =
            ::core::option::Option::Some("pyo3_decimal");
        #[inline]
        fn type_object_raw(py: _pyo3::Python<'_>) -> *mut _pyo3::ffi::PyTypeObject {
            use _pyo3::type_object::LazyStaticType;
            static TYPE_OBJECT: LazyStaticType = LazyStaticType::new();
            TYPE_OBJECT.get_or_init::<Self>(py)
        }
    }
    impl _pyo3::PyClass for Decimal {
        type Dict = _pyo3::impl_::pyclass::PyClassDummySlot;
        type WeakRef = _pyo3::impl_::pyclass::PyClassDummySlot;
        type BaseNativeType = _pyo3::PyAny;
    }
    impl<'a> _pyo3::derive_utils::ExtractExt<'a> for &'a Decimal {
        type Target = _pyo3::PyRef<'a, Decimal>;
    }
    impl<'a> _pyo3::derive_utils::ExtractExt<'a> for &'a mut Decimal {
        type Target = _pyo3::PyRefMut<'a, Decimal>;
    }
    impl _pyo3::IntoPy<_pyo3::PyObject> for Decimal {
        fn into_py(self, py: _pyo3::Python) -> _pyo3::PyObject {
            _pyo3::IntoPy::into_py(_pyo3::Py::new(py, self).unwrap(), py)
        }
    }
    impl _pyo3::impl_::pyclass::PyClassImpl for Decimal {
        const DOC: &'static str = "\u{0}";
        const IS_BASETYPE: bool = false;
        const IS_SUBCLASS: bool = false;
        const IS_MAPPING: bool = false;
        type Layout = _pyo3::PyCell<Self>;
        type BaseType = _pyo3::PyAny;
        type ThreadChecker = _pyo3::impl_::pyclass::ThreadCheckerStub<Decimal>;
        fn for_all_items(
            visitor: &mut dyn ::std::ops::FnMut(&_pyo3::impl_::pyclass::PyClassItems),
        ) {
            use _pyo3::impl_::pyclass::*;
            let collector = PyClassImplCollector::<Self>::new();
            static INTRINSIC_ITEMS: PyClassItems = PyClassItems {
                methods: &[],
                slots: &[],
            };
            visitor(&INTRINSIC_ITEMS);
            visitor(collector.py_methods());
            visitor(collector.object_protocol_items());
            visitor(collector.number_protocol_items());
            visitor(collector.iter_protocol_items());
            visitor(collector.gc_protocol_items());
            visitor(collector.descr_protocol_items());
            visitor(collector.mapping_protocol_items());
            visitor(collector.sequence_protocol_items());
            visitor(collector.async_protocol_items());
            visitor(collector.buffer_protocol_items());
        }
    }
};
pub struct Wrapper(PyCell<Decimal>);
unsafe impl PyNativeType for Wrapper {}
impl<'source> FromPyObject<'source> for Decimal {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        let py_int = ob.cast_as::<PyInt>();
        if let Ok(content) = py_int {
            let num: i128 = content.extract().unwrap();
            return Ok(Decimal(
                RustDecimal::from_i128_with_scale(num, 0),
                *DECIMAL_VERSION_HASH,
            ));
        }
        let _cell = unsafe { Wrapper::unchecked_downcast(ob) };
        let unwrapped: &Decimal = &_cell.0.try_borrow().unwrap();
        if *DECIMAL_VERSION_HASH != unwrapped.1 {
            return Err(exceptions::PyValueError::new_err({
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Input error. VERSION HASH is not the same. "],
                    &match (&*DECIMAL_VERSION_INFO,) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Debug::fmt,
                        )],
                    },
                ));
                res
            }));
        }
        Ok(Decimal(unwrapped.0, *DECIMAL_VERSION_HASH))
    }
}
impl<'source> FromPyObject<'source> for &'source Decimal {
    fn extract(ob: &'source PyAny) -> PyResult<&Self> {
        let py_int = ob.cast_as::<PyInt>();
        if let Ok(content) = py_int {
            let num: i128 = content.extract().unwrap();
            return Ok(Decimal(
                RustDecimal::from_i128_with_scale(num, 0),
                *DECIMAL_VERSION_HASH,
            ));
        }
        let _cell = unsafe { Wrapper::unchecked_downcast(ob) };
        let unwrapped: &Decimal = &_cell.0.try_borrow().unwrap();
        if *DECIMAL_VERSION_HASH != unwrapped.1 {
            return Err(exceptions::PyValueError::new_err({
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Input error. VERSION HASH is not the same. "],
                    &match (&*DECIMAL_VERSION_INFO,) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Debug::fmt,
                        )],
                    },
                ));
                res
            }));
        }
        Ok(unwrapped)
    }
}
impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(::core::fmt::Arguments::new_v1(
            &[""],
            &match (&self.0,) {
                _args => [::core::fmt::ArgumentV1::new(
                    _args.0,
                    ::core::fmt::Display::fmt,
                )],
            },
        ))
    }
}
impl Decimal {
    pub fn new<'p>(arg1: PyObject, arg2: Option<PyObject>, py: Python<'p>) -> PyResult<Decimal> {
        let py_string = arg1.cast_as::<PyString>(py);
        if let Ok(content) = py_string {
            let rust_str: &str = &content.to_str().unwrap();
            let result = RustDecimal::from_str(rust_str);
            if arg2.is_some() {
                return Err(exceptions::PyValueError::new_err({
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["arg1 is String but arg2 was supplied value. "],
                        &match (&arg2,) {
                            _args => [::core::fmt::ArgumentV1::new(
                                _args.0,
                                ::core::fmt::Debug::fmt,
                            )],
                        },
                    ));
                    res
                }));
            }
            return match result {
                Ok(v) => Ok(Self(v, *DECIMAL_VERSION_HASH)),
                Err(_) => Err(exceptions::PyValueError::new_err({
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["arg1 is wrong value. "],
                        &match (&rust_str,) {
                            _args => [::core::fmt::ArgumentV1::new(
                                _args.0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ));
                    res
                })),
            };
        }
        let py_float = arg1.cast_as::<PyFloat>(py);
        if let Ok(content) = py_float {
            let num: f64 = content.extract().unwrap();
            if arg2.is_some() {
                return Err(exceptions::PyValueError::new_err({
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["arg1 is Float but arg2 was supplied value. "],
                        &match (&arg2,) {
                            _args => [::core::fmt::ArgumentV1::new(
                                _args.0,
                                ::core::fmt::Debug::fmt,
                            )],
                        },
                    ));
                    res
                }));
            }
            return Ok(Self(
                RustDecimal::from_f64_retain(num).expect("Failed to load from float value"),
                *DECIMAL_VERSION_HASH,
            ));
        }
        let py_int = arg1.cast_as::<PyInt>(py);
        let num: i128 = if let Ok(content) = py_int {
            content.extract().unwrap()
        } else {
            return Err(exceptions::PyValueError::new_err({
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["arg1 is wrong value. "],
                    &match (&arg1,) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Debug::fmt,
                        )],
                    },
                ));
                res
            }));
        };
        let scale: u32 = if let Some(arg2) = arg2 {
            let py_int = arg2.cast_as::<PyInt>(py);
            if let Ok(content) = py_int {
                content.extract().unwrap()
            } else {
                return Err(exceptions::PyValueError::new_err({
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["arg2 is wrong value. "],
                        &match (&arg2,) {
                            _args => [::core::fmt::ArgumentV1::new(
                                _args.0,
                                ::core::fmt::Debug::fmt,
                            )],
                        },
                    ));
                    res
                }));
            }
        } else {
            0
        };
        Ok(Self(
            RustDecimal::from_i128_with_scale(num, scale),
            *DECIMAL_VERSION_HASH,
        ))
    }
    pub const fn scale(&self) -> u32 {
        self.0.scale()
    }
    pub const fn mantissa(&self) -> i128 {
        self.0.mantissa()
    }
    pub const fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
    pub fn set_sign_positive(&mut self, positive: bool) {
        self.0.set_sign_positive(positive)
    }
    pub fn set_sign_negative(&mut self, negative: bool) {
        self.0.set_sign_negative(negative)
    }
    pub fn set_scale(&mut self, scale: u32) -> PyResult<()> {
        let result = self.0.set_scale(scale);
        match result {
            Ok(v) => Ok(v),
            Err(_) => Err(exceptions::PyRuntimeError::new_err("set_scale Error")),
        }
    }
    pub fn rescale(&mut self, scale: u32) {
        self.0.rescale(scale)
    }
    pub const fn is_sign_negative(&self) -> bool {
        self.0.is_sign_negative()
    }
    pub const fn is_sign_positive(&self) -> bool {
        self.0.is_sign_positive()
    }
    pub fn trunc(&self) -> Decimal {
        self.0.trunc().into()
    }
    pub fn fract(&self) -> Decimal {
        self.0.fract().into()
    }
    pub fn abs(&self) -> Decimal {
        self.0.abs().into()
    }
    pub fn floor(&self) -> Decimal {
        self.0.floor().into()
    }
    pub fn ceil(&self) -> Decimal {
        self.0.ceil().into()
    }
    pub fn max(&self, other: Decimal) -> Decimal {
        self.0.max(other.0).into()
    }
    pub fn min(&self, other: Decimal) -> Decimal {
        self.0.min(other.0).into()
    }
    pub fn normalize(&self) -> Decimal {
        self.0.normalize().into()
    }
    pub fn normalize_assign(&mut self) {
        self.0.normalize_assign()
    }
    pub fn round(&self) -> Decimal {
        self.0.round().into()
    }
    pub fn round_dp(&self, dp: u32) -> Decimal {
        self.0.round_dp(dp).into()
    }
    pub fn round_sf(&self, digits: u32) -> Option<Decimal> {
        let decimal = self.0.round_sf(digits);
        if decimal.is_some() {
            Some(decimal.unwrap().into())
        } else {
            None
        }
    }
    pub fn to_int(&self) -> i64 {
        self.0.to_i64().unwrap()
    }
    pub fn to_float(&self) -> f64 {
        self.0.to_f64().unwrap()
    }
    fn __add__(&self, other: &Decimal) -> PyResult<Decimal> {
        Ok((self.0 + other.0).into())
    }
    fn __sub__(&self, other: &Decimal) -> PyResult<Decimal> {
        Ok((self.0 - other.0).into())
    }
    fn __mul__(&self, other: &Decimal) -> PyResult<Decimal> {
        Ok((self.0 * other.0).into())
    }
    fn __truediv__(&self, other: &Decimal) -> PyResult<Decimal> {
        Ok((self.0 / other.0).into())
    }
    fn __floordiv__(&self, other: &Decimal) -> PyResult<Decimal> {
        Ok((self.0 / other.0).into())
    }
    fn __neg__(&self) -> PyResult<Decimal> {
        Ok((-self.0).into())
    }
    fn __richcmp__(&self, other: Decimal, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }
    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok({
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["Decimal(", ")"],
                &match (&self.to_string(),) {
                    _args => [::core::fmt::ArgumentV1::new(
                        _args.0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ));
            res
        })
    }
}
const _: () = {
    use :: pyo3 as _pyo3;
    impl _pyo3::impl_::pyclass::PyClass__add__SlotFragment<Decimal>
        for _pyo3::impl_::pyclass::PyClassImplCollector<Decimal>
    {
        #[inline]
        unsafe fn __add__(
            self,
            _py: _pyo3::Python,
            _raw_slf: *mut _pyo3::ffi::PyObject,
            arg0: *mut _pyo3::ffi::PyObject,
        ) -> _pyo3::PyResult<*mut _pyo3::ffi::PyObject> {
            let _slf = _raw_slf;
            let _cell = match _py
                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                .downcast::<_pyo3::PyCell<Decimal>>()
            {
                ::std::result::Result::Ok(cell) => cell,
                ::std::result::Result::Err(_) => {
                    return _pyo3::callback::convert(_py, _py.NotImplemented())
                }
            };
            let _ref = _cell.try_borrow()?;
            let _slf: &Decimal = &*_ref;
            let arg0: <&Decimal as _pyo3::derive_utils::ExtractExt<'_>>::Target =
                match _py.from_borrowed_ptr::<_pyo3::PyAny>(arg0).extract() {
                    ::std::result::Result::Ok(value) => value,
                    ::std::result::Result::Err(_) => {
                        return _pyo3::callback::convert(_py, _py.NotImplemented());
                    }
                };
            let arg0 = &*arg0;
            _pyo3::callback::convert(_py, Decimal::__add__(_slf, arg0))
        }
    }
    impl _pyo3::impl_::pyclass::PyClass__sub__SlotFragment<Decimal>
        for _pyo3::impl_::pyclass::PyClassImplCollector<Decimal>
    {
        #[inline]
        unsafe fn __sub__(
            self,
            _py: _pyo3::Python,
            _raw_slf: *mut _pyo3::ffi::PyObject,
            arg0: *mut _pyo3::ffi::PyObject,
        ) -> _pyo3::PyResult<*mut _pyo3::ffi::PyObject> {
            let _slf = _raw_slf;
            let _cell = match _py
                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                .downcast::<_pyo3::PyCell<Decimal>>()
            {
                ::std::result::Result::Ok(cell) => cell,
                ::std::result::Result::Err(_) => {
                    return _pyo3::callback::convert(_py, _py.NotImplemented())
                }
            };
            let _ref = _cell.try_borrow()?;
            let _slf: &Decimal = &*_ref;
            let arg0: <&Decimal as _pyo3::derive_utils::ExtractExt<'_>>::Target =
                match _py.from_borrowed_ptr::<_pyo3::PyAny>(arg0).extract() {
                    ::std::result::Result::Ok(value) => value,
                    ::std::result::Result::Err(_) => {
                        return _pyo3::callback::convert(_py, _py.NotImplemented());
                    }
                };
            let arg0 = &*arg0;
            _pyo3::callback::convert(_py, Decimal::__sub__(_slf, arg0))
        }
    }
    impl _pyo3::impl_::pyclass::PyClass__mul__SlotFragment<Decimal>
        for _pyo3::impl_::pyclass::PyClassImplCollector<Decimal>
    {
        #[inline]
        unsafe fn __mul__(
            self,
            _py: _pyo3::Python,
            _raw_slf: *mut _pyo3::ffi::PyObject,
            arg0: *mut _pyo3::ffi::PyObject,
        ) -> _pyo3::PyResult<*mut _pyo3::ffi::PyObject> {
            let _slf = _raw_slf;
            let _cell = match _py
                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                .downcast::<_pyo3::PyCell<Decimal>>()
            {
                ::std::result::Result::Ok(cell) => cell,
                ::std::result::Result::Err(_) => {
                    return _pyo3::callback::convert(_py, _py.NotImplemented())
                }
            };
            let _ref = _cell.try_borrow()?;
            let _slf: &Decimal = &*_ref;
            let arg0: <&Decimal as _pyo3::derive_utils::ExtractExt<'_>>::Target =
                match _py.from_borrowed_ptr::<_pyo3::PyAny>(arg0).extract() {
                    ::std::result::Result::Ok(value) => value,
                    ::std::result::Result::Err(_) => {
                        return _pyo3::callback::convert(_py, _py.NotImplemented());
                    }
                };
            let arg0 = &*arg0;
            _pyo3::callback::convert(_py, Decimal::__mul__(_slf, arg0))
        }
    }
    impl _pyo3::impl_::pyclass::PyClass__truediv__SlotFragment<Decimal>
        for _pyo3::impl_::pyclass::PyClassImplCollector<Decimal>
    {
        #[inline]
        unsafe fn __truediv__(
            self,
            _py: _pyo3::Python,
            _raw_slf: *mut _pyo3::ffi::PyObject,
            arg0: *mut _pyo3::ffi::PyObject,
        ) -> _pyo3::PyResult<*mut _pyo3::ffi::PyObject> {
            let _slf = _raw_slf;
            let _cell = match _py
                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                .downcast::<_pyo3::PyCell<Decimal>>()
            {
                ::std::result::Result::Ok(cell) => cell,
                ::std::result::Result::Err(_) => {
                    return _pyo3::callback::convert(_py, _py.NotImplemented())
                }
            };
            let _ref = _cell.try_borrow()?;
            let _slf: &Decimal = &*_ref;
            let arg0: <&Decimal as _pyo3::derive_utils::ExtractExt<'_>>::Target =
                match _py.from_borrowed_ptr::<_pyo3::PyAny>(arg0).extract() {
                    ::std::result::Result::Ok(value) => value,
                    ::std::result::Result::Err(_) => {
                        return _pyo3::callback::convert(_py, _py.NotImplemented());
                    }
                };
            let arg0 = &*arg0;
            _pyo3::callback::convert(_py, Decimal::__truediv__(_slf, arg0))
        }
    }
    impl _pyo3::impl_::pyclass::PyClass__floordiv__SlotFragment<Decimal>
        for _pyo3::impl_::pyclass::PyClassImplCollector<Decimal>
    {
        #[inline]
        unsafe fn __floordiv__(
            self,
            _py: _pyo3::Python,
            _raw_slf: *mut _pyo3::ffi::PyObject,
            arg0: *mut _pyo3::ffi::PyObject,
        ) -> _pyo3::PyResult<*mut _pyo3::ffi::PyObject> {
            let _slf = _raw_slf;
            let _cell = match _py
                .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                .downcast::<_pyo3::PyCell<Decimal>>()
            {
                ::std::result::Result::Ok(cell) => cell,
                ::std::result::Result::Err(_) => {
                    return _pyo3::callback::convert(_py, _py.NotImplemented())
                }
            };
            let _ref = _cell.try_borrow()?;
            let _slf: &Decimal = &*_ref;
            let arg0: <&Decimal as _pyo3::derive_utils::ExtractExt<'_>>::Target =
                match _py.from_borrowed_ptr::<_pyo3::PyAny>(arg0).extract() {
                    ::std::result::Result::Ok(value) => value,
                    ::std::result::Result::Err(_) => {
                        return _pyo3::callback::convert(_py, _py.NotImplemented());
                    }
                };
            let arg0 = &*arg0;
            _pyo3::callback::convert(_py, Decimal::__floordiv__(_slf, arg0))
        }
    }
    impl _pyo3::impl_::pyclass::PyMethods<Decimal>
        for _pyo3::impl_::pyclass::PyClassImplCollector<Decimal>
    {
        fn py_methods(self) -> &'static _pyo3::impl_::pyclass::PyClassItems {
            static ITEMS: _pyo3::impl_::pyclass::PyClassItems =
                _pyo3::impl_::pyclass::PyClassItems {
                    methods: &[
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::noargs(
                                "scale\u{0}",
                                _pyo3::impl_::pymethods::PyCFunction({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf: &Decimal = &*_ref;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::scale(_slf),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::noargs(
                                "mantissa\u{0}",
                                _pyo3::impl_::pymethods::PyCFunction({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf: &Decimal = &*_ref;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::mantissa(_slf),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::noargs(
                                "is_zero\u{0}",
                                _pyo3::impl_::pymethods::PyCFunction({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf: &Decimal = &*_ref;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::is_zero(_slf),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::fastcall_cfunction_with_keywords(
                                "set_sign_positive\u{0}",
                                _pyo3::impl_::pymethods::PyCFunctionFastWithKeywords({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *const *mut ::pyo3::ffi::PyObject,
                                        _nargs: ::pyo3::ffi::Py_ssize_t,
                                        _kwnames: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let mut _ref = _cell.try_borrow_mut()?;
                                                    let _slf: &mut Decimal = &mut *_ref;
                                                    const DESCRIPTION : _pyo3 :: impl_ :: extract_argument :: FunctionDescription = _pyo3 :: impl_ :: extract_argument :: FunctionDescription { cls_name : :: std :: option :: Option :: Some (< Decimal as _pyo3 :: type_object :: PyTypeInfo > :: NAME) , func_name : "set_sign_positive" , positional_parameter_names : & ["positive"] , positional_only_parameters : 0usize , required_positional_parameters : 1usize , keyword_only_parameters : & [] , } ;
                                                    let mut output =
                                                        [::std::option::Option::None; 1usize];
                                                    let (_args , _kwargs) = DESCRIPTION . extract_arguments_fastcall :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _nargs , _kwnames , & mut output) ? ;
                                                    let arg0 = _pyo3 :: impl_ :: extract_argument :: extract_argument (_pyo3 :: impl_ :: extract_argument :: unwrap_required_argument (output [0usize]) , "positive") ? ;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::set_sign_positive(_slf, arg0),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::fastcall_cfunction_with_keywords(
                                "set_sign_negative\u{0}",
                                _pyo3::impl_::pymethods::PyCFunctionFastWithKeywords({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *const *mut ::pyo3::ffi::PyObject,
                                        _nargs: ::pyo3::ffi::Py_ssize_t,
                                        _kwnames: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let mut _ref = _cell.try_borrow_mut()?;
                                                    let _slf: &mut Decimal = &mut *_ref;
                                                    const DESCRIPTION : _pyo3 :: impl_ :: extract_argument :: FunctionDescription = _pyo3 :: impl_ :: extract_argument :: FunctionDescription { cls_name : :: std :: option :: Option :: Some (< Decimal as _pyo3 :: type_object :: PyTypeInfo > :: NAME) , func_name : "set_sign_negative" , positional_parameter_names : & ["negative"] , positional_only_parameters : 0usize , required_positional_parameters : 1usize , keyword_only_parameters : & [] , } ;
                                                    let mut output =
                                                        [::std::option::Option::None; 1usize];
                                                    let (_args , _kwargs) = DESCRIPTION . extract_arguments_fastcall :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _nargs , _kwnames , & mut output) ? ;
                                                    let arg0 = _pyo3 :: impl_ :: extract_argument :: extract_argument (_pyo3 :: impl_ :: extract_argument :: unwrap_required_argument (output [0usize]) , "negative") ? ;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::set_sign_negative(_slf, arg0),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::fastcall_cfunction_with_keywords(
                                "set_scale\u{0}",
                                _pyo3::impl_::pymethods::PyCFunctionFastWithKeywords({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *const *mut ::pyo3::ffi::PyObject,
                                        _nargs: ::pyo3::ffi::Py_ssize_t,
                                        _kwnames: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let mut _ref = _cell.try_borrow_mut()?;
                                                    let _slf: &mut Decimal = &mut *_ref;
                                                    const DESCRIPTION : _pyo3 :: impl_ :: extract_argument :: FunctionDescription = _pyo3 :: impl_ :: extract_argument :: FunctionDescription { cls_name : :: std :: option :: Option :: Some (< Decimal as _pyo3 :: type_object :: PyTypeInfo > :: NAME) , func_name : "set_scale" , positional_parameter_names : & ["scale"] , positional_only_parameters : 0usize , required_positional_parameters : 1usize , keyword_only_parameters : & [] , } ;
                                                    let mut output =
                                                        [::std::option::Option::None; 1usize];
                                                    let (_args , _kwargs) = DESCRIPTION . extract_arguments_fastcall :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _nargs , _kwnames , & mut output) ? ;
                                                    let arg0 = _pyo3 :: impl_ :: extract_argument :: extract_argument (_pyo3 :: impl_ :: extract_argument :: unwrap_required_argument (output [0usize]) , "scale") ? ;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::set_scale(_slf, arg0),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::fastcall_cfunction_with_keywords(
                                "rescale\u{0}",
                                _pyo3::impl_::pymethods::PyCFunctionFastWithKeywords({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *const *mut ::pyo3::ffi::PyObject,
                                        _nargs: ::pyo3::ffi::Py_ssize_t,
                                        _kwnames: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let mut _ref = _cell.try_borrow_mut()?;
                                                    let _slf: &mut Decimal = &mut *_ref;
                                                    const DESCRIPTION : _pyo3 :: impl_ :: extract_argument :: FunctionDescription = _pyo3 :: impl_ :: extract_argument :: FunctionDescription { cls_name : :: std :: option :: Option :: Some (< Decimal as _pyo3 :: type_object :: PyTypeInfo > :: NAME) , func_name : "rescale" , positional_parameter_names : & ["scale"] , positional_only_parameters : 0usize , required_positional_parameters : 1usize , keyword_only_parameters : & [] , } ;
                                                    let mut output =
                                                        [::std::option::Option::None; 1usize];
                                                    let (_args , _kwargs) = DESCRIPTION . extract_arguments_fastcall :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _nargs , _kwnames , & mut output) ? ;
                                                    let arg0 = _pyo3 :: impl_ :: extract_argument :: extract_argument (_pyo3 :: impl_ :: extract_argument :: unwrap_required_argument (output [0usize]) , "scale") ? ;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::rescale(_slf, arg0),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::noargs(
                                "is_sign_negative\u{0}",
                                _pyo3::impl_::pymethods::PyCFunction({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf: &Decimal = &*_ref;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::is_sign_negative(_slf),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::noargs(
                                "is_sign_positive\u{0}",
                                _pyo3::impl_::pymethods::PyCFunction({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf: &Decimal = &*_ref;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::is_sign_positive(_slf),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::noargs(
                                "trunc\u{0}",
                                _pyo3::impl_::pymethods::PyCFunction({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf: &Decimal = &*_ref;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::trunc(_slf),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::noargs(
                                "fract\u{0}",
                                _pyo3::impl_::pymethods::PyCFunction({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf: &Decimal = &*_ref;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::fract(_slf),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::noargs(
                                "abs\u{0}",
                                _pyo3::impl_::pymethods::PyCFunction({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf: &Decimal = &*_ref;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::abs(_slf),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::noargs(
                                "floor\u{0}",
                                _pyo3::impl_::pymethods::PyCFunction({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf: &Decimal = &*_ref;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::floor(_slf),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::noargs(
                                "ceil\u{0}",
                                _pyo3::impl_::pymethods::PyCFunction({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf: &Decimal = &*_ref;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::ceil(_slf),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::fastcall_cfunction_with_keywords(
                                "max\u{0}",
                                _pyo3::impl_::pymethods::PyCFunctionFastWithKeywords({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *const *mut ::pyo3::ffi::PyObject,
                                        _nargs: ::pyo3::ffi::Py_ssize_t,
                                        _kwnames: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf: &Decimal = &*_ref;
                                                    const DESCRIPTION : _pyo3 :: impl_ :: extract_argument :: FunctionDescription = _pyo3 :: impl_ :: extract_argument :: FunctionDescription { cls_name : :: std :: option :: Option :: Some (< Decimal as _pyo3 :: type_object :: PyTypeInfo > :: NAME) , func_name : "max" , positional_parameter_names : & ["other"] , positional_only_parameters : 0usize , required_positional_parameters : 1usize , keyword_only_parameters : & [] , } ;
                                                    let mut output =
                                                        [::std::option::Option::None; 1usize];
                                                    let (_args , _kwargs) = DESCRIPTION . extract_arguments_fastcall :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _nargs , _kwnames , & mut output) ? ;
                                                    let arg0 = _pyo3 :: impl_ :: extract_argument :: extract_argument (_pyo3 :: impl_ :: extract_argument :: unwrap_required_argument (output [0usize]) , "other") ? ;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::max(_slf, arg0),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::fastcall_cfunction_with_keywords(
                                "min\u{0}",
                                _pyo3::impl_::pymethods::PyCFunctionFastWithKeywords({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *const *mut ::pyo3::ffi::PyObject,
                                        _nargs: ::pyo3::ffi::Py_ssize_t,
                                        _kwnames: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf: &Decimal = &*_ref;
                                                    const DESCRIPTION : _pyo3 :: impl_ :: extract_argument :: FunctionDescription = _pyo3 :: impl_ :: extract_argument :: FunctionDescription { cls_name : :: std :: option :: Option :: Some (< Decimal as _pyo3 :: type_object :: PyTypeInfo > :: NAME) , func_name : "min" , positional_parameter_names : & ["other"] , positional_only_parameters : 0usize , required_positional_parameters : 1usize , keyword_only_parameters : & [] , } ;
                                                    let mut output =
                                                        [::std::option::Option::None; 1usize];
                                                    let (_args , _kwargs) = DESCRIPTION . extract_arguments_fastcall :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _nargs , _kwnames , & mut output) ? ;
                                                    let arg0 = _pyo3 :: impl_ :: extract_argument :: extract_argument (_pyo3 :: impl_ :: extract_argument :: unwrap_required_argument (output [0usize]) , "other") ? ;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::min(_slf, arg0),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::noargs(
                                "normalize\u{0}",
                                _pyo3::impl_::pymethods::PyCFunction({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf: &Decimal = &*_ref;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::normalize(_slf),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::noargs(
                                "normalize_assign\u{0}",
                                _pyo3::impl_::pymethods::PyCFunction({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let mut _ref = _cell.try_borrow_mut()?;
                                                    let _slf: &mut Decimal = &mut *_ref;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::normalize_assign(_slf),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::noargs(
                                "round\u{0}",
                                _pyo3::impl_::pymethods::PyCFunction({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf: &Decimal = &*_ref;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::round(_slf),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::fastcall_cfunction_with_keywords(
                                "round_dp\u{0}",
                                _pyo3::impl_::pymethods::PyCFunctionFastWithKeywords({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *const *mut ::pyo3::ffi::PyObject,
                                        _nargs: ::pyo3::ffi::Py_ssize_t,
                                        _kwnames: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf: &Decimal = &*_ref;
                                                    const DESCRIPTION : _pyo3 :: impl_ :: extract_argument :: FunctionDescription = _pyo3 :: impl_ :: extract_argument :: FunctionDescription { cls_name : :: std :: option :: Option :: Some (< Decimal as _pyo3 :: type_object :: PyTypeInfo > :: NAME) , func_name : "round_dp" , positional_parameter_names : & ["dp"] , positional_only_parameters : 0usize , required_positional_parameters : 1usize , keyword_only_parameters : & [] , } ;
                                                    let mut output =
                                                        [::std::option::Option::None; 1usize];
                                                    let (_args , _kwargs) = DESCRIPTION . extract_arguments_fastcall :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _nargs , _kwnames , & mut output) ? ;
                                                    let arg0 = _pyo3 :: impl_ :: extract_argument :: extract_argument (_pyo3 :: impl_ :: extract_argument :: unwrap_required_argument (output [0usize]) , "dp") ? ;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::round_dp(_slf, arg0),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::fastcall_cfunction_with_keywords(
                                "round_sf\u{0}",
                                _pyo3::impl_::pymethods::PyCFunctionFastWithKeywords({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *const *mut ::pyo3::ffi::PyObject,
                                        _nargs: ::pyo3::ffi::Py_ssize_t,
                                        _kwnames: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf: &Decimal = &*_ref;
                                                    const DESCRIPTION : _pyo3 :: impl_ :: extract_argument :: FunctionDescription = _pyo3 :: impl_ :: extract_argument :: FunctionDescription { cls_name : :: std :: option :: Option :: Some (< Decimal as _pyo3 :: type_object :: PyTypeInfo > :: NAME) , func_name : "round_sf" , positional_parameter_names : & ["digits"] , positional_only_parameters : 0usize , required_positional_parameters : 1usize , keyword_only_parameters : & [] , } ;
                                                    let mut output =
                                                        [::std::option::Option::None; 1usize];
                                                    let (_args , _kwargs) = DESCRIPTION . extract_arguments_fastcall :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _nargs , _kwnames , & mut output) ? ;
                                                    let arg0 = _pyo3 :: impl_ :: extract_argument :: extract_argument (_pyo3 :: impl_ :: extract_argument :: unwrap_required_argument (output [0usize]) , "digits") ? ;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::round_sf(_slf, arg0),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::noargs(
                                "to_int\u{0}",
                                _pyo3::impl_::pymethods::PyCFunction({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf: &Decimal = &*_ref;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::to_int(_slf),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                        _pyo3::class::PyMethodDefType::Method(
                            _pyo3::impl_::pymethods::PyMethodDef::noargs(
                                "to_float\u{0}",
                                _pyo3::impl_::pymethods::PyCFunction({
                                    unsafe extern "C" fn __wrap(
                                        _slf: *mut ::pyo3::ffi::PyObject,
                                        _args: *mut ::pyo3::ffi::PyObject,
                                    ) -> *mut ::pyo3::ffi::PyObject
                                    {
                                        use :: pyo3 as _pyo3;
                                        let gil = _pyo3::GILPool::new();
                                        let _py = gil.python();
                                        _pyo3::callback::panic_result_into_callback_output(
                                            _py,
                                            ::std::panic::catch_unwind(
                                                move || -> _pyo3::PyResult<_> {
                                                    let _cell = _py
                                                        .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                                        .downcast::<_pyo3::PyCell<Decimal>>()?;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf: &Decimal = &*_ref;
                                                    _pyo3::callback::convert(
                                                        _py,
                                                        Decimal::to_float(_slf),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                    __wrap
                                }),
                                "\u{0}",
                            ),
                        ),
                    ],
                    slots: &[
                        {
                            impl Decimal {
                                #[doc(hidden)]
                                unsafe extern "C" fn __pymethod__new__(
                                    subtype: *mut ::pyo3::ffi::PyTypeObject,
                                    _args: *mut ::pyo3::ffi::PyObject,
                                    _kwargs: *mut ::pyo3::ffi::PyObject,
                                ) -> *mut ::pyo3::ffi::PyObject {
                                    use :: pyo3 as _pyo3;
                                    use _pyo3::callback::IntoPyCallbackOutput;
                                    let gil = _pyo3::GILPool::new();
                                    let _py = gil.python();
                                    _pyo3::callback::panic_result_into_callback_output(
                                        _py,
                                        ::std::panic::catch_unwind(
                                            move || -> _pyo3::PyResult<_> {
                                                const DESCRIPTION : _pyo3 :: impl_ :: extract_argument :: FunctionDescription = _pyo3 :: impl_ :: extract_argument :: FunctionDescription { cls_name : :: std :: option :: Option :: Some (< Decimal as _pyo3 :: type_object :: PyTypeInfo > :: NAME) , func_name : "__new__" , positional_parameter_names : & ["arg1" , "arg2"] , positional_only_parameters : 0usize , required_positional_parameters : 1usize , keyword_only_parameters : & [] , } ;
                                                let mut output =
                                                    [::std::option::Option::None; 2usize];
                                                let (_args , _kwargs) = DESCRIPTION . extract_arguments_tuple_dict :: < _pyo3 :: impl_ :: extract_argument :: NoVarargs , _pyo3 :: impl_ :: extract_argument :: NoVarkeywords > (_py , _args , _kwargs , & mut output) ? ;
                                                let arg0 = _pyo3 :: impl_ :: extract_argument :: extract_argument (_pyo3 :: impl_ :: extract_argument :: unwrap_required_argument (output [0usize]) , "arg1") ? ;
                                                let arg1 = _pyo3 :: impl_ :: extract_argument :: extract_argument_with_default (output [1usize] , "arg2" , | | None) ? ;
                                                let arg2 = _py;
                                                let result = Decimal::new(arg0, arg1, arg2);
                                                let initializer: _pyo3::PyClassInitializer<
                                                    Decimal,
                                                > = result.convert(_py)?;
                                                let cell = initializer
                                                    .create_cell_from_subtype(_py, subtype)?;
                                                ::std::result::Result::Ok(
                                                    cell as *mut _pyo3::ffi::PyObject,
                                                )
                                            },
                                        ),
                                    )
                                }
                            }
                            _pyo3::ffi::PyType_Slot {
                                slot: _pyo3::ffi::Py_tp_new,
                                pfunc: Decimal::__pymethod__new__ as _pyo3::ffi::newfunc as _,
                            }
                        },
                        {
                            unsafe extern "C" fn __wrap(
                                _raw_slf: *mut _pyo3::ffi::PyObject,
                            ) -> *mut _pyo3::ffi::PyObject {
                                let _slf = _raw_slf;
                                let gil = _pyo3::GILPool::new();
                                let _py = gil.python();
                                _pyo3::callback::panic_result_into_callback_output(
                                    _py,
                                    ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                                        let _cell = _py
                                            .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                            .downcast::<_pyo3::PyCell<Decimal>>()?;
                                        let _ref = _cell.try_borrow()?;
                                        let _slf: &Decimal = &*_ref;
                                        _pyo3::callback::convert(_py, Decimal::__neg__(_slf))
                                    }),
                                )
                            }
                            _pyo3::ffi::PyType_Slot {
                                slot: _pyo3::ffi::Py_nb_negative,
                                pfunc: __wrap as _pyo3::ffi::unaryfunc as _,
                            }
                        },
                        {
                            unsafe extern "C" fn __wrap(
                                _raw_slf: *mut _pyo3::ffi::PyObject,
                                arg0: *mut _pyo3::ffi::PyObject,
                                arg1: ::std::os::raw::c_int,
                            ) -> *mut _pyo3::ffi::PyObject {
                                let _slf = _raw_slf;
                                let gil = _pyo3::GILPool::new();
                                let _py = gil.python();
                                _pyo3::callback::panic_result_into_callback_output(
                                    _py,
                                    ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                                        let _cell = match _py
                                            .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                            .downcast::<_pyo3::PyCell<Decimal>>()
                                        {
                                            ::std::result::Result::Ok(cell) => cell,
                                            ::std::result::Result::Err(_) => {
                                                return _pyo3::callback::convert(
                                                    _py,
                                                    _py.NotImplemented(),
                                                )
                                            }
                                        };
                                        let _ref = _cell.try_borrow()?;
                                        let _slf: &Decimal = &*_ref;
                                        let arg0 = match _py
                                            .from_borrowed_ptr::<_pyo3::PyAny>(arg0)
                                            .extract()
                                        {
                                            ::std::result::Result::Ok(value) => value,
                                            ::std::result::Result::Err(_) => {
                                                return _pyo3::callback::convert(
                                                    _py,
                                                    _py.NotImplemented(),
                                                );
                                            }
                                        };
                                        let arg1 =
                                            match _pyo3::class::basic::CompareOp::from_raw(arg1)
                                                .ok_or_else(|| {
                                                    _pyo3::exceptions::PyValueError::new_err(
                                                        "invalid comparison operator",
                                                    )
                                                }) {
                                                ::std::result::Result::Ok(value) => value,
                                                ::std::result::Result::Err(_) => {
                                                    return _pyo3::callback::convert(
                                                        _py,
                                                        _py.NotImplemented(),
                                                    );
                                                }
                                            };
                                        _pyo3::callback::convert(
                                            _py,
                                            Decimal::__richcmp__(_slf, arg0, arg1),
                                        )
                                    }),
                                )
                            }
                            _pyo3::ffi::PyType_Slot {
                                slot: _pyo3::ffi::Py_tp_richcompare,
                                pfunc: __wrap as _pyo3::ffi::richcmpfunc as _,
                            }
                        },
                        {
                            unsafe extern "C" fn __wrap(
                                _raw_slf: *mut _pyo3::ffi::PyObject,
                            ) -> *mut _pyo3::ffi::PyObject {
                                let _slf = _raw_slf;
                                let gil = _pyo3::GILPool::new();
                                let _py = gil.python();
                                _pyo3::callback::panic_result_into_callback_output(
                                    _py,
                                    ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                                        let _cell = _py
                                            .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                            .downcast::<_pyo3::PyCell<Decimal>>()?;
                                        let _ref = _cell.try_borrow()?;
                                        let _slf: &Decimal = &*_ref;
                                        _pyo3::callback::convert(_py, Decimal::__str__(_slf))
                                    }),
                                )
                            }
                            _pyo3::ffi::PyType_Slot {
                                slot: _pyo3::ffi::Py_tp_str,
                                pfunc: __wrap as _pyo3::ffi::reprfunc as _,
                            }
                        },
                        {
                            unsafe extern "C" fn __wrap(
                                _raw_slf: *mut _pyo3::ffi::PyObject,
                            ) -> *mut _pyo3::ffi::PyObject {
                                let _slf = _raw_slf;
                                let gil = _pyo3::GILPool::new();
                                let _py = gil.python();
                                _pyo3::callback::panic_result_into_callback_output(
                                    _py,
                                    ::std::panic::catch_unwind(move || -> _pyo3::PyResult<_> {
                                        let _cell = _py
                                            .from_borrowed_ptr::<_pyo3::PyAny>(_slf)
                                            .downcast::<_pyo3::PyCell<Decimal>>()?;
                                        let _ref = _cell.try_borrow()?;
                                        let _slf: &Decimal = &*_ref;
                                        _pyo3::callback::convert(_py, Decimal::__repr__(_slf))
                                    }),
                                )
                            }
                            _pyo3::ffi::PyType_Slot {
                                slot: _pyo3::ffi::Py_tp_repr,
                                pfunc: __wrap as _pyo3::ffi::reprfunc as _,
                            }
                        },
                        {
                            unsafe extern "C" fn __wrap(
                                _slf: *mut ::pyo3::ffi::PyObject,
                                _other: *mut ::pyo3::ffi::PyObject,
                            ) -> *mut ::pyo3::ffi::PyObject {
                                let gil = ::pyo3::GILPool::new();
                                let py = gil.python();
                                ::pyo3::callback::panic_result_into_callback_output(
                                    py,
                                    ::std::panic::catch_unwind(move || -> ::pyo3::PyResult<_> {
                                        use ::pyo3::impl_::pyclass::*;
                                        let collector = PyClassImplCollector::<Decimal>::new();
                                        let lhs_result = collector.__add__(py, _slf, _other)?;
                                        if lhs_result == ::pyo3::ffi::Py_NotImplemented() {
                                            ::pyo3::ffi::Py_DECREF(lhs_result);
                                            collector.__radd__(py, _other, _slf)
                                        } else {
                                            ::std::result::Result::Ok(lhs_result)
                                        }
                                    }),
                                )
                            }
                            ::pyo3::ffi::PyType_Slot {
                                slot: ::pyo3::ffi::Py_nb_add,
                                pfunc: __wrap as ::pyo3::ffi::binaryfunc as _,
                            }
                        },
                        {
                            unsafe extern "C" fn __wrap(
                                _slf: *mut ::pyo3::ffi::PyObject,
                                _other: *mut ::pyo3::ffi::PyObject,
                            ) -> *mut ::pyo3::ffi::PyObject {
                                let gil = ::pyo3::GILPool::new();
                                let py = gil.python();
                                ::pyo3::callback::panic_result_into_callback_output(
                                    py,
                                    ::std::panic::catch_unwind(move || -> ::pyo3::PyResult<_> {
                                        use ::pyo3::impl_::pyclass::*;
                                        let collector = PyClassImplCollector::<Decimal>::new();
                                        let lhs_result = collector.__sub__(py, _slf, _other)?;
                                        if lhs_result == ::pyo3::ffi::Py_NotImplemented() {
                                            ::pyo3::ffi::Py_DECREF(lhs_result);
                                            collector.__rsub__(py, _other, _slf)
                                        } else {
                                            ::std::result::Result::Ok(lhs_result)
                                        }
                                    }),
                                )
                            }
                            ::pyo3::ffi::PyType_Slot {
                                slot: ::pyo3::ffi::Py_nb_subtract,
                                pfunc: __wrap as ::pyo3::ffi::binaryfunc as _,
                            }
                        },
                        {
                            unsafe extern "C" fn __wrap(
                                _slf: *mut ::pyo3::ffi::PyObject,
                                _other: *mut ::pyo3::ffi::PyObject,
                            ) -> *mut ::pyo3::ffi::PyObject {
                                let gil = ::pyo3::GILPool::new();
                                let py = gil.python();
                                ::pyo3::callback::panic_result_into_callback_output(
                                    py,
                                    ::std::panic::catch_unwind(move || -> ::pyo3::PyResult<_> {
                                        use ::pyo3::impl_::pyclass::*;
                                        let collector = PyClassImplCollector::<Decimal>::new();
                                        let lhs_result = collector.__mul__(py, _slf, _other)?;
                                        if lhs_result == ::pyo3::ffi::Py_NotImplemented() {
                                            ::pyo3::ffi::Py_DECREF(lhs_result);
                                            collector.__rmul__(py, _other, _slf)
                                        } else {
                                            ::std::result::Result::Ok(lhs_result)
                                        }
                                    }),
                                )
                            }
                            ::pyo3::ffi::PyType_Slot {
                                slot: ::pyo3::ffi::Py_nb_multiply,
                                pfunc: __wrap as ::pyo3::ffi::binaryfunc as _,
                            }
                        },
                        {
                            unsafe extern "C" fn __wrap(
                                _slf: *mut ::pyo3::ffi::PyObject,
                                _other: *mut ::pyo3::ffi::PyObject,
                            ) -> *mut ::pyo3::ffi::PyObject {
                                let gil = ::pyo3::GILPool::new();
                                let py = gil.python();
                                ::pyo3::callback::panic_result_into_callback_output(
                                    py,
                                    ::std::panic::catch_unwind(move || -> ::pyo3::PyResult<_> {
                                        use ::pyo3::impl_::pyclass::*;
                                        let collector = PyClassImplCollector::<Decimal>::new();
                                        let lhs_result = collector.__truediv__(py, _slf, _other)?;
                                        if lhs_result == ::pyo3::ffi::Py_NotImplemented() {
                                            ::pyo3::ffi::Py_DECREF(lhs_result);
                                            collector.__rtruediv__(py, _other, _slf)
                                        } else {
                                            ::std::result::Result::Ok(lhs_result)
                                        }
                                    }),
                                )
                            }
                            ::pyo3::ffi::PyType_Slot {
                                slot: ::pyo3::ffi::Py_nb_true_divide,
                                pfunc: __wrap as ::pyo3::ffi::binaryfunc as _,
                            }
                        },
                        {
                            unsafe extern "C" fn __wrap(
                                _slf: *mut ::pyo3::ffi::PyObject,
                                _other: *mut ::pyo3::ffi::PyObject,
                            ) -> *mut ::pyo3::ffi::PyObject {
                                let gil = ::pyo3::GILPool::new();
                                let py = gil.python();
                                ::pyo3::callback::panic_result_into_callback_output(
                                    py,
                                    ::std::panic::catch_unwind(move || -> ::pyo3::PyResult<_> {
                                        use ::pyo3::impl_::pyclass::*;
                                        let collector = PyClassImplCollector::<Decimal>::new();
                                        let lhs_result =
                                            collector.__floordiv__(py, _slf, _other)?;
                                        if lhs_result == ::pyo3::ffi::Py_NotImplemented() {
                                            ::pyo3::ffi::Py_DECREF(lhs_result);
                                            collector.__rfloordiv__(py, _other, _slf)
                                        } else {
                                            ::std::result::Result::Ok(lhs_result)
                                        }
                                    }),
                                )
                            }
                            ::pyo3::ffi::PyType_Slot {
                                slot: ::pyo3::ffi::Py_nb_floor_divide,
                                pfunc: __wrap as ::pyo3::ffi::binaryfunc as _,
                            }
                        },
                    ],
                };
            &ITEMS
        }
    }
};
impl Decimal {
    pub fn from_i128_with_scale<'p>(num: i128, scale: u32) -> Decimal {
        Self(
            RustDecimal::from_i128_with_scale(num, 0),
            *DECIMAL_VERSION_HASH,
        )
    }
}
impl Deref for Decimal {
    type Target = RustDecimal;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Decimal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Into<RustDecimal> for Decimal {
    fn into(self) -> RustDecimal {
        self.0
    }
}
impl Into<Decimal> for RustDecimal {
    fn into(self) -> Decimal {
        Decimal(self, *DECIMAL_VERSION_HASH)
    }
}
/// This module is a python module implemented in Rust.
fn rust_binding(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Decimal>()?;
    m.add_wrapped(&(|arg| get_decimal_version_info::wrap(get_decimal_version_info::DEF, arg)))?;
    Ok(())
}
#[no_mangle]
#[allow(non_snake_case)]
/// This autogenerated function is called by the python interpreter when importing
/// the module.
pub unsafe extern "C" fn PyInit_rust_binding() -> *mut ::pyo3::ffi::PyObject {
    unsafe { __PYO3_PYMODULE_DEF_RUST_BINDING.module_init() }
}
#[doc(hidden)]
static __PYO3_PYMODULE_DEF_RUST_BINDING: ::pyo3::impl_::pymodule::ModuleDef = unsafe {
    ::pyo3::impl_::pymodule::ModuleDef::new(
        "rust_binding\u{0}",
        "This module is a python module implemented in Rust.\u{0}",
        ::pyo3::impl_::pymodule::ModuleInitializer(rust_binding),
    )
};
