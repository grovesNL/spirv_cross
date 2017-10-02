
//! Raw compiler bindings for SPIRV-Cross.

use bindings::root::*;
use ErrorCode;
use spirv;
use std::ptr;
use std::ffi::CStr;

impl spirv::ExecutionModel {
    fn from_raw(raw: spv::ExecutionModel) -> Result<spirv::ExecutionModel, ErrorCode> {
        use spirv::ExecutionModel::*;
        match raw {
            spv::ExecutionModel::ExecutionModelVertex => Ok(Vertex),
            spv::ExecutionModel::ExecutionModelTessellationControl => Ok(TessellationControl),
            spv::ExecutionModel::ExecutionModelTessellationEvaluation => Ok(TessellationEvaluation),
            spv::ExecutionModel::ExecutionModelGeometry => Ok(Geometry),
            spv::ExecutionModel::ExecutionModelFragment => Ok(Fragment),
            spv::ExecutionModel::ExecutionModelGLCompute => Ok(GlCompute),
            spv::ExecutionModel::ExecutionModelKernel => Ok(Kernel),
            _ => Err(ErrorCode::Unhandled),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Compiler {
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

    pub fn get_decoration(
        &self,
        id: u32,
        decoration: spv::Decoration,
    ) -> Result<Option<u32>, ErrorCode> {
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

    pub fn set_decoration(
        &mut self,
        id: u32,
        decoration: spv::Decoration,
        argument: u32,
    ) -> Result<(), ErrorCode> {
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

    pub fn get_entry_points(&self) -> Result<Vec<spirv::EntryPoint>, ErrorCode> {
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

                    let entry_point = spirv::EntryPoint {
                        name,
                        execution_model: try!(spirv::ExecutionModel::from_raw(
                            entry_point_raw.execution_model
                        )),
                        workgroup_size: spirv::WorkgroupSize {
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
        unsafe {
            sc_internal_compiler_delete(self.sc_compiler);
        }
    }
}
