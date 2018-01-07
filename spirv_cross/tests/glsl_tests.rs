extern crate spirv_cross;
use spirv_cross::{glsl, spirv};

mod common;
use common::words_from_bytes;

#[test]
fn glsl_compiler_options_has_default() {
    let compiler_options = glsl::CompilerOptions::default();
    assert_eq!(compiler_options.vertex.invert_y, false);
    assert_eq!(compiler_options.vertex.transform_clip_space, false);
}

#[test]
fn ast_compiles_to_glsl() {
    let mut ast = spirv::Ast::<glsl::Target>::parse(&spirv::Module::from_words(
        words_from_bytes(include_bytes!("shaders/simple.spv")),
    )).unwrap();
    ast.set_compiler_options(&glsl::CompilerOptions {
        version: glsl::Version::V4_60,
        vertex: glsl::CompilerVertexOptions::default(),
    }).unwrap();

    assert_eq!(
        ast.compile().unwrap(),
        "\
#version 460

layout(std140) uniform uniform_buffer_object
{
    mat4 u_model_view_projection;
    float u_scale;
} _22;

layout(location = 0) out vec3 v_normal;
layout(location = 1) in vec3 a_normal;
layout(location = 0) in vec4 a_position;

void main()
{
    v_normal = a_normal;
    gl_Position = (_22.u_model_view_projection * a_position) * _22.u_scale;
}

"
    );
}

#[test]
fn ast_compiles_all_versions_to_glsl() {
    use glsl::Version::*;

    let module = spirv::Module::from_words(words_from_bytes(include_bytes!("shaders/simple.spv")));
    let mut ast = spirv::Ast::<glsl::Target>::parse(&module).unwrap();

    let versions = [
        V1_10,
        V1_20,
        V1_30,
        V1_40,
        V1_50,
        V3_30,
        V4_00,
        V4_10,
        V4_20,
        V4_30,
        V4_40,
        V4_50,
        V4_60,
        V1_00Es,
        V3_00Es,
    ];
    for &version in versions.iter() {
        match ast.set_compiler_options(&glsl::CompilerOptions {
            version,
            vertex: glsl::CompilerVertexOptions::default(),
        }) {
            Err(_) => panic!("Did not compile"),
            _ => (),
        }
    }
}
