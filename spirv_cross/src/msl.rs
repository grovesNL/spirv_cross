use bindings::root::*;
use std::collections::BTreeMap;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::ptr;
use {compiler, spirv, ErrorCode};

/// A MSL target.
#[derive(Debug, Clone)]
pub enum Target {}

pub struct TargetData {
    vertex_attribute_overrides: Vec<spirv_cross::MSLVertexAttr>,
    resource_binding_overrides: Vec<spirv_cross::MSLResourceBinding>,
}

impl spirv::Target for Target {
    type Data = TargetData;
}

/// Location of a vertex attribute to override
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct VertexAttributeLocation(pub u32);

/// Format of the vertex attribute
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Format {
    Other,
    Uint8,
    Uint16,
}

impl Format {
    fn as_raw(&self) -> spirv_cross::MSLVertexFormat {
        use self::spirv_cross::MSLVertexFormat as R;
        use self::Format::*;
        match self {
            Other => R::MSL_VERTEX_FORMAT_OTHER,
            Uint8 => R::MSL_VERTEX_FORMAT_UINT8,
            Uint16 => R::MSL_VERTEX_FORMAT_UINT16,
        }
    }
}

/// Vertex attribute description for overriding
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct VertexAttribute {
    pub buffer_id: u32,
    pub offset: u32,
    pub stride: u32,
    pub step: spirv::VertexAttributeStep,
    pub force_used: bool,
    pub format: Format,
}

/// Location of a resource binding to override
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ResourceBindingLocation {
    pub stage: spirv::ExecutionModel,
    pub desc_set: u32,
    pub binding: u32,
}

/// Resource binding description for overriding
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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
    V2_1,
}

impl Version {
    fn as_raw(&self) -> u32 {
        use self::Version::*;
        match *self {
            V1_0 => 10000,
            V1_1 => 10100,
            V1_2 => 10200,
            V2_0 => 20000,
            V2_1 => 20100,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CompilerOptions {
    /// The target platform.
    pub platform: Platform,
    /// The target MSL version.
    pub version: Version,
    /// Vertex compiler options.
    pub vertex: CompilerVertexOptions,
    /// Whether the built-in point size should be enabled.
    pub enable_point_size_builtin: bool,
    /// Whether rasterization should be enabled.
    pub enable_rasterization: bool,
    /// MSL resource bindings overrides.
    pub resource_binding_overrides: BTreeMap<ResourceBindingLocation, ResourceBinding>,
    /// MSL vertex attribute overrides.
    pub vertex_attribute_overrides: BTreeMap<VertexAttributeLocation, VertexAttribute>,
}

impl CompilerOptions {
    fn as_raw(&self) -> ScMslCompilerOptions {
        ScMslCompilerOptions {
            vertex_invert_y: self.vertex.invert_y,
            vertex_transform_clip_space: self.vertex.transform_clip_space,
            platform: self.platform as _,
            version: self.version.as_raw(),
            enable_point_size_builtin: self.enable_point_size_builtin,
            disable_rasterization: !self.enable_rasterization,
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
            enable_rasterization: true,
            resource_binding_overrides: Default::default(),
            vertex_attribute_overrides: Default::default(),
        }
    }
}

impl<'a> spirv::Parse<Target> for spirv::Ast<Target> {
    fn parse(module: &spirv::Module) -> Result<Self, ErrorCode> {
        let mut sc_compiler = ptr::null_mut();
        unsafe {
            check!(sc_internal_compiler_msl_new(
                &mut sc_compiler,
                module.words.as_ptr(),
                module.words.len(),
            ));
        }

        Ok(spirv::Ast {
            compiler: compiler::Compiler {
                sc_compiler,
                target_data: TargetData {
                    resource_binding_overrides: Vec::new(),
                    vertex_attribute_overrides: Vec::new(),
                },
                has_been_compiled: false,
            },
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

        self.compiler.target_data.resource_binding_overrides.clear();
        self.compiler.target_data.resource_binding_overrides.extend(
            options.resource_binding_overrides.iter().map(|(loc, res)| {
                spirv_cross::MSLResourceBinding {
                    stage: loc.stage.as_raw(),
                    desc_set: loc.desc_set,
                    binding: loc.binding,
                    msl_buffer: res.buffer_id,
                    msl_texture: res.texture_id,
                    msl_sampler: res.sampler_id,
                    used_by_shader: res.force_used,
                }
            }),
        );

        self.compiler.target_data.vertex_attribute_overrides.clear();
        self.compiler.target_data.vertex_attribute_overrides.extend(
            options.vertex_attribute_overrides.iter().map(|(loc, vat)| {
                spirv_cross::MSLVertexAttr {
                    location: loc.0,
                    msl_buffer: vat.buffer_id,
                    msl_offset: vat.offset,
                    msl_stride: vat.stride,
                    per_instance: match vat.step {
                        spirv::VertexAttributeStep::Vertex => false,
                        spirv::VertexAttributeStep::Instance => true,
                    },
                    used_by_shader: vat.force_used,
                    format: vat.format.as_raw(),
                }
            }),
        );

        Ok(())
    }

    /// Generate MSL shader from the AST.
    fn compile(&mut self) -> Result<String, ErrorCode> {
        self.compile_internal()
    }
}

impl spirv::Ast<Target> {
    fn compile_internal(&self) -> Result<String, ErrorCode> {
        let vat_overrides = &self.compiler.target_data.vertex_attribute_overrides;
        let res_overrides = &self.compiler.target_data.resource_binding_overrides;
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
            let shader = match CStr::from_ptr(shader_ptr).to_str() {
                Ok(v) => v.to_owned(),
                Err(_) => return Err(ErrorCode::Unhandled),
            };
            check!(sc_internal_free_pointer(shader_ptr as *mut c_void));
            Ok(shader)
        }
    }

    pub fn is_rasterization_enabled(&self) -> Result<bool, ErrorCode> {
        unsafe {
            let mut is_disabled = false;
            check!(sc_internal_compiler_msl_get_is_rasterization_disabled(
                self.compiler.sc_compiler,
                &mut is_disabled
            ));
            Ok(!is_disabled)
        }
    }
}
