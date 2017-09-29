#include "vendor/SPIRV-Cross/spirv.hpp"
#include "vendor/SPIRV-Cross/spirv_hlsl.hpp"

typedef void ScInternalCompilerBase;
typedef void ScInternalCompilerHlsl;

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
    uint32_t workgroup_size_x;
    uint32_t workgroup_size_y;
    uint32_t workgroup_size_z;
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

ScInternalResult sc_internal_get_latest_exception_message(const char **message);

ScInternalResult sc_internal_compiler_base_parse(const uint32_t *ir, size_t size, ScEntryPoint **entry_points, size_t *entry_points_size);

ScInternalResult sc_internal_compiler_hlsl_compile(const uint32_t *ir, size_t size, const char **hlsl, const ScHlslCompilerOptions *options);

ScInternalResult sc_internal_compiler_msl_compile(const uint32_t *ir, size_t size, const char **msl, const ScMslCompilerOptions *options);

ScInternalResult sc_internal_free_pointer(void *pointer);
}