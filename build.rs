//
// build.rs
// Copyright (C) 2022 fx-kirin <fx.kirin@gmail.com>
// Distributed under terms of the MIT license.
//
use pyo3_decimal_macro::make_build_info;

fn main() {
    make_build_info!();
}
