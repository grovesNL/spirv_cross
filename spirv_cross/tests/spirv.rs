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
