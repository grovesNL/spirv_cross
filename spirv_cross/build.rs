extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .include("src")
        .file("src/wrapper.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_cfg.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_cross.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_glsl.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_hlsl.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_msl.cpp")
        .flag_if_supported("-std=c++14")
        .compile("spirv-cross-rust-wrapper");
}
