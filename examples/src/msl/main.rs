extern crate spirv_cross;
extern crate examples;
use spirv_cross::{msl, spirv};
use examples::words_from_bytes;

fn main() {
    let module = spirv::Module::from_words(words_from_bytes(include_bytes!("../vertex.spv")));

    // Build parser options with some overrides
    let mut parser_options = msl::ParserOptions::default();
    parser_options.vertex_attribute_overrides.insert(
        msl::VertexAttributeLocation(1),
        msl::VertexAttribute {
            buffer_id: 1,
            offset: 2,
            stride: 3,
            step: spirv::VertexAttributeStep::Instance,
            force_used: true,
        },
    );
    parser_options.resource_binding_overrides.insert(
        msl::ResourceBindingLocation {
            stage: spirv::ExecutionModel::Vertex,
            desc_set: 0,
            binding: 0,
        },
        msl::ResourceBinding {
            resource_id: 5,
            force_used: false,
        },
    );

    // Parse a SPIR-V module
    let ast = spirv::Ast::<msl::Target>::parse(&module, &parser_options).unwrap();

    // List all entry points
    for entry_point in &ast.get_entry_points().unwrap() {
        println!("{:?}", entry_point);
    }

    // Compile to MSL
    let shader = ast.compile().unwrap();
    println!("{}", shader);
}
