//! Raw compiler bindings for SPIRV-Cross.

use bindings::root::*;
use spirv::{self, Decoration, Type};
use std::ffi::{CStr, CString};
use std::{mem, ptr, slice};
use ErrorCode;

impl spirv::ExecutionModel {
    fn from_raw(raw: spv::ExecutionModel) -> Result<Self, ErrorCode> {
        use self::spv::ExecutionModel as Em;
        use spirv::ExecutionModel::*;
        match raw {
            Em::ExecutionModelVertex => Ok(Vertex),
            Em::ExecutionModelTessellationControl => Ok(TessellationControl),
            Em::ExecutionModelTessellationEvaluation => Ok(TessellationEvaluation),
            Em::ExecutionModelGeometry => Ok(Geometry),
            Em::ExecutionModelFragment => Ok(Fragment),
            Em::ExecutionModelGLCompute => Ok(GlCompute),
            Em::ExecutionModelKernel => Ok(Kernel),
            _ => Err(ErrorCode::Unhandled),
        }
    }

    pub(crate) fn as_raw(&self) -> spv::ExecutionModel {
        use self::spv::ExecutionModel as Em;
        use spirv::ExecutionModel::*;
        match *self {
            Vertex => Em::ExecutionModelVertex,
            TessellationControl => Em::ExecutionModelTessellationControl,
            TessellationEvaluation => Em::ExecutionModelTessellationEvaluation,
            Geometry => Em::ExecutionModelGeometry,
            Fragment => Em::ExecutionModelFragment,
            GlCompute => Em::ExecutionModelGLCompute,
            Kernel => Em::ExecutionModelKernel,
        }
    }
}

impl spirv::Decoration {
    fn as_raw(&self) -> spv::Decoration {
        match *self {
            Decoration::RelaxedPrecision => spv::Decoration::DecorationRelaxedPrecision,
            Decoration::SpecId => spv::Decoration::DecorationSpecId,
            Decoration::Block => spv::Decoration::DecorationBlock,
            Decoration::BufferBlock => spv::Decoration::DecorationBufferBlock,
            Decoration::RowMajor => spv::Decoration::DecorationRowMajor,
            Decoration::ColMajor => spv::Decoration::DecorationColMajor,
            Decoration::ArrayStride => spv::Decoration::DecorationArrayStride,
            Decoration::MatrixStride => spv::Decoration::DecorationMatrixStride,
            Decoration::GlslShared => spv::Decoration::DecorationGLSLShared,
            Decoration::GlslPacked => spv::Decoration::DecorationGLSLPacked,
            Decoration::CPacked => spv::Decoration::DecorationCPacked,
            Decoration::BuiltIn => spv::Decoration::DecorationBuiltIn,
            Decoration::NoPerspective => spv::Decoration::DecorationNoPerspective,
            Decoration::Flat => spv::Decoration::DecorationFlat,
            Decoration::Patch => spv::Decoration::DecorationPatch,
            Decoration::Centroid => spv::Decoration::DecorationCentroid,
            Decoration::Sample => spv::Decoration::DecorationSample,
            Decoration::Invariant => spv::Decoration::DecorationInvariant,
            Decoration::Restrict => spv::Decoration::DecorationRestrict,
            Decoration::Aliased => spv::Decoration::DecorationAliased,
            Decoration::Volatile => spv::Decoration::DecorationVolatile,
            Decoration::Constant => spv::Decoration::DecorationConstant,
            Decoration::Coherent => spv::Decoration::DecorationCoherent,
            Decoration::NonWritable => spv::Decoration::DecorationNonWritable,
            Decoration::NonReadable => spv::Decoration::DecorationNonReadable,
            Decoration::Uniform => spv::Decoration::DecorationUniform,
            Decoration::SaturatedConversion => spv::Decoration::DecorationSaturatedConversion,
            Decoration::Stream => spv::Decoration::DecorationStream,
            Decoration::Location => spv::Decoration::DecorationLocation,
            Decoration::Component => spv::Decoration::DecorationComponent,
            Decoration::Index => spv::Decoration::DecorationIndex,
            Decoration::Binding => spv::Decoration::DecorationBinding,
            Decoration::DescriptorSet => spv::Decoration::DecorationDescriptorSet,
            Decoration::Offset => spv::Decoration::DecorationOffset,
            Decoration::XfbBuffer => spv::Decoration::DecorationXfbBuffer,
            Decoration::XfbStride => spv::Decoration::DecorationXfbStride,
            Decoration::FuncParamAttr => spv::Decoration::DecorationFuncParamAttr,
            Decoration::FpRoundingMode => spv::Decoration::DecorationFPRoundingMode,
            Decoration::FpFastMathMode => spv::Decoration::DecorationFPFastMathMode,
            Decoration::LinkageAttributes => spv::Decoration::DecorationLinkageAttributes,
            Decoration::NoContraction => spv::Decoration::DecorationNoContraction,
            Decoration::InputAttachmentIndex => spv::Decoration::DecorationInputAttachmentIndex,
            Decoration::Alignment => spv::Decoration::DecorationAlignment,
            Decoration::OverrideCoverageNv => spv::Decoration::DecorationOverrideCoverageNV,
            Decoration::PassthroughNv => spv::Decoration::DecorationPassthroughNV,
            Decoration::ViewportRelativeNv => spv::Decoration::DecorationViewportRelativeNV,
            Decoration::SecondaryViewportRelativeNv => {
                spv::Decoration::DecorationSecondaryViewportRelativeNV
            }
        }
    }
}

impl spirv::Type {
    pub(crate) fn from_raw(
        ty: spirv_cross::SPIRType_BaseType,
        member_types: Vec<u32>,
        array: Vec<u32>,
    ) -> Type {
        use bindings::root::spirv_cross::SPIRType_BaseType as b;
        use spirv::Type::*;
        match ty {
            b::Unknown => Unknown,
            b::Void => Void,
            b::Boolean => Boolean { array },
            b::Char => Char { array },
            b::Int => Int { array },
            b::UInt => UInt { array },
            b::Int64 => Int64 { array },
            b::UInt64 => UInt64 { array },
            b::AtomicCounter => AtomicCounter { array },
            b::Half => Half { array },
            b::Float => Float { array },
            b::Double => Double { array },
            b::Struct => Struct {
                member_types,
                array,
            },
            b::Image => Image { array },
            b::SampledImage => SampledImage { array },
            b::Sampler => Sampler { array },
            b::SByte => SByte { array },
            b::UByte => UByte { array },
            b::Short => Short { array },
            b::UShort => UShort { array },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Compiler<TTargetData> {
    pub(crate) sc_compiler: *mut ScInternalCompilerBase,
    pub(crate) target_data: TTargetData,
    pub(crate) has_been_compiled: bool,
}

impl<TTargetData> Compiler<TTargetData> {
    pub fn compile(&mut self) -> Result<String, ErrorCode> {
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

    pub fn get_decoration(&self, id: u32, decoration: spirv::Decoration) -> Result<u32, ErrorCode> {
        let mut result = 0;
        unsafe {
            check!(sc_internal_compiler_get_decoration(
                self.sc_compiler,
                &mut result,
                id,
                decoration.as_raw(),
            ));
        }
        Ok(result)
    }

    pub fn set_name(&mut self, id: u32, name: &str) -> Result<(), ErrorCode> {
        let name = CString::new(name);
        unsafe {
            match name {
                Ok(name) => {
                    check!(sc_internal_compiler_set_name(
                        self.sc_compiler,
                        id,
                        name.as_ptr(),
                    ));
                }
                _ => return Err(ErrorCode::Unhandled),
            }
        }
        Ok(())
    }

    pub fn unset_decoration(
        &mut self,
        id: u32,
        decoration: spirv::Decoration,
    ) -> Result<(), ErrorCode> {
        unsafe {
            check!(sc_internal_compiler_unset_decoration(
                self.sc_compiler,
                id,
                decoration.as_raw(),
            ));
        }

        Ok(())
    }

    pub fn set_decoration(
        &mut self,
        id: u32,
        decoration: spirv::Decoration,
        argument: u32,
    ) -> Result<(), ErrorCode> {
        unsafe {
            check!(sc_internal_compiler_set_decoration(
                self.sc_compiler,
                id,
                decoration.as_raw(),
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
                        work_group_size: spirv::WorkGroupSize {
                            x: entry_point_raw.work_group_size_x,
                            y: entry_point_raw.work_group_size_y,
                            z: entry_point_raw.work_group_size_z,
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

    pub fn get_cleansed_entry_point_name(
        &self,
        entry_point_name: &str,
        execution_model: spirv::ExecutionModel,
    ) -> Result<String, ErrorCode> {
        let mut cleansed_ptr = ptr::null();
        let entry_point = CString::new(entry_point_name);
        match entry_point {
            Ok(ep) => unsafe {
                check!(sc_internal_compiler_get_cleansed_entry_point_name(
                    self.sc_compiler,
                    ep.as_ptr(),
                    execution_model.as_raw(),
                    &mut cleansed_ptr
                ));
                let cleansed = match CStr::from_ptr(cleansed_ptr).to_str() {
                    Ok(c) => c.to_owned(),
                    _ => return Err(ErrorCode::Unhandled),
                };
                check!(sc_internal_free_pointer(cleansed_ptr as *mut c_void));
                Ok(cleansed)
            },
            _ => return Err(ErrorCode::Unhandled),
        }
    }

    pub fn get_specialization_constants(
        &self,
    ) -> Result<Vec<spirv::SpecializationConstant>, ErrorCode> {
        let mut constants_raw = ptr::null_mut();
        let mut constants_raw_length = 0 as usize;

        unsafe {
            check!(sc_internal_compiler_get_specialization_constants(
                self.sc_compiler,
                &mut constants_raw,
                &mut constants_raw_length,
            ));

            let constants = (0..constants_raw_length)
                .map(|offset| {
                    let constant_raw_ptr = constants_raw.offset(offset as isize);
                    let constant_raw = *constant_raw_ptr;

                    let constant = spirv::SpecializationConstant {
                        id: constant_raw.id,
                        constant_id: constant_raw.constant_id,
                    };

                    Ok(constant)
                })
                .collect::<Result<Vec<_>, _>>();

            check!(sc_internal_free_pointer(constants_raw as *mut c_void));

            Ok(try!(constants))
        }
    }

    pub fn set_scalar_constant(&self, id: u32, value: u64) -> Result<(), ErrorCode> {
        unsafe {
            check!(sc_internal_compiler_set_scalar_constant(
                self.sc_compiler,
                id,
                value,
            ));
        }

        Ok(())
    }

    pub fn get_type(&self, id: u32) -> Result<spirv::Type, ErrorCode> {
        unsafe {
            let mut type_ptr = ptr::null();

            check!(sc_internal_compiler_get_type(
                self.sc_compiler,
                id,
                &mut type_ptr,
            ));

            let raw = *type_ptr;

            let member_types =
                slice::from_raw_parts(raw.member_types, raw.member_types_size).to_vec();
            let array = slice::from_raw_parts(raw.array, raw.array_size).to_vec();
            let result = Type::from_raw(raw.type_, member_types, array);

            if raw.member_types_size > 0 {
                check!(sc_internal_free_pointer(raw.member_types as *mut c_void));
            }
            if raw.array_size > 0 {
                check!(sc_internal_free_pointer(raw.array as *mut c_void));
            }
            check!(sc_internal_free_pointer(type_ptr as *mut c_void));

            Ok(result)
        }
    }

    pub fn get_member_name(&self, id: u32, index: u32) -> Result<String, ErrorCode> {
        unsafe {
            let mut name_ptr = ptr::null();
            check!(sc_internal_compiler_get_member_name(
                self.sc_compiler,
                id,
                index,
                &mut name_ptr,
            ));
            let name = match CStr::from_ptr(name_ptr).to_owned().into_string() {
                Err(_) => return Err(ErrorCode::Unhandled),
                Ok(n) => n,
            };
            check!(sc_internal_free_pointer(name_ptr as *mut c_void));
            Ok(name)
        }
    }

    pub fn get_member_decoration(
        &self,
        id: u32,
        index: u32,
        decoration: Decoration,
    ) -> Result<u32, ErrorCode> {
        let mut result = 0;
        unsafe {
            check!(sc_internal_compiler_get_member_decoration(
                self.sc_compiler,
                id,
                index,
                decoration.as_raw(),
                &mut result,
            ));
        }
        Ok(result)
    }

    pub fn set_member_decoration(
        &self,
        id: u32,
        index: u32,
        decoration: Decoration,
        argument: u32,
    ) -> Result<(), ErrorCode> {
        unsafe {
            check!(sc_internal_compiler_set_member_decoration(
                self.sc_compiler,
                id,
                index,
                decoration.as_raw(),
                argument,
            ));
        }

        Ok(())
    }

    pub fn get_declared_struct_size(&self, id: u32) -> Result<u32, ErrorCode> {
        let mut result = 0;
        unsafe {
            check!(sc_internal_compiler_get_declared_struct_size(
                self.sc_compiler,
                id,
                &mut result,
            ));
        }
        Ok(result)
    }

    pub fn get_declared_struct_member_size(&self, id: u32, index: u32) -> Result<u32, ErrorCode> {
        let mut result = 0;
        unsafe {
            check!(sc_internal_compiler_get_declared_struct_member_size(
                self.sc_compiler,
                id,
                index,
                &mut result,
            ));
        }
        Ok(result)
    }

    pub fn get_shader_resources(&self) -> Result<spirv::ShaderResources, ErrorCode> {
        unsafe {
            let mut shader_resources_raw = mem::zeroed();
            check!(sc_internal_compiler_get_shader_resources(
                self.sc_compiler,
                &mut shader_resources_raw,
            ));

            let fill_resources = |array_raw: &ScResourceArray| {
                let resources_raw = slice::from_raw_parts(array_raw.data, array_raw.num);
                let resources = resources_raw
                    .iter()
                    .map(|resource_raw| {
                        let name = match CStr::from_ptr(resource_raw.name).to_owned().into_string()
                        {
                            Ok(n) => n,
                            _ => return Err(ErrorCode::Unhandled),
                        };

                        check!(sc_internal_free_pointer(resource_raw.name as *mut c_void,));

                        Ok(spirv::Resource {
                            id: resource_raw.id,
                            type_id: resource_raw.type_id,
                            base_type_id: resource_raw.base_type_id,
                            name,
                        })
                    })
                    .collect::<Result<Vec<_>, ErrorCode>>();

                check!(sc_internal_free_pointer(array_raw.data as *mut c_void,));

                resources
            };

            let uniform_buffers = fill_resources(&shader_resources_raw.uniform_buffers)?;
            let storage_buffers = fill_resources(&shader_resources_raw.storage_buffers)?;
            let stage_inputs = fill_resources(&shader_resources_raw.stage_inputs)?;
            let stage_outputs = fill_resources(&shader_resources_raw.stage_outputs)?;
            let subpass_inputs = fill_resources(&shader_resources_raw.subpass_inputs)?;
            let storage_images = fill_resources(&shader_resources_raw.storage_images)?;
            let sampled_images = fill_resources(&shader_resources_raw.sampled_images)?;
            let atomic_counters = fill_resources(&shader_resources_raw.atomic_counters)?;
            let push_constant_buffers =
                fill_resources(&shader_resources_raw.push_constant_buffers)?;
            let separate_images = fill_resources(&shader_resources_raw.separate_images)?;
            let separate_samplers = fill_resources(&shader_resources_raw.separate_samplers)?;

            Ok(spirv::ShaderResources {
                uniform_buffers,
                storage_buffers,
                stage_inputs,
                stage_outputs,
                subpass_inputs,
                storage_images,
                sampled_images,
                atomic_counters,
                push_constant_buffers,
                separate_images,
                separate_samplers,
            })
        }
    }

    pub fn rename_interface_variable(
        &self,
        resources: &[spirv::Resource],
        location: u32,
        new_name: &str,
    ) -> Result<(), ErrorCode> {
        unsafe {
            let mut resources_names = Vec::new();
            for resource in resources.iter() {
                match CString::new(&*resource.name) {
                    Ok(rn) => resources_names.push(rn),
                    Err(_) => return Err(ErrorCode::Unhandled),
                }
            }

            match CString::new(new_name) {
                Ok(n) => {
                    check!(sc_internal_compiler_rename_interface_variable(
                        self.sc_compiler,
                        resources
                            .iter()
                            .enumerate()
                            .map(|(i, r)| ScResource {
                                id: r.id,
                                type_id: r.type_id,
                                base_type_id: r.base_type_id,
                                name: resources_names[i].as_ptr() as _,
                            })
                            .collect::<Vec<_>>()
                            .as_ptr(),
                        resources_names.len(),
                        location,
                        n.as_ptr()
                    ));
                }
                Err(_) => return Err(ErrorCode::Unhandled),
            }

            Ok(())
        }
    }

    pub fn get_work_group_size_specialization_constants(
        &self,
    ) -> Result<spirv::WorkGroupSizeSpecializationConstants, ErrorCode> {
        let mut constants_raw = ptr::null_mut();

        unsafe {
            check!(
                sc_internal_compiler_get_work_group_size_specialization_constants(
                    self.sc_compiler,
                    &mut constants_raw,
                )
            );

            let x = *constants_raw.offset(0);
            let y = *constants_raw.offset(1);
            let z = *constants_raw.offset(2);

            let constants = spirv::WorkGroupSizeSpecializationConstants {
                x: spirv::SpecializationConstant {
                    id: x.id,
                    constant_id: x.constant_id,
                },
                y: spirv::SpecializationConstant {
                    id: y.id,
                    constant_id: y.constant_id,
                },
                z: spirv::SpecializationConstant {
                    id: z.id,
                    constant_id: z.constant_id,
                },
            };

            check!(sc_internal_free_pointer(constants_raw as *mut c_void));

            Ok(constants)
        }
    }
}

impl<TTargetData> Drop for Compiler<TTargetData> {
    fn drop(&mut self) {
        unsafe {
            sc_internal_compiler_delete(self.sc_compiler);
        }
    }
}
