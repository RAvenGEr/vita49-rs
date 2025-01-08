// SPDX-FileCopyrightText: 2025 The vita49-rs Authors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/main.cc")
        .std("c++14")
        .compile("cxx_demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/main.cc");
    println!("cargo:rerun-if-changed=include/my_vrt.h");
}
