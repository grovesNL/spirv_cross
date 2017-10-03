use ErrorCode;
use bindings::root::*;
use compiler;
use std::marker::PhantomData;

/// A stage or compute kernel.
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

/// A work group size.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct WorkgroupSize {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

/// An entry point for a SPIR-V module.
#[derive(Debug, Clone)]
pub struct EntryPoint {
    pub name: String,
    pub execution_model: ExecutionModel,
    pub workgroup_size: WorkgroupSize,
}

/// A SPIR-V shader module.
#[derive(Debug, Clone)]
pub struct Module<'a> {
    pub(crate) words: &'a [u32],
}

impl<'a> Module<'a> {
    /// Creates a shader module from SPIR-V words.
    pub fn from_words(words: &[u32]) -> Module {
        Module { words }
    }
}

/// An abstract syntax tree that corresponds to a SPIR-V module.
pub struct Ast<TTarget> {
    pub(crate) compiler: compiler::Compiler,
    pub(crate) target_type: PhantomData<TTarget>,
}

pub trait Parse<TTarget>: Sized {
    fn parse(module: &Module) -> Result<Self, ErrorCode>;
}

pub trait Compile<TTarget> {
    type CompilerOptions;

    fn set_compile_options(&mut self, options: &Self::CompilerOptions) -> Result<(), ErrorCode>;

    fn compile(&self) -> Result<String, ErrorCode>;
}

impl<TTarget> Ast<TTarget>
where
    Ast<TTarget>: Parse<TTarget> + Compile<TTarget>,
{
    /// Gets a decoration.
    pub fn get_decoration(
        &self,
        id: u32,
        decoration: spv::Decoration,
    ) -> Result<Option<u32>, ErrorCode> {
        self.compiler.get_decoration(id, decoration)
    }

    /// Sets a decoration.
    pub fn set_decoration(
        &mut self,
        id: u32,
        decoration: spv::Decoration,
        argument: u32,
    ) -> Result<(), ErrorCode> {
        self.compiler.set_decoration(id, decoration, argument)
    }

    /// Gets entry points.
    pub fn get_entry_points(&self) -> Result<Vec<EntryPoint>, ErrorCode> {
        self.compiler.get_entry_points()
    }

    /// Parses a module into `Ast`.
    pub fn parse(module: &Module) -> Result<Self, ErrorCode> {
        Parse::<TTarget>::parse(&module)
    }

    /// Sets compile options.
    pub fn set_compile_options(
        &mut self,
        options: <Ast<TTarget> as Compile<TTarget>>::CompilerOptions,
    ) -> Result<(), ErrorCode> {
        Compile::<TTarget>::set_compile_options(self, &options)
    }

    /// Compiles an abstract syntax tree to a `String` in the specified `TTarget` language.
    pub fn compile(&self) -> Result<String, ErrorCode> {
        Compile::<TTarget>::compile(self)
    }
}
