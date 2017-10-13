use {compiler, spirv, ErrorCode};
use bindings::root::*;
use std::ptr;
use std::collections::HashMap;
use std::marker::PhantomData;

/// A MSL target.
#[derive(Debug, Clone)]
pub enum Target {}


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
    pub resource_id: u32,
    pub force_used: bool,
}

/// MSL parser options.
#[derive(Debug, Clone, Default)]
pub struct ParserOptions {
    pub vertex_attribute_overrides: HashMap<VertexAttributeLocation, VertexAttribute>,
    pub resource_binding_overrides: HashMap<ResourceBindingLocation, ResourceBinding>,
}


#[derive(Debug, Clone)]
pub struct CompilerVertexOptions {
    pub invert_y: bool,
    pub transform_clip_space: bool,
}

impl Default for CompilerVertexOptions {
    fn default() -> CompilerVertexOptions {
        CompilerVertexOptions {
            invert_y: false,
            transform_clip_space: false,
        }
    }
}


/// MSL compiler options.
#[derive(Debug, Clone)]
pub struct CompilerOptions {
    pub vertex: CompilerVertexOptions,
}

impl CompilerOptions {
    fn as_raw(&self) -> ScMslCompilerOptions {
        ScMslCompilerOptions {
            vertex_invert_y: self.vertex.invert_y,
            vertex_transform_clip_space: self.vertex.transform_clip_space,
        }
    }
}

impl Default for CompilerOptions {
    fn default() -> CompilerOptions {
        CompilerOptions {
            vertex: CompilerVertexOptions::default(),
        }
    }
}


impl spirv::Parse<Target> for spirv::Ast<Target> {
    type ParserOptions = ParserOptions;

    fn parse(module: &spirv::Module, options: &ParserOptions) -> Result<Self, ErrorCode> {
        let compiler = {
            let vat_overrides = options.vertex_attribute_overrides
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

            let res_overrides = options.resource_binding_overrides
                .iter()
                .map(|(loc, res)| spirv_cross::MSLResourceBinding {
                    stage: loc.stage.as_raw(),
                    desc_set: loc.desc_set,
                    binding: loc.binding,
                    msl_buffer: res.resource_id,
                    msl_texture: res.resource_id,
                    msl_sampler: res.resource_id,
                    used_by_shader: res.force_used,
                })
                .collect::<Vec<_>>();

            let mut compiler = ptr::null_mut();
            unsafe {
                check!(sc_internal_compiler_msl_new(
                    &mut compiler,
                    module.words.as_ptr() as *const u32,
                    module.words.len() as usize,
                    vat_overrides.as_ptr(),
                    vat_overrides.len(),
                    res_overrides.as_ptr(),
                    res_overrides.len(),
                ));
            }

            compiler::Compiler {
                sc_compiler: compiler,
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
    fn set_compile_options(&mut self, options: &CompilerOptions) -> Result<(), ErrorCode> {
        let raw_options = options.as_raw();
        unsafe {
            check!(sc_internal_compiler_msl_set_options(
                self.compiler.sc_compiler,
                &raw_options,
            ));
        }

        Ok(())
    }

    /// Generate MSL shader from the AST.
    fn compile(&self) -> Result<String, ErrorCode> {
        self.compiler.compile()
    }
}
