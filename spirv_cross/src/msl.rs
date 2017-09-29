use super::ErrorCode;
use spirv;
use bindings::root::*;
use std::ptr;
use std::ffi::CStr;
use std::os::raw::c_void;

#[derive(Debug, Clone)]
pub struct CompilerVertexOptions {
    pub invert_y: bool,
    pub transform_clip_space: bool,
}

impl Default for CompilerVertexOptions {
    fn default() -> CompilerVertexOptions {
        CompilerVertexOptions {
            invert_y: false,
            transform_clip_space: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompilerOptions {
    pub vertex: CompilerVertexOptions,
}

impl CompilerOptions {
    fn as_raw(&self) -> ScMslCompilerOptions {
        ScMslCompilerOptions {
            vertex_invert_y: self.vertex.invert_y,
            vertex_transform_clip_space: self.vertex.transform_clip_space,
        }
    }
}

impl Default for CompilerOptions {
    fn default() -> CompilerOptions {
        CompilerOptions {
            vertex: CompilerVertexOptions::default(),
        }
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
        options: &CompilerOptions,
    ) -> Result<String, ErrorCode> {
        unsafe {
            let mut msl_ptr = ptr::null_mut();
            check!(sc_internal_compiler_msl_compile(
                parsed_module.ir.as_ptr() as *const u32,
                parsed_module.ir.len() as usize,
                &mut msl_ptr,
                &options.as_raw()
            ));
            let msl = match CStr::from_ptr(msl_ptr).to_owned().into_string() {
                Err(_) => return Err(ErrorCode::Unhandled),
                Ok(v) => v,
            };
            check!(sc_internal_free_pointer(msl_ptr as *mut c_void));
            Ok(msl)
        }
    }
}
