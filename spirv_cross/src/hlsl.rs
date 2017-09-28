use super::ErrorCode;
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

#[derive(Debug, Clone)]
pub struct Compiler {
    _unconstructable: (),
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            _unconstructable: (),
        }
    }

    pub fn compile(
        &self,
        parsed_module: &spirv::ParsedModule,
        _options: &CompilerOptions,
    ) -> Result<String, ErrorCode> {
        unsafe {
            let mut hlsl_ptr = ptr::null_mut();
            check!(sc_internal_compiler_hlsl_compile(
                parsed_module.ir.as_ptr() as *const u32,
                parsed_module.ir.len() as usize,
                &mut hlsl_ptr,
            ));
            let hlsl = match CStr::from_ptr(hlsl_ptr).to_owned().into_string() {
                Err(_) => return Err(ErrorCode::Unhandled),
                Ok(v) => v,
            };
            check!(sc_internal_free_pointer(hlsl_ptr as *mut c_void));
            Ok(hlsl)
        }
    }
}
