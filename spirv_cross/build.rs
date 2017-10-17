extern crate cc;
use cc::ToolFamily::*;

fn main() {
    let mut build = cc::Build::new();
    build.cpp(true);
    let compiler = build.try_get_compiler();
    if compiler.is_ok() && compiler.unwrap().family == Clang {
        build.flag_if_supported("-std=c++11").cpp_set_stdlib("c++");
    } else {
        build.flag_if_supported("-std=c++14");
    }
    build
        .file("src/wrapper.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_cfg.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_cross.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_glsl.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_hlsl.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_msl.cpp")
        .compile("spirv-cross-rust-wrapper");
}
