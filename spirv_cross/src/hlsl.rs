use {compiler, spirv, ErrorCode};
use bindings::root::*;
use std::ptr;
use std::marker::PhantomData;

/// A HLSL target.
#[derive(Debug, Clone)]
pub enum Target {}

#[derive(Debug, Clone, Default)]
pub struct ParserOptions;

/// A HLSL shader model version.
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ShaderModel {
    V3_0,
    V4_0,
    V4_0L9_0,
    V4_0L9_1,
    V4_0L9_3,
    V4_1,
    V5_0,
    V5_1,
    V6_0,
}

#[allow(non_snake_case, non_camel_case_types)]
impl ShaderModel {
    fn as_raw(&self) -> i32 {
        match *self {
            ShaderModel::V3_0 => 30,
            ShaderModel::V4_0 => 40,
            ShaderModel::V4_0L9_0 => 40,
            ShaderModel::V4_0L9_1 => 40,
            ShaderModel::V4_0L9_3 => 40,
            ShaderModel::V4_1 => 41,
            ShaderModel::V5_0 => 50,
            ShaderModel::V5_1 => 51,
            ShaderModel::V6_0 => 60,
        }
    }
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

/// HLSL compiler options.
#[derive(Debug, Clone)]
pub struct CompilerOptions {
    pub shader_model: ShaderModel,
    pub vertex: CompilerVertexOptions,
}

impl CompilerOptions {
    fn as_raw(&self) -> ScHlslCompilerOptions {
        ScHlslCompilerOptions {
            shader_model: self.shader_model.as_raw(),
            vertex_invert_y: self.vertex.invert_y,
            vertex_transform_clip_space: self.vertex.transform_clip_space,
        }
    }
}

impl Default for CompilerOptions {
    fn default() -> CompilerOptions {
        CompilerOptions {
            shader_model: ShaderModel::V3_0,
            vertex: CompilerVertexOptions::default(),
        }
    }
}

impl spirv::Parse<Target> for spirv::Ast<Target> {
    type ParserOptions = ParserOptions;

    fn parse(module: &spirv::Module, _options: &ParserOptions) -> Result<Self, ErrorCode> {
        let compiler = {
            let mut compiler = ptr::null_mut();
            unsafe {
                check!(sc_internal_compiler_hlsl_new(
                    &mut compiler,
                    module.words.as_ptr() as *const u32,
                    module.words.len() as usize,
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

    /// Set HLSL compiler specific compilation settings.
    fn set_compile_options(&mut self, options: &CompilerOptions) -> Result<(), ErrorCode> {
        let raw_options = options.as_raw();
        unsafe {
            check!(sc_internal_compiler_hlsl_set_options(
                self.compiler.sc_compiler,
                &raw_options,
            ));
        }

        Ok(())
    }

    /// Generate HLSL shader from the AST.
    fn compile(&self) -> Result<String, ErrorCode> {
        self.compiler.compile()
    }
}
