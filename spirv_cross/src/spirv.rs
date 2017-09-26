use super::{CompileTarget, ErrorCode};
use hlsl;
use bindings::root::*;
use std::ptr;
use std::os::raw::c_void;

#[derive(Debug, Clone)]
pub struct Module<'a> {
    ir: &'a [u32],
}

impl<'a> Module<'a> {
    pub fn new(ir: &[u32]) -> Module {
        Module { ir }
    }
}

pub struct ParsedModule {
    // TODO: Temporarily keep reference to compiler to share between parse/compile steps
    pub(crate) internal_compiler: *mut c_void,
    pub(crate) internal_delete_compiler: fn(*mut c_void),
    pub(crate) compile_target: CompileTarget,
}

impl Drop for ParsedModule {
    fn drop(&mut self) {
        (self.internal_delete_compiler)(self.internal_compiler);
    }
}

#[derive(Debug, Clone)]
pub struct ParserOptions;

impl ParserOptions {
    pub fn new() -> ParserOptions {
        ParserOptions
    }
}

#[derive(Debug, Clone)]
pub struct Parser {
    compile_target: CompileTarget,
}

impl Parser {
    // TODO: Remove `compile_target`, see https://github.com/KhronosGroup/SPIRV-Cross/issues/287
    pub fn new(compile_target: CompileTarget) -> Parser {
        Parser { compile_target }
    }

    pub fn parse(
        &self,
        module: &Module,
        _options: &ParserOptions,
    ) -> Result<ParsedModule, ErrorCode> {
        let ptr = module.ir.as_ptr() as *const u32;
        let mut compiler = ptr::null_mut();
        match self.compile_target {
            CompileTarget::Hlsl => unsafe {
                check!(sc_internal_compiler_hlsl_new(
                    &mut compiler,
                    ptr,
                    module.ir.len() as usize,
                ));

                Ok(ParsedModule {
                    internal_compiler: compiler,
                    internal_delete_compiler: hlsl::internal_delete_compiler_hlsl,
                    compile_target: self.compile_target.clone(),
                })
            },
        }
    }
}
