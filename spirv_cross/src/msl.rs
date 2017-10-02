use {compiler, spirv, ErrorCode};
use bindings::root::*;
use std::ptr;

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
        CompilerOptions { vertex: CompilerVertexOptions::default() }
    }
}

#[derive(Debug, Clone)]
pub struct Compiler<'a> {
    base: &'a compiler::Compiler,
}

impl<'a> Compiler<'a> {
    /// Create a new MSL compiler from AST.
    pub fn from_ast(ast: &'a spirv::Ast) -> Self {
        assert_eq!(ast.target, spirv::Target::Msl);
        Compiler {
            base: &ast.compiler,
        }
    }

    /// Set MSL compiler specific compilation settings.
    fn set_options(&self, options: &CompilerOptions) -> Result<(), ErrorCode> {
        let raw_options = options.as_raw();
        unsafe {
            check!(sc_internal_compiler_msl_set_options(
                self.base.sc_compiler,
                &raw_options,
            ));
        }

        Ok(())
    }

    /// Generate MSL shader from the AST.
    pub fn compile(
        &self,
        options: &CompilerOptions,
    ) -> Result<String, ErrorCode> {
        self.set_options(options)?;
        self.base.compile()
    }
}
