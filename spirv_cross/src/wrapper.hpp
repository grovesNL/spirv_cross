#include "vendor/SPIRV-Cross/spirv.hpp"
#include "vendor/SPIRV-Cross/spirv_hlsl.hpp"
#include "vendor/SPIRV-Cross/spirv_msl.hpp"

typedef void ScInternalCompilerBase;
typedef void ScInternalCompilerHlsl;
typedef void ScInternalCompilerMsl;
typedef void ScInternalCompilerGlsl;

extern "C" {

enum ScInternalResult
{
    Success,
    Unhandled,
    CompilationError,
};

typedef struct ScEntryPoint
{
    char *name;
    spv::ExecutionModel execution_model;
    uint32_t work_group_size_x;
    uint32_t work_group_size_y;
    uint32_t work_group_size_z;
} ScEntryPoint;

typedef struct ScHlslRootConstant
{
    uint32_t start;
    uint32_t end;
} ScHlslRootConstant;

typedef struct ScHlslCompilerOptions
{
    int32_t shader_model;
    bool vertex_transform_clip_space;
    bool vertex_invert_y;
    const ScHlslRootConstant *root_constants_layout;
    size_t num_root_constants;
} ScHlslCompilerOptions;

typedef struct ScMslCompilerOptions
{
    bool vertex_transform_clip_space;
    bool vertex_invert_y;
} ScMslCompilerOptions;

typedef struct ScGlslCompilerOptions
{
    bool vertex_transform_clip_space;
    bool vertex_invert_y;
    uint32_t version;
    bool es;
} ScGlslCompilerOptions;

typedef struct ScResource
{
    uint32_t id;
    uint32_t type_id;
    uint32_t base_type_id;
    char *name;
} ScResource;

typedef struct ScResourceArray
{
    ScResource *data;
    size_t num;
} ScResourceArray;

typedef struct ScShaderResources
{
    ScResourceArray uniform_buffers;
    ScResourceArray storage_buffers;
    ScResourceArray stage_inputs;
    ScResourceArray stage_outputs;
    ScResourceArray subpass_inputs;
    ScResourceArray storage_images;
    ScResourceArray sampled_images;
    ScResourceArray atomic_counters;
    ScResourceArray push_constant_buffers;
    ScResourceArray separate_images;
    ScResourceArray separate_samplers;
} ScShaderResources;

typedef struct ScSpecializationConstant
{
    uint32_t id;
    uint32_t constant_id;
} ScSpecializationConstant;

ScInternalResult sc_internal_get_latest_exception_message(const char **message);

ScInternalResult sc_internal_compiler_hlsl_new(ScInternalCompilerHlsl **compiler, const uint32_t *ir, size_t size);
ScInternalResult sc_internal_compiler_hlsl_set_options(const ScInternalCompilerHlsl *compiler, const ScHlslCompilerOptions *options);

ScInternalResult sc_internal_compiler_msl_new(ScInternalCompilerMsl **compiler, const uint32_t *ir, size_t size);
ScInternalResult sc_internal_compiler_msl_set_options(const ScInternalCompilerHlsl *compiler, const ScMslCompilerOptions *options);
ScInternalResult sc_internal_compiler_msl_compile(const ScInternalCompilerBase *compiler, const char **shader,
                                                  const spirv_cross::MSLVertexAttr *p_vat_overrides, size_t vat_override_count,
                                                  const spirv_cross::MSLResourceBinding *p_res_overrides, size_t res_override_count);

ScInternalResult sc_internal_compiler_glsl_new(ScInternalCompilerGlsl **compiler, const uint32_t *ir, size_t size);
ScInternalResult sc_internal_compiler_glsl_set_options(const ScInternalCompilerGlsl *compiler, const ScGlslCompilerOptions *options);
ScInternalResult sc_internal_compiler_glsl_build_combined_image_samplers(const ScInternalCompilerBase *compiler);

ScInternalResult sc_internal_compiler_get_decoration(const ScInternalCompilerBase *compiler, uint32_t *result, uint32_t id, spv::Decoration decoration);
ScInternalResult sc_internal_compiler_set_decoration(const ScInternalCompilerBase *compiler, uint32_t id, spv::Decoration decoration, uint32_t argument);
ScInternalResult sc_internal_compiler_get_entry_points(const ScInternalCompilerBase *compiler, ScEntryPoint **entry_points, size_t *size);
ScInternalResult sc_internal_compiler_get_cleansed_entry_point_name(const ScInternalCompilerBase *compiler, const char *original_entry_point_name, const char **compiled_entry_point_name);
ScInternalResult sc_internal_compiler_get_shader_resources(const ScInternalCompilerBase *compiler, ScShaderResources *shader_resources);
ScInternalResult sc_internal_compiler_get_specialization_constants(const ScInternalCompilerBase *compiler, ScSpecializationConstant **constants, size_t *size);
ScInternalResult sc_internal_compiler_set_scalar_constant(const ScInternalCompilerBase *compiler, uint32_t id, uint64_t constant);
ScInternalResult sc_internal_compiler_compile(const ScInternalCompilerBase *compiler, const char **shader);
ScInternalResult sc_internal_compiler_delete(ScInternalCompilerBase *compiler);

ScInternalResult sc_internal_free_pointer(void *pointer);
}
