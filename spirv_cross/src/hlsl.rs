use super::{CompileTarget, ErrorCode};
use spirv;
use bindings::root::*;
use std::ptr;
use std::ffi::CStr;
use std::os::raw::c_void;

#[derive(Debug, Clone)]
pub struct CompilerOptions;

impl CompilerOptions {
    pub fn new() -> CompilerOptions {
        CompilerOptions
    }
}

pub(super) fn internal_delete_compiler_hlsl(internal_compiler: *mut c_void) {
    unsafe {
        if ScInternalResult::Success != sc_internal_compiler_hlsl_delete(internal_compiler) {
            panic!("Cannot delete compiler");
        }
    }
}

#[derive(Debug, Clone)]
pub struct Compiler;

impl Compiler {
    pub fn new() -> Compiler {
        Compiler
    }

    pub fn compile(
        &self,
        module: &spirv::ParsedModule,
        _options: &CompilerOptions,
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
            check!(sc_internal_free_pointer(hlsl_ptr as *mut c_void));
            Ok(hlsl)
        }
    }
}
