extern crate spirv_cross;
use spirv_cross::{hlsl, spirv};

mod common;
use common::words_from_bytes;

#[test]
fn ast_gets_entry_points() {
    let entry_points = spirv::Ast::<hlsl::Target>::parse(&spirv::Module::from_words(
        words_from_bytes(include_bytes!("shaders/simple.spv")),
    )).unwrap()
        .get_entry_points()
        .unwrap();

    assert_eq!(entry_points.len(), 1);
    assert_eq!(entry_points[0].name, "main");
}

#[test]
fn ast_gets_shader_resources() {
    let shader_resources = spirv::Ast::<hlsl::Target>::parse(&spirv::Module::from_words(
        words_from_bytes(include_bytes!("shaders/simple.spv")),
    )).unwrap()
        .get_shader_resources()
        .unwrap();

    let spirv::ShaderResources {
        uniform_buffers,
        stage_inputs,
        stage_outputs,
        ..
    } = shader_resources;

    assert_eq!(uniform_buffers.len(), 1);
    assert_eq!(uniform_buffers[0].name, "uniform_buffer_object");
    assert_eq!(shader_resources.storage_buffers.len(), 0);
    assert_eq!(stage_inputs.len(), 2);
    assert!(
        stage_inputs
            .iter()
            .any(|stage_input| { stage_input.name == "a_normal" })
    );
    assert!(
        stage_inputs
            .iter()
            .any(|stage_input| { stage_input.name == "a_position" })
    );
    assert_eq!(stage_outputs.len(), 1);
    assert!(
        stage_outputs
            .iter()
            .any(|stage_output| { stage_output.name == "v_normal" })
    );
    assert_eq!(shader_resources.subpass_inputs.len(), 0);
    assert_eq!(shader_resources.storage_images.len(), 0);
    assert_eq!(shader_resources.sampled_images.len(), 0);
    assert_eq!(shader_resources.atomic_counters.len(), 0);
    assert_eq!(shader_resources.push_constant_buffers.len(), 0);
    assert_eq!(shader_resources.separate_images.len(), 0);
    assert_eq!(shader_resources.separate_samplers.len(), 0);
}
