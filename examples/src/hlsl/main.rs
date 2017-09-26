extern crate spirv_cross;
use spirv_cross::spirv;
use spirv_cross::hlsl;
use spirv_cross::CompileTarget;

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
    let parsed_vertex_module = spirv::Parser::new(CompileTarget::Hlsl)
        .parse(&vertex_module, &spirv::ParserOptions::new())
        .unwrap();
    let hlsl = hlsl::Compiler::new()
        .compile(&parsed_vertex_module, &hlsl::CompilerOptions::new())
        .unwrap();
    println!("{}", hlsl);
}
