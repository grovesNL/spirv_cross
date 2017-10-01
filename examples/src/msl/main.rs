extern crate spirv_cross;
use spirv_cross::{spirv, msl};

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
    let ast = spirv::Ast::parse(&vertex_module, spirv::Target::Msl).unwrap();

    // List all entry points
    for entry_point in &ast.get_entry_points().unwrap() {
        println!("{:?}", entry_point);
    }

    // Compile to MSL
    let msl = msl::Compiler::from_ast(&ast)
        .compile(&msl::CompilerOptions::default())
        .unwrap();

    println!("{}", msl);
}
