extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    cc::Build::new()
        .cpp(true)
        .file("src/wrapper.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_cfg.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_cross.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_glsl.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_hlsl.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_msl.cpp")
        .compile("spirv-cross-rust-wrapper");

    let bindings = bindgen::Builder::default()
        .header("src/wrapper.hpp")
        .clang_args(["-x", "c++", "-std=c++14"].iter())
        .enable_cxx_namespaces()
        .whitelisted_function("sc_internal.*")
        .whitelisted_type("spv::.*")
        .bitfield_enum(".*(Mask|Flags)")
        .whitelisted_type("spirv_cross::Resource")
        .opaque_type("std::.*")
        .generate()
        .expect("Unable to generate bindings");

    println!("{:?}", out_path);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
