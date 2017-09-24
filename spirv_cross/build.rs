extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/wrapper.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_cfg.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_cross.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_glsl.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_hlsl.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_msl.cpp")
        .compile("spirv-cross-rust-wrapper");
}
