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
    use std::os::raw::c_void;

    #[derive(Debug)]
    pub enum ErrorCode {
        Unhandled = 1,
    }

    pub struct SpirvModule<'a> {
        ir: &'a [u32],
    }

    impl<'a> SpirvModule<'a> {
        pub fn new(ir: &[u32]) -> SpirvModule {
            SpirvModule { ir }
        }
    }

    pub struct ParsedSpirvModule {
        // TODO: Currently parsing and compilation must occur with the same compiler instance
        // (i.e. as opposed to splitting the parser/compiler which would be the ideal)
        // So temporarily we create the compiler instance during parse and reuse it during compilation
        // See https://github.com/KhronosGroup/SPIRV-Cross/issues/287
        internal_compiler: *mut c_void,
        internal_delete_compiler: fn(*mut c_void),
    }

    impl Drop for ParsedSpirvModule {
        fn drop(&mut self) {
            (self.internal_delete_compiler)(self.internal_compiler);
        }
    }

    pub struct HlslParseOptions;

    impl HlslParseOptions {
        pub fn new() -> HlslParseOptions {
            HlslParseOptions
        }
    }

    pub struct HlslCompileOptions;

    impl HlslCompileOptions {
        pub fn new() -> HlslCompileOptions {
            HlslCompileOptions
        }
    }

    pub struct HlslCompiler;

    fn internal_delete_compiler_hlsl(internal_compiler: *mut c_void) {
        unsafe {
            if ScInternalResult::Success != sc_internal_compiler_hlsl_delete(internal_compiler) {
                panic!("Cannot delete compiler");
            }
        }
    }

    impl HlslCompiler {
        pub fn new() -> HlslCompiler {
            HlslCompiler
        }

        pub fn parse(
            &self,
            module: &SpirvModule,
            _options: &HlslParseOptions,
        ) -> Result<ParsedSpirvModule, ErrorCode> {
            let ptr = module.ir.as_ptr() as *const u32;
            let mut compiler = ptr::null_mut();
            unsafe {
                check!(sc_internal_compiler_hlsl_new(
                    &mut compiler,
                    ptr,
                    module.ir.len() as usize
                ));

                Ok(ParsedSpirvModule {
                    internal_compiler: compiler,
                    internal_delete_compiler: internal_delete_compiler_hlsl,
                })
            }
        }

        pub fn compile(
            &self,
            module: &ParsedSpirvModule,
            _options: &HlslCompileOptions,
        ) -> Result<String, ErrorCode> {
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
}
