#include "vendor/SPIRV-Cross/spirv_hlsl.hpp"
#include "wrapper.hpp"

#define INTERNAL_RESULT(block)              \
    try                                     \
    {                                       \
        {                                   \
            block                           \
        }                                   \
        return ScInternalResult::Success;   \
    }                                       \
    catch (const std::exception &ex)        \
    {                                       \
        return ScInternalResult::Unhandled; \
    }                                       \
    catch (...)                             \
    {                                       \
        return ScInternalResult::Unhandled; \
    }                                       \
    return ScInternalResult::Unhandled;

extern "C" {
ScInternalResult sc_internal_compiler_hlsl_new(ScInternalCompilerHlsl **compiler, const uint32_t *ir, size_t size)
{
    INTERNAL_RESULT(
            *compiler = new spirv_cross::CompilerHLSL(ir, size);)
}
ScInternalResult sc_internal_compiler_hlsl_delete(ScInternalCompilerHlsl *compiler)
{
    INTERNAL_RESULT(
        delete (spirv_cross::CompilerHLSL *)compiler;)
}
ScInternalResult sc_internal_compiler_hlsl_compile(const ScInternalCompilerHlsl *compiler, char **hlsl)
{
    INTERNAL_RESULT(*hlsl = strdup(((spirv_cross::CompilerHLSL *)compiler)->compile().c_str());)
}
ScInternalResult sc_internal_deallocate_string(char *str)
{
    INTERNAL_RESULT(free(str);)
}
}
