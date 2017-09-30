use super::ErrorCode;
use spirv;
use bindings::root::*;
use std::ptr;
use std::ffi::CStr;
use std::os::raw::c_void;

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

#[derive(Debug, Clone)]
pub struct Compiler {
    base: spirv::Compiler,
}

impl Compiler {
    pub fn from_module(module: &spirv::Module) -> Result<Self, ErrorCode> {
        let base = unsafe {
            let mut compiler = ptr::null_mut();
            check!(sc_internal_compiler_hlsl_new(
                &mut compiler,
                module.ir.as_ptr() as *const u32,
                module.ir.len() as usize,
            ));

            spirv::Compiler {
                sc_compiler: compiler,
            }
        };

        Ok(Compiler { base })
    }

    pub fn get_decoration(&self, id: u32, decoration: spv::Decoration) -> Result<Option<u32>, ErrorCode> {
        self.base.get_decoration(id, decoration)
    }

    pub fn set_decoration(&self, id: u32, decoration: spv::Decoration, argument: u32) -> Result<(), ErrorCode> {
        self.base.set_decoration(id, decoration, argument)
    }

    pub fn get_entry_points(&self) -> Result<Vec<spirv::EntryPoint>, ErrorCode> {
        self.base.get_entry_points()
    }

    fn set_options(&self, options: &CompilerOptions) -> Result<(), ErrorCode> {
        let raw_options = options.as_raw();
        unsafe {
            check!(sc_internal_compiler_hlsl_set_options(
                self.base.sc_compiler,
                &raw_options,
            ));
        }

        Ok(())
    }

    pub fn compile(
        &self,
        options: &CompilerOptions,
    ) -> Result<String, ErrorCode> {
        self.set_options(options)?;
        self.base.compile()
    }
}
