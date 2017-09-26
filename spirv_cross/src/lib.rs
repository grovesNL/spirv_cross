use ScInternal::root::*;
use std::ptr;
use std::ffi::CStr;
use std::os::raw::c_void;

mod ScInternal {
    #![allow(dead_code)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!("bindings.rs"));
}

macro_rules! check {
    ($check:expr) => {{
        let result = $check;
        if ScInternalResult::Success != result {
            return match result {
                _ => Err(ErrorCode::Unhandled)
            }
        }
    }}
}


#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ErrorCode {
    Unhandled = 1,
}

#[derive(Debug, Clone)]
pub struct SpirvModule<'a> {
    ir: &'a [u32],
}

impl<'a> SpirvModule<'a> {
    pub fn new(ir: &[u32]) -> SpirvModule {
        SpirvModule { ir }
    }
}

pub struct ParsedSpirvModule {
    // TODO: Temporarily keep reference to compiler to share between parse/compile steps
    internal_compiler: *mut c_void,
    internal_delete_compiler: fn(*mut c_void),
    compile_target: CompileTarget,
}

impl Drop for ParsedSpirvModule {
    fn drop(&mut self) {
        (self.internal_delete_compiler)(self.internal_compiler);
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CompileTarget {
    Hlsl,
}

#[derive(Debug, Clone)]
pub struct SpirvParserOptions;

impl SpirvParserOptions {
    pub fn new() -> SpirvParserOptions {
        SpirvParserOptions
    }
}

#[derive(Debug, Clone)]
pub struct SpirvParser {
    compile_target: CompileTarget,
}

impl SpirvParser {
    // TODO: Remove `compile_target`, see https://github.com/KhronosGroup/SPIRV-Cross/issues/287
    pub fn new(compile_target: CompileTarget) -> SpirvParser {
        SpirvParser { compile_target }
    }

    pub fn parse(
        &self,
        module: &SpirvModule,
        _options: &SpirvParserOptions,
    ) -> Result<ParsedSpirvModule, ErrorCode> {
        let ptr = module.ir.as_ptr() as *const u32;
        let mut compiler = ptr::null_mut();
        match self.compile_target {
            CompileTarget::Hlsl => unsafe {
                check!(sc_internal_compiler_hlsl_new(
                    &mut compiler,
                    ptr,
                    module.ir.len() as usize,
                ));

                Ok(ParsedSpirvModule {
                    internal_compiler: compiler,
                    internal_delete_compiler: internal_delete_compiler_hlsl,
                    compile_target: self.compile_target.clone(),
                })
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct HlslCompilerOptions;

impl HlslCompilerOptions {
    pub fn new() -> HlslCompilerOptions {
        HlslCompilerOptions
    }
}

fn internal_delete_compiler_hlsl(internal_compiler: *mut c_void) {
    unsafe {
        if ScInternalResult::Success != sc_internal_compiler_hlsl_delete(internal_compiler) {
            panic!("Cannot delete compiler");
        }
    }
}

#[derive(Debug, Clone)]
pub struct HlslCompiler;

impl HlslCompiler {
    pub fn new() -> HlslCompiler {
        HlslCompiler
    }

    pub fn compile(
        &self,
        module: &ParsedSpirvModule,
        _options: &HlslCompilerOptions,
    ) -> Result<String, ErrorCode> {
        if module.compile_target != CompileTarget::Hlsl {
            return Err(ErrorCode::Unhandled);
        }
        let compiler = module.internal_compiler;
        unsafe {
            let mut hlsl_ptr = ptr::null_mut();
            check!(sc_internal_compiler_hlsl_compile(compiler, &mut hlsl_ptr));
            let hlsl = match CStr::from_ptr(hlsl_ptr).to_owned().into_string() {
                Err(_) => return Err(ErrorCode::Unhandled),
                Ok(v) => v,
            };
            check!(sc_internal_deallocate_string(hlsl_ptr));
            Ok(hlsl)
        }
    }
}
