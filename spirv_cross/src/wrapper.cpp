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
ScInternalResult sc_internal_compiler_base_parse(const uint32_t *ir, size_t size, ScEntryPoint **entry_points, size_t *entry_points_size)
{
    INTERNAL_RESULT(
        auto const &compiler = new spirv_cross::Compiler(ir, size);
        auto const &entry_point_names = compiler->get_entry_points();
        auto const &len = entry_point_names.size();
        ScEntryPoint *eps = (ScEntryPoint *)malloc(len * sizeof(ScEntryPoint));
        size_t i = 0;
        for (auto const &name
             : entry_point_names) {
            auto const &entry_point = compiler->get_entry_point(name);
            eps[i].name = strdup(name.c_str());
            eps[i].execution_model = entry_point.model;
            eps[i].workgroup_size_x = entry_point.workgroup_size.x;
            eps[i].workgroup_size_y = entry_point.workgroup_size.y;
            eps[i].workgroup_size_z = entry_point.workgroup_size.z;
            i++;
        }
            *entry_points = eps;
        *entry_points_size = len;
        delete compiler;)
}

ScInternalResult sc_internal_compiler_hlsl_compile(const uint32_t *ir, size_t size, char **hlsl, const ScHlslCompilerOptions *options)
{
    INTERNAL_RESULT(
        auto const &compiler = new spirv_cross::CompilerHLSL(ir, size);

        auto glsl_options = compiler->spirv_cross::CompilerGLSL::get_options();
        glsl_options.vertex.fixup_clipspace = options->vertex_transform_clip_space;
        glsl_options.vertex.flip_vert_y = options->vertex_invert_y;
        compiler->spirv_cross::CompilerGLSL::set_options(glsl_options);

        auto hlsl_options = compiler->get_options();
        hlsl_options.shader_model = options->shader_model;
        compiler->set_options(hlsl_options);

        *hlsl = strdup(compiler->compile().c_str());
        delete compiler;)
}

ScInternalResult sc_internal_free_pointer(void *pointer)
{
    INTERNAL_RESULT(free(pointer);)
}
}
