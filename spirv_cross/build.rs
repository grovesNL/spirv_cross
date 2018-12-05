extern crate cc;

fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS");
    let is_macos = target_os.is_ok() && target_os.unwrap() == "macos";

    let mut build = cc::Build::new();
    build.cpp(true);

    let compiler = build.try_get_compiler();
    let is_clang = compiler.is_ok() && compiler.unwrap().is_like_clang();

    if is_macos && is_clang {
        build.flag("-std=c++14").cpp_set_stdlib("c++");
    } else {
        build.flag_if_supported("-std=c++14");
    }

    build
        .file("src/wrapper.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_cfg.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_cross.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_cross_parsed_ir.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_parser.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_cross_util.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_glsl.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_hlsl.cpp")
        .file("src/vendor/SPIRV-Cross/spirv_msl.cpp")
        .compile("spirv-cross-rust-wrapper");
}
