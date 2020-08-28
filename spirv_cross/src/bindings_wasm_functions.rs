//! These are manually implemented wrappers that accept the same signatures generated by bindgen
//! and instead forward the calls onto the raw Emscripten bindings. Any necessary type conversions,
//! copying, etc. are performed internally, so higher level code (such as `Compiler`)  is mostly
//! unaware whether native or web code is being called. Some exceptions are places where pointers
//! would consumed directly (i.e. reading a string from a pointer), so `ptr_util` is provided for
//! those cases.

use crate::emscripten;
use crate::{bindings, ErrorCode};
use js_sys::{global, Object, Reflect, Uint32Array, Uint8Array};
use std::ffi::CStr;
use wasm_bindgen::prelude::*;

const U32_SIZE: u32 = std::mem::size_of::<u32>() as u32;

#[wasm_bindgen]
extern "C" {
    // Raw SPIRV-Cross bindings
    // Pointers and the result type are replaced with `u32`
    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_glsl_new(compiler: u32, ir: u32, size: u32) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_glsl_set_options(compiler: u32, options: u32) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_glsl_build_combined_image_samplers(compiler: u32) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_glsl_get_combined_image_samplers(
        compiler: u32,
        samplers: u32,
        size: u32,
    ) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_glsl_add_header_line(compiler: u32, str: u32) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_glsl_flatten_buffer_block(compiler: u32, id: u32) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_get_decoration(
        compiler: u32,
        result: u32,
        id: u32,
        decoration: u32,
    ) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_set_decoration(
        compiler: u32,
        id: u32,
        decoration: u32,
        argument: u32,
    ) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_unset_decoration(compiler: u32, id: u32, decoration: u32) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_get_name(compiler: u32, id: u32, name: u32) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_set_name(compiler: u32, id: u32, name: u32) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_get_entry_points(compiler: u32, entry_points: u32, size: u32) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_get_active_buffer_ranges(
        compiler: u32,
        id: u32,
        active_buffer_ranges: u32,
        size: u32,
    ) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_get_cleansed_entry_point_name(
        compiler: u32,
        original_entry_point_name: u32,
        execution_model: u32,
        compiled_entry_point_name: u32,
    ) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_get_shader_resources(compiler: u32, shader_resources: u32) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_get_specialization_constants(
        compiler: u32,
        constants: u32,
        size: u32,
    ) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_set_scalar_constant(
        compiler: u32,
        id: u32,
        constant_high_bits: u32,
        constant_low_bits: u32,
    ) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_get_type(compiler: u32, id: u32, spirv_type: u32) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_get_member_name(compiler: u32, id: u32, index: u32, name: u32) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_get_member_decoration(
        compiler: u32,
        id: u32,
        index: u32,
        decoration: u32,
        result: u32,
    ) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_set_member_decoration(
        compiler: u32,
        id: u32,
        index: u32,
        decoration: u32,
        argument: u32,
    ) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_get_declared_struct_size(compiler: u32, id: u32, result: u32) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_get_declared_struct_member_size(
        compiler: u32,
        id: u32,
        index: u32,
        result: u32,
    ) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_rename_interface_variable(
        compiler: u32,
        resources: u32,
        resources_size: u32,
        location: u32,
        name: u32,
    ) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_get_work_group_size_specialization_constants(
        compiler: u32,
        constants: u32,
    ) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_compile(compiler: u32, shader: u32) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_compiler_delete(compiler: u32) -> u32;

    #[wasm_bindgen(js_namespace = sc_internal)]
    fn _sc_internal_free_pointer(pointer: u32) -> u32;
}

fn map_internal_result(result: u32) -> bindings::ScInternalResult {
    match result {
        0 => bindings::ScInternalResult::Success,
        1 => bindings::ScInternalResult::Unhandled,
        2 => bindings::ScInternalResult::CompilationError,
        _ => unreachable!(),
    }
}

pub fn sc_internal_get_latest_exception_message(
    message: *mut *const ::std::os::raw::c_char,
) -> bindings::ScInternalResult {
    // This is unhandled for now
    // Exceptions are disabled and assertions are used instead
    bindings::ScInternalResult::Success
}

pub fn sc_internal_compiler_glsl_new(
    compiler: *mut *mut bindings::ScInternalCompilerGlsl,
    ir: *const u32,
    size: usize,
) -> bindings::ScInternalResult {
    let spirv_bytes = size * (U32_SIZE as usize);
    unsafe {
        let spirv = std::slice::from_raw_parts(ir as *const u8, spirv_bytes);
        let module = emscripten::get_module();
        let spirv_ptr = module.allocate(spirv_bytes as u32);
        module.set_from_u8_slice(spirv_ptr, spirv);
        let compiler_ptr_to_ptr = module.allocate(U32_SIZE);
        let result = map_internal_result(_sc_internal_compiler_glsl_new(
            compiler_ptr_to_ptr.as_offset(),
            spirv_ptr.as_offset(),
            size as u32,
        ));
        *compiler = module.get_u32(compiler_ptr_to_ptr) as *mut bindings::ScInternalCompilerGlsl;
        module.free(compiler_ptr_to_ptr);
        module.free(spirv_ptr);
        result
    }
}

pub fn sc_internal_compiler_glsl_set_options(
    compiler: *const bindings::ScInternalCompilerGlsl,
    options: *const bindings::ScGlslCompilerOptions,
) -> bindings::ScInternalResult {
    // For native usage, we expect Rust to manage the memory of options
    // For web usage, we have to copy it to the Emscripten heap temporarily
    // Alternatively, we could allow C++ and Emscripten to provide a pointer, then fill out
    // the struct fields on the Rust side instead - this already happens in some of the bindings
    let module = emscripten::get_module();
    let compiler_options_size = std::mem::size_of::<bindings::ScGlslCompilerOptions>();

    unsafe {
        let bytes = std::slice::from_raw_parts(options as *const u8, compiler_options_size);
        let copied_options_ptr = module.allocate(compiler_options_size as u32);
        module.set_from_u8_slice(copied_options_ptr, bytes);
        let result = map_internal_result(_sc_internal_compiler_glsl_set_options(
            compiler as u32,
            copied_options_ptr.as_offset(),
        ));
        module.free(copied_options_ptr);
        result
    }
}

pub fn sc_internal_compiler_glsl_build_combined_image_samplers(
    compiler: *const bindings::ScInternalCompilerBase,
) -> bindings::ScInternalResult {
    map_internal_result(_sc_internal_compiler_glsl_build_combined_image_samplers(
        compiler as u32,
    ))
}

pub fn sc_internal_compiler_glsl_get_combined_image_samplers(
    compiler: *const bindings::ScInternalCompilerBase,
    samplers: *mut *const bindings::ScCombinedImageSampler,
    size: *mut usize,
) -> bindings::ScInternalResult {
    let module = emscripten::get_module();
    unsafe {
        let samplers_ptr_to_ptr = module.allocate(U32_SIZE);
        let size_ptr = module.allocate(U32_SIZE);
        let result = map_internal_result(_sc_internal_compiler_glsl_get_combined_image_samplers(
            compiler as u32,
            samplers_ptr_to_ptr.as_offset(),
            size_ptr.as_offset(),
        ));

        *samplers = module.get_u32(samplers_ptr_to_ptr) as *const bindings::ScCombinedImageSampler;
        *size = module.get_u32(size_ptr) as usize;

        module.free(samplers_ptr_to_ptr);
        module.free(size_ptr);

        result
    }
}

pub fn sc_internal_compiler_glsl_add_header_line(
    compiler: *const bindings::ScInternalCompilerBase,
    str: *const ::std::os::raw::c_char,
) -> bindings::ScInternalResult {
    let module = emscripten::get_module();
    unsafe {
        let str_bytes = CStr::from_ptr(str).to_bytes();
        let str_ptr = module.allocate(str_bytes.len() as u32);
        module.set_from_u8_slice(str_ptr, str_bytes);
        let result = map_internal_result(_sc_internal_compiler_glsl_add_header_line(
            compiler as u32,
            str_ptr.as_offset(),
        ));
        module.free(str_ptr);
        result
    }
}

pub fn sc_internal_compiler_glsl_flatten_buffer_block(
    compiler: *const bindings::ScInternalCompilerBase,
    id: u32,
) -> bindings::ScInternalResult {
    let module = emscripten::get_module();
    unsafe {
        let result = map_internal_result(_sc_internal_compiler_glsl_flatten_buffer_block(
            compiler as u32,
            id,
        ));
        result
    }
}

pub fn sc_internal_compiler_get_decoration(
    compiler: *const bindings::ScInternalCompilerBase,
    result: *mut u32,
    id: u32,
    decoration: bindings::spv::Decoration,
) -> bindings::ScInternalResult {
    let module = emscripten::get_module();
    unsafe {
        let result_ptr = module.allocate(U32_SIZE);
        let ret = map_internal_result(_sc_internal_compiler_get_decoration(
            compiler as u32,
            result_ptr.as_offset(),
            id,
            decoration as u32,
        ));
        *result = module.get_u32(result_ptr) as u32;
        module.free(result_ptr);
        ret
    }
}

pub fn sc_internal_compiler_set_decoration(
    compiler: *const bindings::ScInternalCompilerBase,
    id: u32,
    decoration: bindings::spv::Decoration,
    argument: u32,
) -> bindings::ScInternalResult {
    map_internal_result(_sc_internal_compiler_set_decoration(
        compiler as u32,
        id,
        decoration as u32,
        argument,
    ))
}

pub fn sc_internal_compiler_unset_decoration(
    compiler: *const bindings::ScInternalCompilerBase,
    id: u32,
    decoration: bindings::spv::Decoration,
) -> bindings::ScInternalResult {
    map_internal_result(_sc_internal_compiler_unset_decoration(
        compiler as u32,
        id,
        decoration as u32,
    ))
}

pub fn sc_internal_compiler_get_name(
    compiler: *const bindings::ScInternalCompilerBase,
    id: u32,
    name: *mut *const ::std::os::raw::c_char,
) -> bindings::ScInternalResult {
    let module = emscripten::get_module();
    unsafe {
        let name_ptr_to_ptr = module.allocate(U32_SIZE);
        let result = map_internal_result(_sc_internal_compiler_get_name(
            compiler as u32,
            id,
            name_ptr_to_ptr.as_offset(),
        ));
        let name_ptr = module.get_u32(name_ptr_to_ptr);
        *name = name_ptr as *const ::std::os::raw::c_char;
        module.free(name_ptr_to_ptr);
        result
    }
}

pub fn sc_internal_compiler_set_name(
    compiler: *const bindings::ScInternalCompilerBase,
    id: u32,
    name: *const ::std::os::raw::c_char,
) -> bindings::ScInternalResult {
    let module = emscripten::get_module();
    unsafe {
        let name_bytes = CStr::from_ptr(name).to_bytes();
        let name_ptr = module.allocate(name_bytes.len() as u32);
        module.set_from_u8_slice(name_ptr, name_bytes);
        let result = map_internal_result(_sc_internal_compiler_set_name(
            compiler as u32,
            id,
            name_ptr.as_offset(),
        ));
        module.free(name_ptr);
        result
    }
}

pub fn sc_internal_compiler_get_entry_points(
    compiler: *const bindings::ScInternalCompilerBase,
    entry_points: *mut *mut bindings::ScEntryPoint,
    size: *mut usize,
) -> bindings::ScInternalResult {
    let module = emscripten::get_module();
    unsafe {
        let entry_points_ptr_to_ptr = module.allocate(U32_SIZE);
        let size_ptr = module.allocate(U32_SIZE);

        let result = map_internal_result(_sc_internal_compiler_get_entry_points(
            compiler as u32,
            entry_points_ptr_to_ptr.as_offset(),
            size_ptr.as_offset(),
        ));

        *entry_points = module.get_u32(entry_points_ptr_to_ptr) as *mut bindings::ScEntryPoint;
        *size = module.get_u32(size_ptr) as usize;

        module.free(size_ptr);
        module.free(entry_points_ptr_to_ptr);

        result
    }
}

pub fn sc_internal_compiler_get_active_buffer_ranges(
    compiler: *const bindings::ScInternalCompilerBase,
    id: u32,
    active_buffer_ranges: *mut *mut bindings::ScBufferRange,
    size: *mut usize,
) -> bindings::ScInternalResult {
    let module = emscripten::get_module();
    unsafe {
        let active_buffer_ranges_ptr_to_ptr = module.allocate(U32_SIZE);
        let size_ptr = module.allocate(U32_SIZE);

        let result = map_internal_result(_sc_internal_compiler_get_active_buffer_ranges(
            compiler as u32,
            id,
            active_buffer_ranges_ptr_to_ptr.as_offset(),
            size_ptr.as_offset(),
        ));

        *active_buffer_ranges =
            module.get_u32(active_buffer_ranges_ptr_to_ptr) as *mut bindings::ScBufferRange;
        *size = module.get_u32(size_ptr) as usize;

        module.free(size_ptr);
        module.free(active_buffer_ranges_ptr_to_ptr);

        result
    }
}

pub fn sc_internal_compiler_get_cleansed_entry_point_name(
    compiler: *const bindings::ScInternalCompilerBase,
    original_entry_point_name: *const ::std::os::raw::c_char,
    execution_model: bindings::spv::ExecutionModel,
    compiled_entry_point_name: *mut *const ::std::os::raw::c_char,
) -> bindings::ScInternalResult {
    let module = emscripten::get_module();
    unsafe {
        let original_name_bytes = CStr::from_ptr(original_entry_point_name).to_bytes_with_nul();
        let original_name_ptr = module.allocate(original_name_bytes.len() as u32);
        module.set_from_u8_slice(original_name_ptr, original_name_bytes);

        let compiled_name_ptr_to_ptr = module.allocate(U32_SIZE);
        let result = map_internal_result(_sc_internal_compiler_get_cleansed_entry_point_name(
            compiler as u32,
            original_name_ptr.as_offset(),
            execution_model as u32,
            compiled_name_ptr_to_ptr.as_offset(),
        ));
        let compiled_name_ptr = module.get_u32(compiled_name_ptr_to_ptr);
        *compiled_entry_point_name = compiled_name_ptr as *const ::std::os::raw::c_char;

        module.free(compiled_name_ptr_to_ptr);
        module.free(original_name_ptr);

        result
    }
}

pub fn sc_internal_compiler_get_shader_resources(
    compiler: *const bindings::ScInternalCompilerBase,
    shader_resources: *mut bindings::ScShaderResources,
) -> bindings::ScInternalResult {
    let module = emscripten::get_module();
    unsafe {
        let num_bytes = std::mem::size_of::<bindings::ScShaderResources>();
        let shader_resources_ptr = module.allocate(num_bytes as u32);
        let result = map_internal_result(_sc_internal_compiler_get_shader_resources(
            compiler as u32,
            shader_resources_ptr.as_offset(),
        ));
        module.read_bytes_into_pointer_while(
            shader_resources_ptr,
            |byte, bytes_read| bytes_read < num_bytes,
            false,
            shader_resources as *mut u8,
        );
        module.free(shader_resources_ptr);
        result
    }
}

pub fn sc_internal_compiler_get_specialization_constants(
    compiler: *const bindings::ScInternalCompilerBase,
    constants: *mut *mut bindings::ScSpecializationConstant,
    size: *mut usize,
) -> bindings::ScInternalResult {
    let module = emscripten::get_module();
    unsafe {
        let constants_ptr_to_ptr = module.allocate(U32_SIZE);
        let constants_size_ptr = module.allocate(U32_SIZE);
        let result = map_internal_result(_sc_internal_compiler_get_specialization_constants(
            compiler as u32,
            constants_ptr_to_ptr.as_offset(),
            constants_size_ptr.as_offset() as u32,
        ));
        *constants =
            module.get_u32(constants_ptr_to_ptr) as *mut bindings::ScSpecializationConstant;
        *size = module.get_u32(constants_size_ptr) as usize;
        module.free(constants_size_ptr);
        module.free(constants_ptr_to_ptr);
        result
    }
}

pub fn sc_internal_compiler_set_scalar_constant(
    compiler: *const bindings::ScInternalCompilerBase,
    id: u32,
    constant_high_bits: u32,
    constant_low_bits: u32,
) -> bindings::ScInternalResult {
    map_internal_result(_sc_internal_compiler_set_scalar_constant(
        compiler as u32,
        id,
        constant_high_bits,
        constant_low_bits,
    ))
}

pub fn sc_internal_compiler_get_type(
    compiler: *const bindings::ScInternalCompilerBase,
    id: u32,
    spirv_type: *mut *const bindings::ScType,
) -> bindings::ScInternalResult {
    let module = emscripten::get_module();
    unsafe {
        let type_ptr_to_ptr = module.allocate(U32_SIZE);
        let result = map_internal_result(_sc_internal_compiler_get_type(
            compiler as u32,
            id,
            type_ptr_to_ptr.as_offset(),
        ));
        let type_ptr = module.get_u32(type_ptr_to_ptr);
        *spirv_type = type_ptr as *const bindings::ScType;
        module.free(type_ptr_to_ptr);
        result
    }
}

pub fn sc_internal_compiler_get_member_name(
    compiler: *const bindings::ScInternalCompilerBase,
    id: u32,
    index: u32,
    name: *mut *const ::std::os::raw::c_char,
) -> bindings::ScInternalResult {
    let module = emscripten::get_module();
    unsafe {
        let name_ptr_to_ptr = module.allocate(U32_SIZE);
        let result = map_internal_result(_sc_internal_compiler_get_member_name(
            compiler as u32,
            id,
            index,
            name_ptr_to_ptr.as_offset(),
        ));
        let name_ptr = module.get_u32(name_ptr_to_ptr);
        *name = name_ptr as *const ::std::os::raw::c_char;
        module.free(name_ptr_to_ptr);
        result
    }
}

pub fn sc_internal_compiler_get_member_decoration(
    compiler: *const bindings::ScInternalCompilerBase,
    id: u32,
    index: u32,
    decoration: bindings::spv::Decoration,
    result: *mut u32,
) -> bindings::ScInternalResult {
    let module = emscripten::get_module();
    unsafe {
        let result_ptr = module.allocate(U32_SIZE);
        let ret = map_internal_result(_sc_internal_compiler_get_member_decoration(
            compiler as u32,
            id,
            index,
            decoration as u32,
            result_ptr.as_offset(),
        ));
        *result = module.get_u32(result_ptr) as u32;
        module.free(result_ptr);
        ret
    }
}

pub fn sc_internal_compiler_set_member_decoration(
    compiler: *const bindings::ScInternalCompilerBase,
    id: u32,
    index: u32,
    decoration: bindings::spv::Decoration,
    argument: u32,
) -> bindings::ScInternalResult {
    map_internal_result(_sc_internal_compiler_set_member_decoration(
        compiler as u32,
        id,
        index,
        decoration as u32,
        argument,
    ))
}

pub fn sc_internal_compiler_get_declared_struct_size(
    compiler: *const bindings::ScInternalCompilerBase,
    id: u32,
    result: *mut u32,
) -> bindings::ScInternalResult {
    let module = emscripten::get_module();
    unsafe {
        let result_ptr = module.allocate(U32_SIZE);
        let ret = map_internal_result(_sc_internal_compiler_get_declared_struct_size(
            compiler as u32,
            id,
            result_ptr.as_offset(),
        ));
        *result = module.get_u32(result_ptr) as u32;
        module.free(result_ptr);
        ret
    }
}

pub fn sc_internal_compiler_get_declared_struct_member_size(
    compiler: *const bindings::ScInternalCompilerBase,
    id: u32,
    index: u32,
    result: *mut u32,
) -> bindings::ScInternalResult {
    let module = emscripten::get_module();
    unsafe {
        let result_ptr = module.allocate(U32_SIZE);
        let ret = map_internal_result(_sc_internal_compiler_get_declared_struct_member_size(
            compiler as u32,
            id,
            index,
            result_ptr.as_offset(),
        ));
        *result = module.get_u32(result_ptr) as u32;
        module.free(result_ptr);
        ret
    }
}

pub fn sc_internal_compiler_rename_interface_variable(
    compiler: *const bindings::ScInternalCompilerBase,
    resources: *const bindings::ScResource,
    resources_size: usize,
    location: u32,
    name: *const ::std::os::raw::c_char,
) -> bindings::ScInternalResult {
    let module = emscripten::get_module();
    unsafe {
        let mut resources_copied = std::slice::from_raw_parts(resources, resources_size).to_vec();

        for mut resource in &mut resources_copied {
            // Update resource name to point to Emscripten heap instead
            let resource_name_bytes = CStr::from_ptr(resource.name).to_bytes();
            let resource_name_ptr = module.allocate(resource_name_bytes.len() as u32);
            module.set_from_u8_slice(resource_name_ptr, resource_name_bytes);
            resource.name = resource_name_ptr.as_offset() as *mut std::os::raw::c_char;
        }

        let resources_ptr = module.allocate(std::mem::size_of::<bindings::ScResource>() as u32);
        module.set_from_u8_slice(
            resources_ptr,
            std::slice::from_raw_parts(
                resources_copied.as_ptr() as *const u8,
                resources_size * std::mem::size_of::<bindings::ScResource>(),
            ),
        );
        let name_bytes = CStr::from_ptr(name).to_bytes();
        let name_ptr = module.allocate(name_bytes.len() as u32);
        module.set_from_u8_slice(name_ptr, name_bytes);
        let result = map_internal_result(_sc_internal_compiler_rename_interface_variable(
            compiler as u32,
            resources_ptr.as_offset(),
            resources_size as u32,
            location,
            name_ptr.as_offset(),
        ));

        for resource in resources_copied {
            module.free(emscripten::Pointer::from_offset(resource.name as u32));
        }

        module.free(name_ptr);
        result
    }
}

pub fn sc_internal_compiler_get_work_group_size_specialization_constants(
    compiler: *const bindings::ScInternalCompilerBase,
    constants: *mut *mut bindings::ScSpecializationConstant,
) -> bindings::ScInternalResult {
    let module = emscripten::get_module();
    let constants_length = 3; // x, y, z
    unsafe {
        let constants_ptr_to_ptr = module.allocate(
            std::mem::size_of::<bindings::ScSpecializationConstant>() as u32 * constants_length,
        );
        let result = map_internal_result(
            _sc_internal_compiler_get_work_group_size_specialization_constants(
                compiler as u32,
                constants_ptr_to_ptr.as_offset(),
            ),
        );
        let constants_ptr = module.get_u32(constants_ptr_to_ptr);
        *constants = constants_ptr as *mut bindings::ScSpecializationConstant;
        module.free(constants_ptr_to_ptr);
        result
    }
}

pub fn sc_internal_compiler_compile(
    compiler: *const bindings::ScInternalCompilerBase,
    shader: *mut *const ::std::os::raw::c_char,
) -> bindings::ScInternalResult {
    let module = emscripten::get_module();
    unsafe {
        let shader_ptr_to_ptr = module.allocate(U32_SIZE);
        let result = map_internal_result(_sc_internal_compiler_compile(
            compiler as u32,
            shader_ptr_to_ptr.as_offset(),
        ));
        let shader_ptr = module.get_u32(shader_ptr_to_ptr);
        *shader = shader_ptr as *const ::std::os::raw::c_char;
        module.free(shader_ptr_to_ptr);
        result
    }
}

pub fn sc_internal_compiler_delete(
    compiler: *mut bindings::ScInternalCompilerBase,
) -> bindings::ScInternalResult {
    map_internal_result(_sc_internal_compiler_delete(compiler as u32))
}

pub fn sc_internal_free_pointer(
    pointer: *mut ::std::os::raw::c_void,
) -> bindings::ScInternalResult {
    map_internal_result(_sc_internal_free_pointer(pointer as u32))
}
