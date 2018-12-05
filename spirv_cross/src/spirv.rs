use compiler;
use std::marker::PhantomData;
use ErrorCode;

/// A stage or compute kernel.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct CombinedImageSampler {
    pub combined_id: u32,
    pub image_id: u32,
    pub sampler_id: u32,
}

/// A stage or compute kernel.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
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
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct EntryPoint {
    pub name: String,
    pub execution_model: ExecutionModel,
    pub work_group_size: WorkGroupSize,
}

/// A resource.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Resource {
    pub id: u32,
    pub type_id: u32,
    pub base_type_id: u32,
    pub name: String,
}

/// Specialization constant reference.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct SpecializationConstant {
    pub id: u32,
    pub constant_id: u32,
}

/// Work group size specialization constants.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct WorkGroupSizeSpecializationConstants {
    pub x: SpecializationConstant,
    pub y: SpecializationConstant,
    pub z: SpecializationConstant,
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

#[derive(Debug, Clone)]
pub enum Type {
    // TODO: Add missing fields to relevant variants from SPIRType
    Unknown,
    Void,
    Boolean {
        array: Vec<u32>,
    },
    Char {
        array: Vec<u32>,
    },
    Int {
        array: Vec<u32>,
    },
    UInt {
        array: Vec<u32>,
    },
    Int64 {
        array: Vec<u32>,
    },
    UInt64 {
        array: Vec<u32>,
    },
    AtomicCounter {
        array: Vec<u32>,
    },
    Half {
        array: Vec<u32>,
    },
    Float {
        array: Vec<u32>,
    },
    Double {
        array: Vec<u32>,
    },
    Struct {
        member_types: Vec<u32>,
        array: Vec<u32>,
    },
    Image {
        array: Vec<u32>,
    },
    SampledImage {
        array: Vec<u32>,
    },
    Sampler {
        array: Vec<u32>,
    },
    SByte {
        array: Vec<u32>,
    },
    UByte {
        array: Vec<u32>,
    },
    Short {
        array: Vec<u32>,
    },
    UShort {
        array: Vec<u32>,
    },
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

    /// Sets a name.
    pub fn set_name(&mut self, id: u32, name: &str) -> Result<(), ErrorCode> {
        self.compiler.set_name(id, name)
    }

    /// Unsets a decoration.
    pub fn unset_decoration(&mut self, id: u32, decoration: Decoration) -> Result<(), ErrorCode> {
        self.compiler.unset_decoration(id, decoration)
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
        execution_model: ExecutionModel,
    ) -> Result<String, ErrorCode> {
        assert!(
            self.compiler.has_been_compiled,
            "`compile` must be called first"
        );
        self.compiler
            .get_cleansed_entry_point_name(entry_point_name, execution_model)
    }

    /// Gets all specialization constants.
    pub fn get_specialization_constants(&self) -> Result<Vec<SpecializationConstant>, ErrorCode> {
        self.compiler.get_specialization_constants()
    }

    /// Set reference of a scalar constant to a value, overriding the default.
    ///
    /// Can be used to override specialization constants.
    pub fn set_scalar_constant(&mut self, id: u32, value: u64) -> Result<(), ErrorCode> {
        self.compiler.set_scalar_constant(id, value)
    }

    /// Gets shader resources.
    pub fn get_shader_resources(&self) -> Result<ShaderResources, ErrorCode> {
        self.compiler.get_shader_resources()
    }

    /// Gets the SPIR-V type associated with an ID.
    pub fn get_type(&self, id: u32) -> Result<Type, ErrorCode> {
        self.compiler.get_type(id)
    }

    /// Gets the identifier for a member located at `index` within an `OpTypeStruct`.
    pub fn get_member_name(&self, id: u32, index: u32) -> Result<String, ErrorCode> {
        self.compiler.get_member_name(id, index)
    }

    /// Gets a decoration for a member located at `index` within an `OpTypeStruct`.
    pub fn get_member_decoration(
        &self,
        id: u32,
        index: u32,
        decoration: Decoration,
    ) -> Result<u32, ErrorCode> {
        self.compiler.get_member_decoration(id, index, decoration)
    }

    /// Sets a decoration for a member located at `index` within an `OpTypeStruct`.
    pub fn set_member_decoration(
        &mut self,
        id: u32,
        index: u32,
        decoration: Decoration,
        argument: u32,
    ) -> Result<(), ErrorCode> {
        self.compiler
            .set_member_decoration(id, index, decoration, argument)
    }

    /// Gets the effective size of a buffer block.
    pub fn get_declared_struct_size(&self, id: u32) -> Result<u32, ErrorCode> {
        self.compiler.get_declared_struct_size(id)
    }

    /// Gets the effective size of a buffer block struct member.
    pub fn get_declared_struct_member_size(&self, id: u32, index: u32) -> Result<u32, ErrorCode> {
        self.compiler.get_declared_struct_member_size(id, index)
    }

    /// Renames an interface variable.
    pub fn rename_interface_variable(
        &mut self,
        resources: &[Resource],
        location: u32,
        name: &str,
    ) -> Result<(), ErrorCode> {
        self.compiler
            .rename_interface_variable(resources, location, name)
    }

    /// Gets work group size specialization constants.
    pub fn get_work_group_size_specialization_constants(
        &self,
    ) -> Result<WorkGroupSizeSpecializationConstants, ErrorCode> {
        self.compiler.get_work_group_size_specialization_constants()
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
