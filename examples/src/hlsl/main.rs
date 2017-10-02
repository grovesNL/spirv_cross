extern crate spirv_cross;
extern crate examples;
use spirv_cross::{hlsl, spirv};
use examples::words_from_bytes;

fn main() {
    let module = spirv::Module::from_words(words_from_bytes(include_bytes!("../vertex.spv")));

    // Parse a SPIR-V module
    let mut ast = spirv::Ast::<hlsl::Target>::parse(&module).unwrap();
    ast.set_compile_options(hlsl::CompilerOptions {
        shader_model: hlsl::ShaderModel::V5_1,
        vertex: hlsl::CompilerVertexOptions::default(),
    }).unwrap();

    // List all entry points
    for entry_point in &ast.get_entry_points().unwrap() {
        println!("{:?}", entry_point);
    }

    // Compile to HLSL
    let shader = ast.compile().unwrap();
    println!("{}", shader);
}
