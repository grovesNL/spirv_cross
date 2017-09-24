extern crate spirv_cross;
use spirv_cross::compile::{HlslCompiler, SpirvModule};

fn ir_words_from_bytes(buf: &[u8]) -> &[u32] {
    unsafe {
        std::slice::from_raw_parts(
            buf.as_ptr() as *const u32,
            buf.len() / std::mem::size_of::<u32>(),
        )
    }
}

fn main() {
    let vert_module = SpirvModule::new(ir_words_from_bytes(include_bytes!("vertex.spv")));
    let hlsl_compiler = HlslCompiler::new();
    let hlsl = match hlsl_compiler.compile(&vert_module) {
        Err(e) => panic!(e),
        Ok(v) => v,
    };
    println!("{}", hlsl);
}
