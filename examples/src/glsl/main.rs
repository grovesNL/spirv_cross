extern crate examples;
extern crate spirv_cross;
use examples::words_from_bytes;
use spirv_cross::{glsl, spirv};

fn main() {
    let module = spirv::Module::from_words(words_from_bytes(include_bytes!("../vertex.spv")));

    // Parse a SPIR-V module
    let mut ast = spirv::Ast::<glsl::Target>::parse(&module).unwrap();
    ast.set_compiler_options(&glsl::CompilerOptions {
        version: glsl::Version::V4_60,
        no_420_pack_extension: false,
        vertex: glsl::CompilerVertexOptions::default(),
    })
    .unwrap();

    // List all entry points
    for entry_point in &ast.get_entry_points().unwrap() {
        println!("{:?}", entry_point);
    }

    // Compile to GLSL
    let shader = ast.compile().unwrap();
    println!("{}", shader);
}
