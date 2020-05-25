use crate::bindings as br;
use crate::ptr_util::read_into_vec_from_ptr;
use crate::{compiler, spirv, ErrorCode};
use std::marker::PhantomData;
use std::ptr;

/// A GLSL target.
#[derive(Debug, Clone)]
pub enum Target {}

pub struct TargetData {
    combined_image_samplers_built: bool,
}

impl spirv::Target for Target {
    type Data = TargetData;
}

#[allow(non_snake_case, non_camel_case_types)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Version {
    V1_10,
    V1_20,
    V1_30,
    V1_40,
    V1_50,
    V3_30,
    V4_00,
    V4_10,
    V4_20,
    V4_30,
    V4_40,
    V4_50,
    V4_60,
    V1_00Es,
    V3_00Es,
}

#[derive(Debug, Clone)]
pub struct CompilerVertexOptions {
    pub invert_y: bool,
    pub transform_clip_space: bool,
    pub support_nonzero_base_instance: bool,
}

impl Default for CompilerVertexOptions {
    fn default() -> CompilerVertexOptions {
        CompilerVertexOptions {
            invert_y: false,
            transform_clip_space: false,
            support_nonzero_base_instance: true,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[repr(u8)]
pub enum Precision {
    DontCare,
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone)]
pub struct CompilerFragmentOptions {
    pub default_float_precision: Precision,
    pub default_int_precision: Precision,
}

impl Default for CompilerFragmentOptions {
    fn default() -> CompilerFragmentOptions {
        CompilerFragmentOptions {
            default_float_precision: Precision::Medium,
            default_int_precision: Precision::High,
        }
    }
}

/// GLSL compiler options.
#[derive(Debug, Clone)]
pub struct CompilerOptions {
    pub version: Version,
    pub force_temporary: bool,
    pub vulkan_semantics: bool,
    pub separate_shader_objects: bool,
    pub flatten_multidimensional_arrays: bool,
    pub enable_420_pack_extension: bool,
    pub emit_push_constant_as_uniform_buffer: bool,
    pub emit_uniform_buffer_as_plain_uniforms: bool,
    pub emit_line_directives: bool,
    pub enable_storage_image_qualifier_deduction: bool,
    pub force_zero_initialized_variables: bool,
    pub vertex: CompilerVertexOptions,
    pub fragment: CompilerFragmentOptions,
}

impl CompilerOptions {
    fn as_raw(&self) -> br::ScGlslCompilerOptions {
        use self::Version::*;
        let (version, es) = match self.version {
            V1_10 => (1_10, false),
            V1_20 => (1_20, false),
            V1_30 => (1_30, false),
            V1_40 => (1_40, false),
            V1_50 => (1_50, false),
            V3_30 => (3_30, false),
            V4_00 => (4_00, false),
            V4_10 => (4_10, false),
            V4_20 => (4_20, false),
            V4_30 => (4_30, false),
            V4_40 => (4_40, false),
            V4_50 => (4_50, false),
            V4_60 => (4_60, false),
            V1_00Es => (1_00, true),
            V3_00Es => (3_00, true),
        };
        br::ScGlslCompilerOptions {
            vertex_invert_y: self.vertex.invert_y,
            vertex_transform_clip_space: self.vertex.transform_clip_space,
            version,
            es,
            vertex_support_nonzero_base_instance: self.vertex.support_nonzero_base_instance,
            fragment_default_float_precision: self.fragment.default_float_precision as u8,
            fragment_default_int_precision: self.fragment.default_int_precision as u8,
            force_temporary: self.force_temporary,
            vulkan_semantics: self.vulkan_semantics,
            separate_shader_objects: self.separate_shader_objects,
            flatten_multidimensional_arrays: self.flatten_multidimensional_arrays,
            enable_420_pack_extension: self.enable_420_pack_extension,
            emit_push_constant_as_uniform_buffer: self.emit_push_constant_as_uniform_buffer,
            emit_uniform_buffer_as_plain_uniforms: self.emit_uniform_buffer_as_plain_uniforms,
            emit_line_directives: self.emit_line_directives,
            enable_storage_image_qualifier_deduction: self.enable_storage_image_qualifier_deduction,
            force_zero_initialized_variables: self.force_zero_initialized_variables,
        }
    }
}

impl Default for CompilerOptions {
    fn default() -> CompilerOptions {
        CompilerOptions {
            version: Version::V4_50,
            force_temporary: false,
            vulkan_semantics: false,
            separate_shader_objects: false,
            flatten_multidimensional_arrays: false,
            enable_420_pack_extension: true,
            emit_push_constant_as_uniform_buffer: false,
            emit_uniform_buffer_as_plain_uniforms: false,
            emit_line_directives: false,
            enable_storage_image_qualifier_deduction: true,
            force_zero_initialized_variables: false,
            vertex: CompilerVertexOptions::default(),
            fragment: CompilerFragmentOptions::default(),
        }
    }
}

impl spirv::Parse<Target> for spirv::Ast<Target> {
    fn parse(module: &spirv::Module) -> Result<Self, ErrorCode> {
        let compiler = {
            let mut compiler = ptr::null_mut();
            unsafe {
                check!(br::sc_internal_compiler_glsl_new(
                    &mut compiler,
                    module.words.as_ptr() as *const u32,
                    module.words.len() as usize,
                ));
            }

            compiler::Compiler {
                sc_compiler: compiler,
                target_data: TargetData {
                    combined_image_samplers_built: false,
                },
                has_been_compiled: false,
            }
        };

        Ok(spirv::Ast {
            compiler,
            target_type: PhantomData,
        })
    }
}

impl spirv::Compile<Target> for spirv::Ast<Target> {
    type CompilerOptions = CompilerOptions;

    /// Set GLSL compiler specific compilation settings.
    fn set_compiler_options(&mut self, options: &CompilerOptions) -> Result<(), ErrorCode> {
        let raw_options = options.as_raw();
        unsafe {
            check!(br::sc_internal_compiler_glsl_set_options(
                self.compiler.sc_compiler,
                &raw_options,
            ));
        }

        Ok(())
    }

    /// Generate GLSL shader from the AST.
    fn compile(&mut self) -> Result<String, ErrorCode> {
        self.build_combined_image_samplers()?;
        self.compiler.compile()
    }
}

impl spirv::Ast<Target> {
    pub fn build_combined_image_samplers(&mut self) -> Result<(), ErrorCode> {
        unsafe {
            if !self.compiler.target_data.combined_image_samplers_built {
                check!(br::sc_internal_compiler_glsl_build_combined_image_samplers(
                    self.compiler.sc_compiler
                ));
                self.compiler.target_data.combined_image_samplers_built = true
            }
        }

        Ok(())
    }

    pub fn get_combined_image_samplers(
        &mut self,
    ) -> Result<Vec<spirv::CombinedImageSampler>, ErrorCode> {
        self.build_combined_image_samplers()?;
        unsafe {
            let mut samplers_raw: *const br::ScCombinedImageSampler = std::ptr::null();
            let mut samplers_raw_length: usize = 0;

            check!(br::sc_internal_compiler_glsl_get_combined_image_samplers(
                self.compiler.sc_compiler,
                &mut samplers_raw as _,
                &mut samplers_raw_length as _,
            ));

            let samplers = read_into_vec_from_ptr(samplers_raw, samplers_raw_length)
                .iter()
                .map(|sc| spirv::CombinedImageSampler {
                    combined_id: sc.combined_id,
                    image_id: sc.image_id,
                    sampler_id: sc.sampler_id,
                })
                .collect();

            Ok(samplers)
        }
    }
}
