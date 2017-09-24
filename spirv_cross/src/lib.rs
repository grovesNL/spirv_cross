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

pub mod compile {
    use ScInternal::root::*;
    use std::ptr;
    use std::ffi::CStr;

    pub enum ErrorCode {
        Unhandled = 1,
    }

    pub struct SpirvModule<'a> {
        pub ir: &'a [u32],
    }

    impl<'a> SpirvModule<'a> {
        pub fn new(ir: &[u32]) -> SpirvModule {
            SpirvModule { ir }
        }
    }

    pub struct HlslCompiler;

    impl HlslCompiler {
        pub fn new() -> HlslCompiler {
            HlslCompiler
        }
        pub fn compile(&self, module: &SpirvModule) -> Result<String, ErrorCode> {
            let ptr = module.ir.as_ptr() as *const u32;
            let mut compiler = ptr::null_mut();
            let hlsl;
            unsafe {
                check!(sc_internal_compiler_hlsl_new(
                    &mut compiler,
                    ptr,
                    module.ir.len() as usize
                ));
                let mut hlsl_ptr = ptr::null_mut();
                check!(sc_internal_compiler_hlsl_compile(compiler, &mut hlsl_ptr));
                hlsl = match CStr::from_ptr(hlsl_ptr).to_owned().into_string() {
                    Err(_) => return Err(ErrorCode::Unhandled),
                    Ok(v) => v,
                };
                check!(sc_internal_deallocate_string(hlsl_ptr));
                check!(sc_internal_compiler_hlsl_delete(compiler));
            };
            Ok(hlsl)
        }
    }
}
