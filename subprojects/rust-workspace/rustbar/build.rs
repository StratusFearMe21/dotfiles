use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use case::CaseExt;

fn main() {
    let lang = tree_sitter_dconfsomebar::language();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let mut out_enum = BufWriter::new(File::create(Path::new(&out_dir).join("kinds.rs")).unwrap());

    out_enum
        .write_all(
            b"#[repr(u16)] #[derive(Default,num_enum::FromPrimitive,Debug)] pub enum NodeKind {",
        )
        .unwrap();

    for id in 0..lang.node_kind_count() as u16 {
        if lang.node_kind_is_named(id) && lang.node_kind_is_visible(id) {
            let kind = lang.node_kind_for_id(id).unwrap().to_camel();
            out_enum
                .write_fmt(format_args!("{}={},", kind, id))
                .unwrap();
        }
    }

    out_enum.write_all(b"#[default] Invalid}").unwrap();

    out_enum
        .write_all(
            b"#[repr(u16)] #[derive(Default,num_enum::FromPrimitive,Debug)] pub enum FieldName {",
        )
        .unwrap();

    for id in 0..lang.field_count() as u16 {
        if let Some(name) = lang.field_name_for_id(id) {
            out_enum
                .write_fmt(format_args!("{}={},", name.to_camel(), id))
                .unwrap();
        }
    }

    out_enum.write_all(b"#[default] Invalid}").unwrap();
    println!("cargo:rerun-if-changed=../tree-sitter-dconfsomebar/src/parser.c");

    cxx_build::bridge("src/main.rs")
        .file("src/wrapper.cpp")
        .compile("cxxbridge-demo");

    println!("cargo:rustc-link-lib=giac");
    println!("cargo:rustc-link-lib=gmp");
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/wrapper.cpp");
    println!("cargo:rerun-if-changed=src/wrapper.h");
}
