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
    pub(crate) ir: &'a [u32],
}

impl<'a> Module<'a> {
    pub fn new(ir: &[u32]) -> Module {
        Module { ir }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Compiler {
    pub sc_compiler: *mut ScInternalCompilerBase,
}

impl Compiler {
    pub fn compile(&self) -> Result<String, ErrorCode> {
        unsafe {
            let mut shader_ptr = ptr::null();
            check!(sc_internal_compiler_compile(
                self.sc_compiler,
                &mut shader_ptr,
            ));
            let shader = match CStr::from_ptr(shader_ptr).to_owned().into_string() {
                Err(_) => return Err(ErrorCode::Unhandled),
                Ok(v) => v,
            };
            check!(sc_internal_free_pointer(shader_ptr as *mut c_void));
            Ok(shader)
        }
    }

    pub fn get_decoration(&self, id: u32, decoration: spv::Decoration) -> Result<Option<u32>, ErrorCode> {
        let mut result = 0;
        unsafe {
            check!(sc_internal_compiler_get_decoration(
                self.sc_compiler,
                &mut result,
                id,
                decoration,
            ));
        }

        Ok(match result {
            0 => None,
            _ => Some(result),
        })
    }

    pub fn set_decoration(&self, id: u32, decoration: spv::Decoration, argument: u32) -> Result<(), ErrorCode> {
        unsafe {
            check!(sc_internal_compiler_set_decoration(
                self.sc_compiler,
                id,
                decoration,
                argument,
            ));
        }

        Ok(())
    }

    pub fn get_entry_points(&self) -> Result<Vec<EntryPoint>, ErrorCode> {
        let mut entry_points_raw = ptr::null_mut();
        let mut entry_points_raw_length = 0 as usize;

        unsafe {
            check!(sc_internal_compiler_get_entry_points(
                self.sc_compiler,
                &mut entry_points_raw,
                &mut entry_points_raw_length,
            ));

            let entry_points = (0..entry_points_raw_length)
                .map(|offset| {
                    let entry_point_raw_ptr = entry_points_raw.offset(offset as isize);
                    let entry_point_raw = *entry_point_raw_ptr;
                    let name = match CStr::from_ptr(entry_point_raw.name)
                        .to_owned()
                        .into_string()
                    {
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

            Ok(try!(entry_points))
        }
    }
}

impl Drop for Compiler {
    fn drop(&mut self) {
        unsafe { sc_internal_compiler_delete(self.sc_compiler); }
    }
}
