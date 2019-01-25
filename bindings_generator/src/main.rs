extern crate bindgen;

use std::env;

fn main() {
    let out_path = env::current_dir().unwrap();
    bindgen::Builder::default()
        .header(
            out_path
                .join("../spirv_cross/src/wrapper.hpp")
                .to_str()
                .unwrap(),
        )
        .clang_args(["-x", "c++", "-std=c++14"].iter())
        .enable_cxx_namespaces()
        .whitelisted_function("sc_internal.*")
        .whitelisted_type("spv::.*")
        .bitfield_enum(".*(Mask|Flags)")
        .whitelisted_type("spirv_cross::Resource")
        .whitelisted_type("spirv_cross::MSLVertexAttr")
        .whitelisted_type("spirv_cross::MSLResourceBinding")
        .opaque_type("std::.*")
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("../spirv_cross/src/bindings.rs"))
        .expect("Couldn't write bindings!");
}
