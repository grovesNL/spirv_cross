extern crate examples;
extern crate spirv_cross;
use examples::words_from_bytes;
use spirv_cross::{hlsl, spirv};

fn main() {
    let module = spirv::Module::from_words(words_from_bytes(include_bytes!("../vertex.spv")));

    // Parse a SPIR-V module
    let mut ast = spirv::Ast::<hlsl::Target>::parse(&module).unwrap();
    ast.set_compiler_options(&hlsl::CompilerOptions {
        shader_model: hlsl::ShaderModel::V5_1,
        point_size_compat: false,
        point_coord_compat: false,
        vertex: hlsl::CompilerVertexOptions::default(),
        force_storage_buffer_as_uav: false,
        nonwritable_uav_texture_as_srv: false,
    })
    .unwrap();

    // List all entry points
    for entry_point in &ast.get_entry_points().unwrap() {
        println!("{:?}", entry_point);
    }

    // Compile to HLSL
    let shader = ast.compile().unwrap();
    println!("{}", shader);
}
