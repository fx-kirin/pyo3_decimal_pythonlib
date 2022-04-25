use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3_decimal_macro::make_decimal;
use gdb_breakpoint::breakpoint;

make_decimal!();

/// This module is a python module implemented in Rust.
#[pymodule]
fn rust_binding(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Decimal>()?;
    m.add_wrapped(wrap_pyfunction!(get_decimal_version_info))?;

    Ok(())
}
