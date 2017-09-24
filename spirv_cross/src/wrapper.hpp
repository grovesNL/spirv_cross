#include "vendor/SPIRV-Cross/spirv.hpp"
#include "vendor/SPIRV-Cross/spirv_hlsl.hpp"

typedef void ScInternalCompilerHlsl;

extern "C" {
enum ScInternalResult
{
    Success = 0,
    Unhandled = 1
};
ScInternalResult sc_internal_compiler_hlsl_new(ScInternalCompilerHlsl **compiler, const uint32_t *ir, size_t size);
ScInternalResult sc_internal_compiler_hlsl_delete(const ScInternalCompilerHlsl *compiler);
ScInternalResult sc_internal_compiler_hlsl_compile(const ScInternalCompilerHlsl *compiler, char **compiled);
ScInternalResult sc_internal_deallocate_string(char *str);
}