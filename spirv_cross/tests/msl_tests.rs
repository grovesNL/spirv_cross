extern crate spirv_cross;
use spirv_cross::{msl, spirv};

mod common;
use common::words_from_bytes;

#[test]
fn msl_compiler_options_has_default() {
    let compiler_options = msl::CompilerOptions::default();
    assert_eq!(compiler_options.vertex.invert_y, false);
    assert_eq!(compiler_options.vertex.transform_clip_space, false);
}

#[test]
fn ast_compiles_to_msl() {
    let mut ast = spirv::Ast::<msl::Target>::parse(&spirv::Module::from_words(
        words_from_bytes(include_bytes!("shaders/simple.spv")),
    )).unwrap();
    ast.set_compile_options(msl::CompilerOptions {
        vertex: msl::CompilerVertexOptions::default(),
    }).unwrap();

    assert_eq!(
        ast.compile().unwrap(),
        "\
#include <metal_stdlib>
#include <simd/simd.h>

using namespace metal;

struct uniform_buffer_object
{
    float4x4 u_model_view_projection;
};

struct main0_in
{
    float3 a_normal [[attribute(1)]];
    float4 a_position [[attribute(0)]];
};

struct main0_out
{
    float3 v_normal [[user(locn0)]];
    float4 gl_Position [[position]];
};

vertex main0_out main0(main0_in in [[stage_in]], constant uniform_buffer_object& _22 [[buffer(0)]])
{
    main0_out out = {};
    out.v_normal = in.a_normal;
    out.gl_Position = _22.u_model_view_projection * in.a_position;
    return out;
}

"
    );
}
