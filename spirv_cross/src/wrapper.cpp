#include "vendor/SPIRV-Cross/spirv_hlsl.hpp"
#include "vendor/SPIRV-Cross/spirv_msl.hpp"
#include "wrapper.hpp"

static const char *latest_exception_message;

#define INTERNAL_RESULT(block_to_attempt)                 \
    do                                                    \
    {                                                     \
        try                                               \
        {                                                 \
            {                                             \
                block_to_attempt                          \
            }                                             \
            return ScInternalResult::Success;             \
        }                                                 \
        catch (const spirv_cross::CompilerError &ex)      \
        {                                                 \
            latest_exception_message = strdup(ex.what()); \
            return ScInternalResult::CompilationError;    \
        }                                                 \
        catch (const std::exception &ex)                  \
        {                                                 \
            return ScInternalResult::Unhandled;           \
        }                                                 \
        catch (...)                                       \
        {                                                 \
            return ScInternalResult::Unhandled;           \
        }                                                 \
        return ScInternalResult::Unhandled;               \
    } while (0);

extern "C" {
ScInternalResult sc_internal_get_latest_exception_message(const char **message)
{
    INTERNAL_RESULT(
            *message = latest_exception_message;)
}

ScInternalResult sc_internal_compiler_base_parse(const uint32_t *ir, size_t size, ScEntryPoint **entry_points, size_t *entry_points_size)
{
    INTERNAL_RESULT(
        do {
            // Unsafe
            spirv_cross::Compiler compiler(ir, size);
            auto const &sc_entry_point_names = compiler.get_entry_points();
            auto const sc_size = sc_entry_point_names.size();
            auto const &sc_entry_points = std::make_unique<spirv_cross::SPIREntryPoint[]>(size);
            for (uint32_t i = 0; i < sc_size; i++)
            {
                sc_entry_points[i] = compiler.get_entry_point(sc_entry_point_names[i]);
            }

            // Release to FFI
            *entry_points = (ScEntryPoint *)malloc(size * sizeof(ScEntryPoint));
            *entry_points_size = sc_size;
            for (uint32_t i = 0; i < sc_size; i++)
            {
                auto const &sc_entry_point = sc_entry_points[i];
                entry_points[i]->name = strdup(sc_entry_point.name.c_str());
                entry_points[i]->execution_model = sc_entry_point.model;
                entry_points[i]->workgroup_size_x = sc_entry_point.workgroup_size.x;
                entry_points[i]->workgroup_size_y = sc_entry_point.workgroup_size.y;
                entry_points[i]->workgroup_size_z = sc_entry_point.workgroup_size.z;
                i++;
            }
        } while (0);)
}

ScInternalResult sc_internal_compiler_hlsl_compile(const uint32_t *ir, size_t size, const char **hlsl, const ScHlslCompilerOptions *options)
{
    INTERNAL_RESULT(
        do {
            // Unsafe
            spirv_cross::CompilerHLSL compiler(ir, size);

            auto glsl_options = compiler.spirv_cross::CompilerGLSL::get_options();
            glsl_options.vertex.fixup_clipspace = options->vertex_transform_clip_space;
            glsl_options.vertex.flip_vert_y = options->vertex_invert_y;
            compiler.spirv_cross::CompilerGLSL::set_options(glsl_options);

            auto hlsl_options = compiler.get_options();
            hlsl_options.shader_model = options->shader_model;
            compiler.set_options(hlsl_options);

            auto const &compiled = compiler.compile();

            // Release to FFI
            *hlsl = strdup(strdup(compiled.c_str()));
        } while (0);)
}

ScInternalResult sc_internal_compiler_msl_compile(const uint32_t *ir, size_t size, const char **msl, const ScMslCompilerOptions *options)
{
    INTERNAL_RESULT(
        do {
            // Unsafe
            spirv_cross::CompilerMSL compiler(ir, size);

            auto glsl_options = compiler.spirv_cross::CompilerGLSL::get_options();
            glsl_options.vertex.fixup_clipspace = options->vertex_transform_clip_space;
            glsl_options.vertex.flip_vert_y = options->vertex_invert_y;
            compiler.spirv_cross::CompilerGLSL::set_options(glsl_options);

            auto const &compiled = compiler.compile();

            // Release to FFI
            *msl = strdup(strdup(compiled.c_str()));
        } while (0);)
}

ScInternalResult sc_internal_free_pointer(void *pointer)
{
    INTERNAL_RESULT(free(pointer);)
}
}
