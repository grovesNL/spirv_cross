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
        .whitelisted_function("sc_internal.*")
        .whitelisted_type("spv::.*")
        .bitfield_enum(".*(Mask|Flags)")
        .whitelisted_type("SPIRV_CROSS_NAMESPACE::Resource")
        .whitelisted_type("SPIRV_CROSS_NAMESPACE::MSLVertexAttr")
        .whitelisted_type("SPIRV_CROSS_NAMESPACE::MSLResourceBinding")
        // TODO: Simplify with glob
        .whitelisted_type("ScInternalCompilerBase")
        .whitelisted_type("ScInternalCompilerHlsl")
        .whitelisted_type("ScInternalCompilerMsl")
        .whitelisted_type("ScInternalCompilerGlsl")
        .whitelisted_type("ScInternalResult")
        .whitelisted_type("ScEntryPoint")
        .whitelisted_type("ScCombinedImageSampler")
        .whitelisted_type("ScHlslRootConstant")
        .whitelisted_type("ScHlslCompilerOptions")
        .whitelisted_type("ScMslCompilerOptions")
        .whitelisted_type("ScGlslCompilerOptions")
        .whitelisted_type("ScResource")
        .whitelisted_type("ScResourceArray")
        .whitelisted_type("ScShaderResources")
        .whitelisted_type("ScSpecializationConstant")
        .whitelisted_type("ScType")
        .opaque_type("std::.*")
        .clang_args(vec!["-DSPIRV_CROSS_WRAPPER_GLSL", "-DSPIRV_CROSS_WRAPPER_MSL", "-DSPIRV_CROSS_WRAPPER_HLSL"])
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
        .whitelisted_type("spv::.*")
        .bitfield_enum(".*(Mask|Flags)")
        .whitelisted_type("SPIRV_CROSS_NAMESPACE::Resource")
        .whitelisted_type("SPIRV_CROSS_NAMESPACE::MSLVertexAttr")
        .whitelisted_type("SPIRV_CROSS_NAMESPACE::MSLResourceBinding")
        // TODO: Simplify with glob
        .whitelisted_type("ScInternalCompilerBase")
        .whitelisted_type("ScInternalCompilerHlsl")
        .whitelisted_type("ScInternalCompilerMsl")
        .whitelisted_type("ScInternalCompilerGlsl")
        .whitelisted_type("ScInternalResult")
        .whitelisted_type("ScEntryPoint")
        .whitelisted_type("ScCombinedImageSampler")
        .whitelisted_type("ScHlslRootConstant")
        .whitelisted_type("ScHlslCompilerOptions")
        .whitelisted_type("ScMslCompilerOptions")
        .whitelisted_type("ScGlslCompilerOptions")
        .whitelisted_type("ScResource")
        .whitelisted_type("ScResourceArray")
        .whitelisted_type("ScShaderResources")
        .whitelisted_type("ScSpecializationConstant")
        .whitelisted_type("ScType")
        .opaque_type("std::.*")
        .clang_args(vec!["-DSPIRV_CROSS_WRAPPER_GLSL", "-DSPIRV_CROSS_WRAPPER_MSL", "-DSPIRV_CROSS_WRAPPER_HLSL"])
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("../spirv_cross/src/bindings_wasm.rs"))
        .expect("Couldn't write bindings!");
}
