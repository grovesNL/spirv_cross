use ErrorCode;
use bindings::root::*;
use compiler;
use std::marker::PhantomData;

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
    pub(crate) words: &'a [u32],
}

impl<'a> Module<'a> {
    pub fn from_words(words: &[u32]) -> Module {
        Module { words }
    }
}

pub(crate) trait RawCompilerOptions<T> {
    fn as_raw(&self) -> T;
}

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
    pub fn get_decoration(
        &self,
        id: u32,
        decoration: spv::Decoration,
    ) -> Result<Option<u32>, ErrorCode> {
        self.compiler.get_decoration(id, decoration)
    }

    pub fn set_decoration(
        &mut self,
        id: u32,
        decoration: spv::Decoration,
        argument: u32,
    ) -> Result<(), ErrorCode> {
        self.compiler.set_decoration(id, decoration, argument)
    }

    pub fn get_entry_points(&self) -> Result<Vec<EntryPoint>, ErrorCode> {
        self.compiler.get_entry_points()
    }

    pub fn parse(module: &Module) -> Result<Self, ErrorCode> {
        Parse::<TTarget>::parse(&module)
    }

    pub fn set_compile_options(
        &mut self,
        options: <Ast<TTarget> as Compile<TTarget>>::CompilerOptions,
    ) -> Result<(), ErrorCode> {
        Compile::<TTarget>::set_compile_options(self, &options)
    }

    pub fn compile(&self) -> Result<String, ErrorCode> {
        Compile::<TTarget>::compile(self)
    }
}
