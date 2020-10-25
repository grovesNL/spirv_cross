extern crate bindgen;

use std::env;

fn main() {
    let out_path = env::current_dir().unwrap();
    // For native targets, include all types and functions
    bindgen::Builder::default()
        .header(
            out_path
                .join("../spirv_cross/src/wrapper.hpp")
                .to_str()
                .unwrap(),
        )
        .clang_arg("-I/Library/Developer/CommandLineTools/usr/include/c++/v1")
        .clang_arg("-I/Library/Developer/CommandLineTools/usr/lib/clang/10.0.1/include")
        .clang_args(["-x", "c++", "-std=c++14"].iter())
        .enable_cxx_namespaces()
        .whitelist_function("sc_internal.*")
        .whitelist_type("spv::.*")
        .whitelist_type("Sc.*")
        .bitfield_enum(".*(Mask|Flags)")
        .rustified_enum("spv::BuiltIn")
        .rustified_enum("spv::Decoration")
        .rustified_enum("spv::ExecutionModel")
        .rustified_enum("ScInternalResult")
        .rustified_enum("spirv_cross::SPIRType_BaseType")
        .rustified_enum("spirv_cross::MSLVertexFormat")
        .opaque_type("std::.*")
        .clang_args(vec![
            "-DSPIRV_CROSS_WRAPPER_GLSL",
            "-DSPIRV_CROSS_WRAPPER_MSL",
            "-DSPIRV_CROSS_WRAPPER_HLSL",
        ])
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("../spirv_cross/src/bindings_native.rs"))
        .expect("Couldn't write bindings!");
    // For wasm targets, include all types, functions will be implemented manually
    bindgen::Builder::default()
        .header(
            out_path
                .join("../spirv_cross/src/wrapper.hpp")
                .to_str()
                .unwrap(),
        )
        .clang_arg("-I/Library/Developer/CommandLineTools/usr/include/c++/v1")
        .clang_arg("-I/Library/Developer/CommandLineTools/usr/lib/clang/10.0.1/include")
        .clang_args(["-x", "c++", "-std=c++14"].iter())
        .enable_cxx_namespaces()
        .whitelist_type("spv::.*")
        .whitelist_type("Sc.*")
        .bitfield_enum(".*(Mask|Flags)")
        .rustified_enum("spv::BuiltIn")
        .rustified_enum("spv::Decoration")
        .rustified_enum("spv::ExecutionModel")
        .rustified_enum("ScInternalResult")
        .rustified_enum("spirv_cross::SPIRType_BaseType")
        .rustified_enum("spirv_cross::MSLVertexFormat")
        .opaque_type("std::.*")
        .clang_args(vec![
            "-DSPIRV_CROSS_WRAPPER_GLSL",
            "-DSPIRV_CROSS_WRAPPER_MSL",
            "-DSPIRV_CROSS_WRAPPER_HLSL",
        ])
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("../spirv_cross/src/bindings_wasm.rs"))
        .expect("Couldn't write bindings!");
}
