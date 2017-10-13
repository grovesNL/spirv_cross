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

ScInternalResult sc_internal_compiler_hlsl_new(ScInternalCompilerHlsl **compiler, const uint32_t *ir, size_t size)
{
    INTERNAL_RESULT(*compiler = new spirv_cross::CompilerHLSL(ir, size);)
}

ScInternalResult sc_internal_compiler_hlsl_set_options(const ScInternalCompilerHlsl *compiler, const ScHlslCompilerOptions *options)
{
    INTERNAL_RESULT(
        do {
            // Unsafe
            auto compiler_glsl = (spirv_cross::CompilerGLSL *)compiler;
            auto glsl_options = compiler_glsl->spirv_cross::CompilerGLSL::get_options();
            glsl_options.vertex.fixup_clipspace = options->vertex_transform_clip_space;
            glsl_options.vertex.flip_vert_y = options->vertex_invert_y;
            compiler_glsl->spirv_cross::CompilerGLSL::set_options(glsl_options);

            auto compiler_hlsl = (spirv_cross::CompilerHLSL *)compiler;
            auto hlsl_options = compiler_hlsl->get_options();
            hlsl_options.shader_model = options->shader_model;
            compiler_hlsl->set_options(hlsl_options);
        } while (0);)
}

ScInternalResult sc_internal_compiler_msl_new(ScInternalCompilerMsl **compiler, const uint32_t *ir, size_t size,
                                              const spirv_cross::MSLVertexAttr *p_vat_overrides, size_t vat_override_count,
                                              const spirv_cross::MSLResourceBinding *p_res_overrides, size_t res_override_count)
{
    INTERNAL_RESULT(do {
            *compiler = new spirv_cross::CompilerMSL(ir, size,
                                                     (spirv_cross::MSLVertexAttr *)p_vat_overrides, vat_override_count,
                                                     (spirv_cross::MSLResourceBinding *)p_res_overrides, res_override_count);
        } while (0);)
}

ScInternalResult sc_internal_compiler_msl_set_options(const ScInternalCompilerMsl *compiler, const ScMslCompilerOptions *options)
{
    INTERNAL_RESULT(
        do {
            // Unsafe
            auto compiler_glsl = (spirv_cross::CompilerGLSL *)compiler;
            auto glsl_options = compiler_glsl->spirv_cross::CompilerGLSL::get_options();
            glsl_options.vertex.fixup_clipspace = options->vertex_transform_clip_space;
            glsl_options.vertex.flip_vert_y = options->vertex_invert_y;
            compiler_glsl->spirv_cross::CompilerGLSL::set_options(glsl_options);
        } while (0);)
}

ScInternalResult sc_internal_compiler_get_decoration(const ScInternalCompilerBase *compiler, uint32_t *result, uint32_t id, spv::Decoration decoration)
{
    INTERNAL_RESULT(*result = ((spirv_cross::Compiler *)compiler)->get_decoration(id, decoration);)
}

ScInternalResult sc_internal_compiler_set_decoration(const ScInternalCompilerBase *compiler, uint32_t id, spv::Decoration decoration, uint32_t argument)
{
    INTERNAL_RESULT(((spirv_cross::Compiler *)compiler)->set_decoration(id, decoration, argument);)
}

ScInternalResult sc_internal_compiler_get_entry_points(const ScInternalCompilerBase *comp, ScEntryPoint **entry_points, size_t *size)
{
    INTERNAL_RESULT(
        do {
            // Unsafe
            auto const &compiler = *((spirv_cross::Compiler *)comp);
            auto const &sc_entry_point_names = compiler.get_entry_points();
            auto const sc_size = sc_entry_point_names.size();
            auto const &sc_entry_points = std::make_unique<spirv_cross::SPIREntryPoint[]>(sc_size);
            for (uint32_t i = 0; i < sc_size; i++)
            {
                sc_entry_points[i] = compiler.get_entry_point(sc_entry_point_names[i]);
            }

            // Release to FFI
            *entry_points = (ScEntryPoint *)malloc(sc_size * sizeof(ScEntryPoint));
            *size = sc_size;
            for (uint32_t i = 0; i < sc_size; i++)
            {
                auto const &sc_entry_point = sc_entry_points[i];
                entry_points[i]->name = strdup(sc_entry_point.name.c_str());
                entry_points[i]->execution_model = sc_entry_point.model;
                entry_points[i]->work_group_size_x = sc_entry_point.workgroup_size.x;
                entry_points[i]->work_group_size_y = sc_entry_point.workgroup_size.y;
                entry_points[i]->work_group_size_z = sc_entry_point.workgroup_size.z;
            }
        } while (0);)
}

void fill_resource_array(ScResourceArray *resources, const std::vector<spirv_cross::Resource> &sc_resources)
{
    auto const sc_size = sc_resources.size();

    if (sc_size == 0)
    {
        resources->num = 0;
        resources->data = 0x0;
        return;
    }

    resources->num = sc_size;
    resources->data = (ScResource *)malloc(sc_size * sizeof(ScResource));
    for (uint32_t i = 0; i < sc_size; i++)
    {
        auto const &resource = sc_resources[i];
        resources->data[i].id = resource.id;
        resources->data[i].type_id = resource.type_id;
        resources->data[i].base_type_id = resource.base_type_id;
        resources->data[i].name = strdup(resource.name.c_str());
    }
}

ScInternalResult sc_internal_compiler_get_shader_resources(const ScInternalCompilerBase *compiler, ScShaderResources *shader_resources)
{
    INTERNAL_RESULT(
        do {
            // Unsafe
            auto const sc_resources = ((const spirv_cross::Compiler *)compiler)->get_shader_resources();

            fill_resource_array(&shader_resources->uniform_buffers, sc_resources.uniform_buffers);
            fill_resource_array(&shader_resources->storage_buffers, sc_resources.storage_buffers);
            fill_resource_array(&shader_resources->stage_inputs, sc_resources.stage_inputs);
            fill_resource_array(&shader_resources->stage_outputs, sc_resources.stage_outputs);
            fill_resource_array(&shader_resources->subpass_inputs, sc_resources.subpass_inputs);
            fill_resource_array(&shader_resources->storage_images, sc_resources.storage_images);
            fill_resource_array(&shader_resources->sampled_images, sc_resources.sampled_images);
            fill_resource_array(&shader_resources->atomic_counters, sc_resources.atomic_counters);
            fill_resource_array(&shader_resources->push_constant_buffers, sc_resources.push_constant_buffers);
            fill_resource_array(&shader_resources->separate_images, sc_resources.separate_images);
            fill_resource_array(&shader_resources->separate_samplers, sc_resources.separate_samplers);
        } while (0);)
}

ScInternalResult sc_internal_compiler_compile(const ScInternalCompilerBase *compiler, const char **shader)
{
    INTERNAL_RESULT(
        do {
            // Unsafe
            // Release to FFI
            *shader = strdup(strdup(((spirv_cross::Compiler *)compiler)->compile().c_str()));
        } while (0);)
}

ScInternalResult sc_internal_compiler_delete(ScInternalCompilerBase *compiler)
{
    INTERNAL_RESULT(delete (spirv_cross::Compiler *)compiler;)
}

ScInternalResult sc_internal_free_pointer(void *pointer)
{
    INTERNAL_RESULT(free(pointer);)
}
}
