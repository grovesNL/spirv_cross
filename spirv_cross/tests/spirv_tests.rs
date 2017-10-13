extern crate spirv_cross;
use spirv_cross::{hlsl as lang, spirv};

mod common;
use common::words_from_bytes;

#[test]
fn ast_gets_entry_points() {
    let parser_options = lang::ParserOptions::default();
    let module = spirv::Module::from_words(
        words_from_bytes(include_bytes!("shaders/simple.spv")),
    );
    let entry_points = spirv::Ast::<lang::Target>::parse(&module, &parser_options)
        .unwrap()
        .get_entry_points()
        .unwrap();

    assert_eq!(entry_points.len(), 1);
    assert_eq!(entry_points[0].name, "main");
}

#[test]
fn ast_gets_shader_resources() {
    let parser_options = lang::ParserOptions::default();
    let module = spirv::Module::from_words(
        words_from_bytes(include_bytes!("shaders/simple.spv")),
    );
    let shader_resources = spirv::Ast::<lang::Target>::parse(&module, &parser_options)
        .unwrap()
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

#[test]
fn ast_gets_decoration() {
    let parser_options = lang::ParserOptions::default();
    let module = spirv::Module::from_words(
        words_from_bytes(include_bytes!("shaders/simple.spv")),
    );
    let ast = spirv::Ast::<lang::Target>::parse(&module, &parser_options).unwrap();

    let stage_inputs = ast.get_shader_resources().unwrap().stage_inputs;
    let decoration = ast.get_decoration(stage_inputs[0].id, spirv::Decoration::DescriptorSet)
        .unwrap();
    assert_eq!(decoration, 0);
}

#[test]
fn ast_sets_decoration() {
    let parser_options = lang::ParserOptions::default();
    let module = spirv::Module::from_words(
        words_from_bytes(include_bytes!("shaders/simple.spv")),
    );
    let mut ast = spirv::Ast::<lang::Target>::parse(&module, &parser_options).unwrap();

    let stage_inputs = ast.get_shader_resources().unwrap().stage_inputs;
    let updated_value = 3;
    ast.set_decoration(
        stage_inputs[0].id,
        spirv::Decoration::DescriptorSet,
        updated_value,
    ).unwrap();
    assert_eq!(
        ast.get_decoration(stage_inputs[0].id, spirv::Decoration::DescriptorSet)
            .unwrap(),
        updated_value
    );
}
