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
        .clang_args(["-x", "c++", "-std=c++14"].iter())
        .enable_cxx_namespaces()
        .whitelist_function("sc_internal.*")
        .whitelist_type("spv::.*")
        .bitfield_enum(".*(Mask|Flags)")
        .whitelist_type("SPIRV_CROSS_NAMESPACE::Resource")
        .whitelist_type("SPIRV_CROSS_NAMESPACE::MSLVertexAttr")
        .whitelist_type("SPIRV_CROSS_NAMESPACE::MSLResourceBinding")
        // TODO: Simplify with glob
        .whitelist_type("ScInternalCompilerBase")
        .whitelist_type("ScInternalCompilerHlsl")
        .whitelist_type("ScInternalCompilerMsl")
        .whitelist_type("ScInternalCompilerGlsl")
        .whitelist_type("ScInternalResult")
        .whitelist_type("ScEntryPoint")
        .whitelist_type("ScCombinedImageSampler")
        .whitelist_type("ScHlslRootConstant")
        .whitelist_type("ScHlslCompilerOptions")
        .whitelist_type("ScMslCompilerOptions")
        .whitelist_type("ScGlslCompilerOptions")
        .whitelist_type("ScResource")
        .whitelist_type("ScResourceArray")
        .whitelist_type("ScShaderResources")
        .whitelist_type("ScSpecializationConstant")
        .whitelist_type("ScType")
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
        .clang_args(["-x", "c++", "-std=c++14"].iter())
        .enable_cxx_namespaces()
        .whitelist_type("spv::.*")
        .bitfield_enum(".*(Mask|Flags)")
        .whitelist_type("SPIRV_CROSS_NAMESPACE::Resource")
        .whitelist_type("SPIRV_CROSS_NAMESPACE::MSLVertexAttr")
        .whitelist_type("SPIRV_CROSS_NAMESPACE::MSLResourceBinding")
        // TODO: Simplify with glob
        .whitelist_type("ScInternalCompilerBase")
        .whitelist_type("ScInternalCompilerHlsl")
        .whitelist_type("ScInternalCompilerMsl")
        .whitelist_type("ScInternalCompilerGlsl")
        .whitelist_type("ScInternalResult")
        .whitelist_type("ScEntryPoint")
        .whitelist_type("ScCombinedImageSampler")
        .whitelist_type("ScHlslRootConstant")
        .whitelist_type("ScHlslCompilerOptions")
        .whitelist_type("ScMslCompilerOptions")
        .whitelist_type("ScGlslCompilerOptions")
        .whitelist_type("ScResource")
        .whitelist_type("ScResourceArray")
        .whitelist_type("ScShaderResources")
        .whitelist_type("ScSpecializationConstant")
        .whitelist_type("ScType")
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
