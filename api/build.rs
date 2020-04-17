// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

extern crate protoc_rust;
use protoc_rust::{Customize, Codegen};

use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    fs::create_dir_all("src/protos").unwrap();

    Codegen::new()
    .protoc_path(protoc_bin_vendored::protoc_bin_path().unwrap())
    .out_dir("src/protos")
    .input(&["../protos/payload.proto", "../protos/state.proto"])
    .includes(&["../protos"])
    .customize(Customize::default())
    // ...
    .run()
    .unwrap();
/*
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protos",
        input: &["../protos/payload.proto", "../protos/state.proto"],
        includes: &["../protos"],
        customize: Customize::default(),
    }).expect("protoc");
*/    
    let mut file = File::create("src/protos/mod.rs").unwrap();
    file.write_all(b"pub mod payload;\n").unwrap();
    file.write_all(b"pub mod state;\n").unwrap();
}


