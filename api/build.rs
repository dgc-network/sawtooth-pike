// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

extern crate protobuf_codegen_pure;
extern crate protoc_rust;
use protoc_rust::Customize;

use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    fs::create_dir_all("src/protos").unwrap();
/*
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protos",
        input: &["../protos/payload.proto", "../protos/state.proto"],
        includes: &["../protos"],
        customize: Customize::default(),
    }).expect("protoc");
*/
    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src/protos",
        input: &["../protos/payload.proto", "../protos/state.proto"],
        includes: &["../protos"],
        customize: Customize {
          ..Default::default()
        },
    }).expect("protoc");
    
    let mut file = File::create("src/protos/mod.rs").unwrap();
    file.write_all(b"pub mod payload;\n").unwrap();
    file.write_all(b"pub mod state;\n").unwrap();
}


