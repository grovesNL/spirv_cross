extern crate spirv_cross;
use spirv_cross::{spirv, hlsl};

fn ir_words_from_bytes(buf: &[u8]) -> &[u32] {
    unsafe {
        std::slice::from_raw_parts(
            buf.as_ptr() as *const u32,
            buf.len() / std::mem::size_of::<u32>(),
        )
    }
}

fn main() {
    let vertex_module = spirv::Module::new(ir_words_from_bytes(include_bytes!("vertex.spv")));

    // Parse a SPIR-V module
    let parsed_vertex_module = spirv::Parser::new()
        .parse(&vertex_module, &spirv::ParserOptions::default())
        .unwrap();

    // List all entry points
    for entry_point in &parsed_vertex_module.entry_points {
        println!("{:?}", entry_point);
    }

    // Compile to HLSL
    let hlsl = hlsl::Compiler::new()
        .compile(&parsed_vertex_module, &hlsl::CompilerOptions::default())
        .unwrap();

    println!("{}", hlsl);
}
