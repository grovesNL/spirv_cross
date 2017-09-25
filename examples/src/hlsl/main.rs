extern crate spirv_cross;
use spirv_cross::compile::{HlslCompileOptions, HlslCompiler, SpirvParseOptions, SpirvModule};

fn ir_words_from_bytes(buf: &[u8]) -> &[u32] {
    unsafe {
        std::slice::from_raw_parts(
            buf.as_ptr() as *const u32,
            buf.len() / std::mem::size_of::<u32>(),
        )
    }
}

fn main() {
    let vertex_module = SpirvModule::new(ir_words_from_bytes(include_bytes!("vertex.spv")));
    let hlsl_compiler = HlslCompiler::new();
    let parsed_vertex_module = hlsl_compiler
        .parse(&vertex_module, &SpirvParseOptions::new())
        .unwrap();
    let hlsl = hlsl_compiler
        .compile(&parsed_vertex_module, &HlslCompileOptions::new())
        .unwrap();
    println!("{}", hlsl);
}
