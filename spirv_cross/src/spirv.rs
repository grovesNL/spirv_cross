use super::{CompileTarget, ErrorCode};
use hlsl;
use bindings::root::*;
use std::ptr;
use std::os::raw::c_void;
use std::ffi::CStr;

pub use bindings::root::spv::ExecutionModel;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct WorkgroupSize {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(Debug, Clone)]
pub struct EntryPoint {
    pub name: String,
    pub execution_model: ExecutionModel,
    pub workgroup_size: WorkgroupSize,
}

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

impl ParsedModule {
    pub fn get_entry_points(&self) -> Result<Vec<EntryPoint>, ErrorCode> {
        unsafe {
            let mut entry_points = ptr::null_mut();
            let mut entry_points_length = 0 as usize;

            check!(sc_internal_compiler_base_get_entry_points(
                self.internal_compiler,
                &mut entry_points,
                &mut entry_points_length,
            ));

            (0..entry_points_length)
                .map(|offset| {
                    let ep_ptr = entry_points.offset(offset as isize);
                    let ep = *ep_ptr;
                    let name = match CStr::from_ptr(ep.name).to_owned().into_string() {
                        Ok(n) => n,
                        _ => return Err(ErrorCode::Unhandled),
                    };

                    let entry_point = EntryPoint {
                        name,
                        execution_model: ep.execution_model,
                        workgroup_size: WorkgroupSize {
                            x: ep.workgroup_size_x,
                            y: ep.workgroup_size_y,
                            z: ep.workgroup_size_z,
                        },
                    };

                    check!(sc_internal_free_pointer(ep.name as *mut c_void));
                    check!(sc_internal_free_pointer(ep_ptr as *mut c_void));

                    Ok(entry_point)
                })
                .collect::<Result<Vec<_>, _>>()
        }
    }
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
