use ErrorCode;
use compiler;
use std::marker::PhantomData;

/// A stage or compute kernel.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ExecutionModel {
    Vertex,
    TessellationControl,
    TessellationEvaluation,
    Geometry,
    Fragment,
    GlCompute,
    Kernel,
}

/// A decoration.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Decoration {
    RelaxedPrecision,
    SpecId,
    Block,
    BufferBlock,
    RowMajor,
    ColMajor,
    ArrayStride,
    MatrixStride,
    GlslShared,
    GlslPacked,
    CPacked,
    BuiltIn,
    NoPerspective,
    Flat,
    Patch,
    Centroid,
    Sample,
    Invariant,
    Restrict,
    Aliased,
    Volatile,
    Constant,
    Coherent,
    NonWritable,
    NonReadable,
    Uniform,
    SaturatedConversion,
    Stream,
    Location,
    Component,
    Index,
    Binding,
    DescriptorSet,
    Offset,
    XfbBuffer,
    XfbStride,
    FuncParamAttr,
    FpRoundingMode,
    FpFastMathMode,
    LinkageAttributes,
    NoContraction,
    InputAttachmentIndex,
    Alignment,
    OverrideCoverageNv,
    PassthroughNv,
    ViewportRelativeNv,
    SecondaryViewportRelativeNv,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum VertexAttributeStep {
    Vertex,
    Instance,
}

/// A work group size.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct WorkGroupSize {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

/// An entry point for a SPIR-V module.
#[derive(Debug, Clone)]
pub struct EntryPoint {
    pub name: String,
    pub execution_model: ExecutionModel,
    pub work_group_size: WorkGroupSize,
}

/// A resource.
#[derive(Debug, Clone)]
pub struct Resource {
    pub id: u32,
    pub type_id: u32,
    pub base_type_id: u32,
    pub name: String,
}

/// Specialization constant reference.
#[derive(Debug, Clone)]
pub struct SpecializationConstant {
    pub id: u32,
    pub constant_id: u32,
}

/// Shader resources.
#[derive(Debug, Clone)]
pub struct ShaderResources {
    pub uniform_buffers: Vec<Resource>,
    pub storage_buffers: Vec<Resource>,
    pub stage_inputs: Vec<Resource>,
    pub stage_outputs: Vec<Resource>,
    pub subpass_inputs: Vec<Resource>,
    pub storage_images: Vec<Resource>,
    pub sampled_images: Vec<Resource>,
    pub atomic_counters: Vec<Resource>,
    pub push_constant_buffers: Vec<Resource>,
    pub separate_images: Vec<Resource>,
    pub separate_samplers: Vec<Resource>,
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

pub trait Target {
    type Data;
}

/// An abstract syntax tree that corresponds to a SPIR-V module.
pub struct Ast<TTarget>
where
    TTarget: Target,
{
    pub(crate) compiler: compiler::Compiler<TTarget::Data>,
    pub(crate) target_type: PhantomData<TTarget>,
}

pub trait Parse<TTarget>: Sized {
    fn parse(module: &Module) -> Result<Self, ErrorCode>;
}

pub trait Compile<TTarget> {
    type CompilerOptions;

    fn set_compiler_options(&mut self, &Self::CompilerOptions) -> Result<(), ErrorCode>;
    fn compile(&mut self) -> Result<String, ErrorCode>;
}

impl<TTarget> Ast<TTarget>
where
    Self: Parse<TTarget> + Compile<TTarget>,
    TTarget: Target,
{
    /// Gets a decoration.
    pub fn get_decoration(&self, id: u32, decoration: Decoration) -> Result<u32, ErrorCode> {
        self.compiler.get_decoration(id, decoration)
    }

    /// Sets a decoration.
    pub fn set_decoration(
        &mut self,
        id: u32,
        decoration: Decoration,
        argument: u32,
    ) -> Result<(), ErrorCode> {
        self.compiler.set_decoration(id, decoration, argument)
    }

    /// Gets entry points.
    pub fn get_entry_points(&self) -> Result<Vec<EntryPoint>, ErrorCode> {
        self.compiler.get_entry_points()
    }

    /// Gets cleansed entry point names. `compile` must be called first.
    pub fn get_cleansed_entry_point_name(
        &self,
        entry_point_name: &str,
    ) -> Result<String, ErrorCode> {
        assert!(
            self.compiler.has_been_compiled,
            "`compile` must be called first"
        );
        self.compiler
            .get_cleansed_entry_point_name(entry_point_name)
    }

    /// Gets all specialization constants.
    pub fn get_specialization_constants(&self) -> Result<Vec<SpecializationConstant>, ErrorCode> {
        self.compiler.get_specialization_constants()
    }

    /// Set reference of a scalar constant to a value, overriding the default.
    ///
    /// Can be used to override specialization constants.
    pub fn set_scalar_constant(&self, id: u32, value: u64) -> Result<(), ErrorCode> {
        self.compiler.set_scalar_constant(id, value)
    }

    /// Gets shader resources.
    pub fn get_shader_resources(&self) -> Result<ShaderResources, ErrorCode> {
        self.compiler.get_shader_resources()
    }

    /// Parses a module into `Ast`.
    pub fn parse(module: &Module) -> Result<Self, ErrorCode> {
        Parse::<TTarget>::parse(&module)
    }

    /// Sets compile options.
    pub fn set_compiler_options(
        &mut self,
        options: &<Self as Compile<TTarget>>::CompilerOptions,
    ) -> Result<(), ErrorCode> {
        Compile::<TTarget>::set_compiler_options(self, options)
    }

    /// Compiles an abstract syntax tree to a `String` in the specified `TTarget` language.
    pub fn compile(&mut self) -> Result<String, ErrorCode> {
        self.compiler.has_been_compiled = true;
        Compile::<TTarget>::compile(self)
    }
}
