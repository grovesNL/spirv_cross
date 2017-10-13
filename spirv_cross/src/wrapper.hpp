#include "vendor/SPIRV-Cross/spirv.hpp"
#include "vendor/SPIRV-Cross/spirv_hlsl.hpp"
#include "vendor/SPIRV-Cross/spirv_msl.hpp"

typedef void ScInternalCompilerBase;
typedef void ScInternalCompilerHlsl;
typedef void ScInternalCompilerMsl;

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

typedef struct ScHlslCompilerOptions
{
    int32_t shader_model;
    bool vertex_transform_clip_space;
    bool vertex_invert_y;
} ScHlslCompilerOptions;

typedef struct ScMslCompilerOptions
{
    bool vertex_transform_clip_space;
    bool vertex_invert_y;
} ScMslCompilerOptions;

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


ScInternalResult sc_internal_get_latest_exception_message(const char **message);

ScInternalResult sc_internal_compiler_hlsl_new(ScInternalCompilerHlsl **compiler, const uint32_t *ir, size_t size);
ScInternalResult sc_internal_compiler_hlsl_set_options(const ScInternalCompilerHlsl *compiler, const ScHlslCompilerOptions *options);

ScInternalResult sc_internal_compiler_msl_new(ScInternalCompilerMsl **compiler, const uint32_t *ir, size_t size,
                                              const spirv_cross::MSLVertexAttr *p_vat_overrides, size_t vat_override_count,
                                              const spirv_cross::MSLResourceBinding *p_res_overrides, size_t res_override_count);
ScInternalResult sc_internal_compiler_msl_set_options(const ScInternalCompilerHlsl *compiler, const ScMslCompilerOptions *options);

ScInternalResult sc_internal_compiler_get_decoration(const ScInternalCompilerBase *compiler, uint32_t *result, uint32_t id, spv::Decoration decoration);
ScInternalResult sc_internal_compiler_set_decoration(const ScInternalCompilerBase *compiler, uint32_t id, spv::Decoration decoration, uint32_t argument);
ScInternalResult sc_internal_compiler_get_entry_points(const ScInternalCompilerBase *compiler, ScEntryPoint **entry_points, size_t *size);
ScInternalResult sc_internal_compiler_get_shader_resources(const ScInternalCompilerBase *compiler, ScShaderResources *shader_resources);
ScInternalResult sc_internal_compiler_compile(const ScInternalCompilerBase *compiler, const char **shader);
ScInternalResult sc_internal_compiler_delete(ScInternalCompilerBase *compiler);

ScInternalResult sc_internal_free_pointer(void *pointer);
}
