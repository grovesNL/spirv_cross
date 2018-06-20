use bindings::root::*;
use std::collections::HashMap;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::ptr;
use {compiler, spirv, ErrorCode};

/// A MSL target.
#[derive(Debug, Clone)]
pub enum Target {}

pub struct TargetData {
    pub(crate) vertex_attribute_overrides: HashMap<VertexAttributeLocation, VertexAttribute>,
    pub(crate) resource_binding_overrides: HashMap<ResourceBindingLocation, ResourceBinding>,
}

impl spirv::Target for Target {
    type Data = TargetData;
}

/// Location of a vertex attribute to override
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct VertexAttributeLocation(pub u32);

/// Vertex attribute description for overriding
#[derive(Debug, Clone)]
pub struct VertexAttribute {
    pub buffer_id: u32,
    pub offset: u32,
    pub stride: u32,
    pub step: spirv::VertexAttributeStep,
    pub force_used: bool,
}

/// Location of a resource binding to override
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ResourceBindingLocation {
    pub stage: spirv::ExecutionModel,
    pub desc_set: u32,
    pub binding: u32,
}

/// Resource binding description for overriding
#[derive(Debug, Clone)]
pub struct ResourceBinding {
    pub buffer_id: u32,
    pub texture_id: u32,
    pub sampler_id: u32,
    pub force_used: bool,
}

/// A MSL shader platform.
#[repr(u8)]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Platform {
    iOS = 0,
    macOS = 1,
}

/// A MSL shader model version.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Version {
    V1_0,
    V1_1,
    V1_2,
    V2_0,
}

impl Version {
    fn as_raw(&self) -> u32 {
        use self::Version::*;
        match *self {
            V1_0 => 10000,
            V1_1 => 10100,
            V1_2 => 10200,
            V2_0 => 20000,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompilerVertexOptions {
    pub invert_y: bool,
    pub transform_clip_space: bool,
}

impl Default for CompilerVertexOptions {
    fn default() -> Self {
        CompilerVertexOptions {
            invert_y: false,
            transform_clip_space: false,
        }
    }
}

/// MSL compiler options.
#[derive(Debug, Clone)]
pub struct CompilerOptions {
    ///
    pub platform: Platform,
    ///
    pub version: Version,
    ///
    pub vertex: CompilerVertexOptions,
    ///
    pub enable_point_size_builtin: bool,
    ///
    pub resolve_specialized_array_lengths: bool,
    /// MSL resource bindings overrides.
    pub resource_binding_overrides: HashMap<ResourceBindingLocation, ResourceBinding>,
    /// MSL vertex attribute overrides.
    pub vertex_attribute_overrides: HashMap<VertexAttributeLocation, VertexAttribute>,
}

impl CompilerOptions {
    fn as_raw(&self) -> ScMslCompilerOptions {
        ScMslCompilerOptions {
            vertex_invert_y: self.vertex.invert_y,
            vertex_transform_clip_space: self.vertex.transform_clip_space,
            platform: self.platform as _,
            version: self.version.as_raw(),
            enable_point_size_builtin: self.enable_point_size_builtin,
            resolve_specialized_array_lengths: self.resolve_specialized_array_lengths,
        }
    }
}

impl Default for CompilerOptions {
    fn default() -> Self {
        CompilerOptions {
            platform: Platform::macOS,
            version: Version::V1_2,
            vertex: CompilerVertexOptions::default(),
            enable_point_size_builtin: true,
            resolve_specialized_array_lengths: true,
            resource_binding_overrides: Default::default(),
            vertex_attribute_overrides: Default::default(),
        }
    }
}

impl<'a> spirv::Parse<Target> for spirv::Ast<Target> {
    fn parse(module: &spirv::Module) -> Result<Self, ErrorCode> {
        let compiler = {
            let mut compiler = ptr::null_mut();
            unsafe {
                check!(sc_internal_compiler_msl_new(
                    &mut compiler,
                    module.words.as_ptr() as *const u32,
                    module.words.len() as usize
                ));
            }

            compiler::Compiler {
                sc_compiler: compiler,
                target_data: TargetData {
                    resource_binding_overrides: Default::default(),
                    vertex_attribute_overrides: Default::default(),
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

    /// Set MSL compiler specific compilation settings.
    fn set_compiler_options(&mut self, options: &CompilerOptions) -> Result<(), ErrorCode> {
        let raw_options = options.as_raw();
        unsafe {
            check!(sc_internal_compiler_msl_set_options(
                self.compiler.sc_compiler,
                &raw_options,
            ));
        }

        self.compiler.target_data.resource_binding_overrides =
            options.resource_binding_overrides.clone();
        self.compiler.target_data.vertex_attribute_overrides =
            options.vertex_attribute_overrides.clone();

        Ok(())
    }

    /// Generate MSL shader from the AST.
    fn compile(&mut self) -> Result<String, ErrorCode> {
        self.compile_internal()
    }
}

impl spirv::Ast<Target> {
    fn compile_internal(&self) -> Result<String, ErrorCode> {
        let vat_overrides = self
            .compiler
            .target_data
            .vertex_attribute_overrides
            .iter()
            .map(|(loc, vat)| spirv_cross::MSLVertexAttr {
                location: loc.0,
                msl_buffer: vat.buffer_id,
                msl_offset: vat.offset,
                msl_stride: vat.stride,
                per_instance: match vat.step {
                    spirv::VertexAttributeStep::Vertex => false,
                    spirv::VertexAttributeStep::Instance => true,
                },
                used_by_shader: vat.force_used,
            })
            .collect::<Vec<_>>();

        let res_overrides = self
            .compiler
            .target_data
            .resource_binding_overrides
            .iter()
            .map(|(loc, res)| spirv_cross::MSLResourceBinding {
                stage: loc.stage.as_raw(),
                desc_set: loc.desc_set,
                binding: loc.binding,
                msl_buffer: res.buffer_id,
                msl_texture: res.texture_id,
                msl_sampler: res.sampler_id,
                used_by_shader: res.force_used,
            })
            .collect::<Vec<_>>();

        unsafe {
            let mut shader_ptr = ptr::null();
            check!(sc_internal_compiler_msl_compile(
                self.compiler.sc_compiler,
                &mut shader_ptr,
                vat_overrides.as_ptr(),
                vat_overrides.len(),
                res_overrides.as_ptr(),
                res_overrides.len(),
            ));
            let shader = match CStr::from_ptr(shader_ptr).to_owned().into_string() {
                Err(_) => return Err(ErrorCode::Unhandled),
                Ok(v) => v,
            };
            check!(sc_internal_free_pointer(shader_ptr as *mut c_void));
            Ok(shader)
        }
    }
}
