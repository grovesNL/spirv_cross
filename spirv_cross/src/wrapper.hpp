#include "vendor/SPIRV-Cross/spirv.hpp"
#include "vendor/SPIRV-Cross/spirv_hlsl.hpp"

typedef void ScInternalCompilerBase;
typedef void ScInternalCompilerHlsl;

extern "C" {

enum ScInternalResult
{
    Success = 0,
    Unhandled = 1
};

typedef struct ScEntryPoint
{
    char *name;
    spv::ExecutionModel execution_model;
    uint32_t workgroup_size_x;
    uint32_t workgroup_size_y;
    uint32_t workgroup_size_z;
} ScEntryPoint;

ScInternalResult sc_internal_compiler_base_get_entry_points(const ScInternalCompilerBase *compiler, ScEntryPoint **entry_points, size_t *size);
ScInternalResult sc_internal_compiler_hlsl_new(ScInternalCompilerHlsl **compiler, const uint32_t *ir, size_t size);
ScInternalResult sc_internal_compiler_hlsl_delete(ScInternalCompilerHlsl *compiler);
ScInternalResult sc_internal_compiler_hlsl_compile(const ScInternalCompilerHlsl *compiler, char **hlsl);
ScInternalResult sc_internal_free_pointer(void *pointer);
}