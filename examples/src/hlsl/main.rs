extern crate spirv_cross;
use spirv_cross::{CompileTarget, HlslCompiler, HlslCompilerOptions, SpirvModule, SpirvParser,
                  SpirvParserOptions};

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
    let parsed_vertex_module = SpirvParser::new(CompileTarget::Hlsl)
        .parse(&vertex_module, &SpirvParserOptions::new())
        .unwrap();
    let hlsl = HlslCompiler::new()
        .compile(&parsed_vertex_module, &HlslCompilerOptions::new())
        .unwrap();
    println!("{}", hlsl);
}
