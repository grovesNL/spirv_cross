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
ScInternalResult sc_internal_compiler_base_get_entry_points(const ScInternalCompilerBase *compiler, ScEntryPoint **entry_points, size_t *size)
{
    INTERNAL_RESULT(
        auto const &entry_point_names = ((spirv_cross::Compiler *)compiler)->get_entry_points();
        auto const &len = entry_point_names.size();
        ScEntryPoint *eps = (ScEntryPoint *)malloc(len * sizeof(ScEntryPoint));
        size_t i = 0;
        for (auto const &name
             : entry_point_names) {
            auto const &entry_point = ((spirv_cross::Compiler *)compiler)->get_entry_point(name);
            eps[i].name = strdup(name.c_str());
            eps[i].execution_model = entry_point.model;
            eps[i].workgroup_size_x = entry_point.workgroup_size.x;
            eps[i].workgroup_size_y = entry_point.workgroup_size.y;
            eps[i].workgroup_size_z = entry_point.workgroup_size.z;
            i++;
        }
            *size = len;
        *entry_points = eps;)
}
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
ScInternalResult sc_internal_free_pointer(void *pointer)
{
    INTERNAL_RESULT(free(pointer);)
}
}
