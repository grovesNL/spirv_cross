use std::process::Command;

fn main() {
    let output = Command::new("emcc")
        .args(&[
            "-O3",
            "-std=c++14",
            "-fno-exceptions",
            "-DSPIRV_CROSS_EXCEPTIONS_TO_ASSERTIONS=1",
            "-DSPIRV_CROSS_WRAPPER_NO_EXCEPTIONS=1",
            "-DSPIRV_CROSS_WRAPPER_GLSL",
            "--closure",
            "1",
            "-o",
            "spirv_cross_wrapper_glsl.js",
            "-s",
            "MODULARIZE=1",
            "-s",
            "EXPORT_NAME=sc_internal_wrapper",
            "-s",
            "WASM=1",
            "-s",
            "ALLOW_MEMORY_GROWTH=1",
            "-s",
            r#"EXPORTED_FUNCTIONS=[
		        "_sc_internal_get_latest_exception_message",
                "_sc_internal_compiler_glsl_new",
		        "_sc_internal_compiler_glsl_set_options",
		        "_sc_internal_compiler_glsl_build_combined_image_samplers",
		        "_sc_internal_compiler_glsl_get_combined_image_samplers",
		        "_sc_internal_compiler_get_decoration",
		        "_sc_internal_compiler_set_decoration",
		        "_sc_internal_compiler_unset_decoration",
		        "_sc_internal_compiler_set_name",
		        "_sc_internal_compiler_get_entry_points",
		        "_sc_internal_compiler_get_cleansed_entry_point_name",
                "_sc_internal_compiler_get_shader_resources",
		        "_sc_internal_compiler_get_specialization_constants",
		        "_sc_internal_compiler_set_scalar_constant",
		        "_sc_internal_compiler_get_type",
		        "_sc_internal_compiler_get_member_name",
		        "_sc_internal_compiler_get_member_decoration",
		        "_sc_internal_compiler_set_member_decoration",
		        "_sc_internal_compiler_get_declared_struct_size",
		        "_sc_internal_compiler_get_declared_struct_member_size",
		        "_sc_internal_compiler_rename_interface_variable",
		        "_sc_internal_compiler_get_work_group_size_specialization_constants",
		        "_sc_internal_compiler_compile",
		        "_sc_internal_compiler_delete",
		        "_sc_internal_free_pointer"
            ]"#,
	        "-s",
            "../spirv_cross/src/wrapper.cpp",
            "../spirv_cross/src/vendor/SPIRV-Cross/spirv_cfg.cpp",
            "../spirv_cross/src/vendor/SPIRV-Cross/spirv_cross.cpp",
            "../spirv_cross/src/vendor/SPIRV-Cross/spirv_cross_parsed_ir.cpp",
            "../spirv_cross/src/vendor/SPIRV-Cross/spirv_parser.cpp",
            "../spirv_cross/src/vendor/SPIRV-Cross/spirv_cross_util.cpp",
            "../spirv_cross/src/vendor/SPIRV-Cross/spirv_glsl.cpp"
        ])
        .output()
        .expect("Failed to run emcc");
    println!("{:?}", output);
}
