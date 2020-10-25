extern crate examples;
extern crate spirv_cross;
use examples::words_from_bytes;
use spirv_cross::{hlsl, spirv};

fn main() {
    let module = spirv::Module::from_words(words_from_bytes(include_bytes!("../vertex.spv")));

    // Parse a SPIR-V module
    let mut ast = spirv::Ast::<hlsl::Target>::parse(&module).unwrap();
    let mut options = hlsl::CompilerOptions::default();
    options.shader_model = hlsl::ShaderModel::V5_1;
    options.point_size_compat = false;
    options.point_coord_compat = false;
    options.vertex = hlsl::CompilerVertexOptions::default();
    options.force_storage_buffer_as_uav = false;
    options.nonwritable_uav_texture_as_srv = false;
    options.force_zero_initialized_variables = true;
    options.entry_point = None;
    ast.set_compiler_options(&options).unwrap();

    // List all entry points
    for entry_point in &ast.get_entry_points().unwrap() {
        println!("{:?}", entry_point);
    }

    // Compile to HLSL
    let shader = ast.compile().unwrap();
    println!("{}", shader);
}
