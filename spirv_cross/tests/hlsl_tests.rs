extern crate spirv_cross;
use spirv_cross::{hlsl, spirv};

mod common;
use common::words_from_bytes;

#[test]
fn ast_compiles_to_hlsl() {
    let mut ast = spirv::Ast::<hlsl::Target>::parse(&spirv::Module::from_words(
        words_from_bytes(include_bytes!("shaders/simple.spv")),
    )).unwrap();
    ast.set_compile_options(hlsl::CompilerOptions {
        shader_model: hlsl::ShaderModel::V6_0,
        vertex: hlsl::CompilerVertexOptions::default(),
    }).unwrap();

    assert_eq!(
        ast.compile().unwrap(),
        "\
struct uniform_buffer_object
{
    float4x4 u_model_view_projection;
};

ConstantBuffer<uniform_buffer_object> _22;

static float4 gl_Position;
static float3 v_normal;
static float3 a_normal;
static float4 a_position;

struct SPIRV_Cross_Input
{
    float4 a_position : TEXCOORD0;
    float3 a_normal : TEXCOORD1;
};

struct SPIRV_Cross_Output
{
    float3 v_normal : TEXCOORD0;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    v_normal = a_normal;
    gl_Position = mul(a_position, _22.u_model_view_projection);
}

SPIRV_Cross_Output main(SPIRV_Cross_Input stage_input)
{
    a_normal = stage_input.a_normal;
    a_position = stage_input.a_position;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.v_normal = v_normal;
    return stage_output;
}
"
    );
}
