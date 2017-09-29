use super::ErrorCode;
use bindings::root::*;
use std::ptr;
use std::os::raw::c_void;
use std::ffi::CStr;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ExecutionModel {
    Vertex = 0,
    TessellationControl = 1,
    TessellationEvaluation = 2,
    Geometry = 3,
    Fragment = 4,
    GlCompute = 5,
    Kernel = 6,
}

impl ExecutionModel {
    fn from_raw(raw: spv::ExecutionModel) -> Result<ExecutionModel, ErrorCode> {
        match raw {
            spv::ExecutionModel::ExecutionModelVertex => Ok(ExecutionModel::Vertex),
            spv::ExecutionModel::ExecutionModelTessellationControl => {
                Ok(ExecutionModel::TessellationControl)
            }
            spv::ExecutionModel::ExecutionModelTessellationEvaluation => {
                Ok(ExecutionModel::TessellationEvaluation)
            }
            spv::ExecutionModel::ExecutionModelGeometry => Ok(ExecutionModel::Geometry),
            spv::ExecutionModel::ExecutionModelFragment => Ok(ExecutionModel::Fragment),
            spv::ExecutionModel::ExecutionModelGLCompute => Ok(ExecutionModel::GlCompute),
            spv::ExecutionModel::ExecutionModelKernel => Ok(ExecutionModel::Kernel),
            _ => Err(ErrorCode::Unhandled),
        }
    }
}

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
    pub entry_points: Vec<EntryPoint>,
    pub(crate) ir: Vec<u32>,
}

impl ParsedModule {
    fn new(ir: Vec<u32>, entry_points: Vec<EntryPoint>) -> ParsedModule {
        ParsedModule { entry_points, ir }
    }
}

#[derive(Debug, Clone)]
pub struct ParserOptions;

impl Default for ParserOptions {
    fn default() -> ParserOptions {
        ParserOptions
    }
}

#[derive(Debug, Clone)]
pub struct Parser {
    _unconstructable: (),
}

impl Parser {
    pub fn new() -> Parser {
        Parser { _unconstructable: () }
    }

    pub fn parse(
        &self,
        module: &Module,
        _options: &ParserOptions,
    ) -> Result<ParsedModule, ErrorCode> {
        unsafe {
            let mut entry_points_raw = ptr::null_mut();
            let mut entry_points_raw_length = 0 as usize;

            let mut ir = vec![0; module.ir.len()];
            ir.copy_from_slice(module.ir);

            check!(sc_internal_compiler_base_parse(
                ir.as_ptr() as *const u32,
                ir.len() as usize,
                &mut entry_points_raw,
                &mut entry_points_raw_length,
            ));

            let entry_points = (0..entry_points_raw_length)
                .map(|offset| {
                    let entry_point_raw_ptr = entry_points_raw.offset(offset as isize);
                    let entry_point_raw = *entry_point_raw_ptr;
                    let name = match CStr::from_ptr(entry_point_raw.name)
                        .to_owned()
                        .into_string() {
                        Ok(n) => n,
                        _ => return Err(ErrorCode::Unhandled),
                    };

                    let entry_point = EntryPoint {
                        name,
                        execution_model: try!(
                            ExecutionModel::from_raw(entry_point_raw.execution_model)
                        ),
                        workgroup_size: WorkgroupSize {
                            x: entry_point_raw.workgroup_size_x,
                            y: entry_point_raw.workgroup_size_y,
                            z: entry_point_raw.workgroup_size_z,
                        },
                    };

                    check!(sc_internal_free_pointer(
                        entry_point_raw.name as *mut c_void,
                    ));
                    check!(sc_internal_free_pointer(entry_point_raw_ptr as *mut c_void));

                    Ok(entry_point)
                })
                .collect::<Result<Vec<_>, _>>();

            Ok(ParsedModule::new(ir, try!(entry_points)))
        }
    }
}
