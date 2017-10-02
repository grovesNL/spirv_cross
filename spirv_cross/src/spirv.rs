use ErrorCode;
use bindings::root::*;
use compiler;
use std::ptr;

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
    pub(crate) ir: &'a [u32],
}

impl<'a> Module<'a> {
    pub fn new(ir: &[u32]) -> Module {
        Module { ir }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Target {
    Hlsl,
    Msl,
}

pub struct Ast {
    pub(crate) target: Target,
    pub(crate) compiler: compiler::Compiler,
}

impl Ast {
    pub fn parse(module: &Module, target: Target) -> Result<Self, ErrorCode> {
        let compiler = {
            let mut compiler = ptr::null_mut();
            match target {
                Target::Hlsl => unsafe {
                    check!(sc_internal_compiler_hlsl_new(
                        &mut compiler,
                        module.ir.as_ptr() as *const u32,
                        module.ir.len() as usize,
                    ));
                },
                Target::Msl => unsafe {
                    check!(sc_internal_compiler_msl_new(
                        &mut compiler,
                        module.ir.as_ptr() as *const u32,
                        module.ir.len() as usize,
                    ));
                },
            }

            compiler::Compiler { sc_compiler: compiler }
        };

        Ok(Ast {
            compiler,
            target,
        })
    }

    pub fn get_decoration(&self, id: u32, decoration: spv::Decoration) -> Result<Option<u32>, ErrorCode> {
        self.compiler.get_decoration(id, decoration)
    }

    pub fn set_decoration(&self, id: u32, decoration: spv::Decoration, argument: u32) -> Result<(), ErrorCode> {
        self.compiler.set_decoration(id, decoration, argument)
    }

    pub fn get_entry_points(&self) -> Result<Vec<EntryPoint>, ErrorCode> {
        self.compiler.get_entry_points()
    }
}
