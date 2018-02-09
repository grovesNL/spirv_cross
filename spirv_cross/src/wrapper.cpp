#pragma warning(disable : 4996 4101)
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
    INTERNAL_RESULT(*message = latest_exception_message;)
}

ScInternalResult sc_internal_compiler_hlsl_new(ScInternalCompilerHlsl **compiler, const uint32_t *ir, const size_t size)
{
    INTERNAL_RESULT(*compiler = new spirv_cross::CompilerHLSL(ir, size);)
}

ScInternalResult sc_internal_compiler_hlsl_set_options(const ScInternalCompilerHlsl *compiler, const ScHlslCompilerOptions *options)
{
    INTERNAL_RESULT(
        do {
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

ScInternalResult sc_internal_compiler_hlsl_set_root_constant_layout(const ScInternalCompilerHlsl *compiler, const ScHlslRootConstant *constants, size_t count)
{
    INTERNAL_RESULT(
        do {
            std::vector<spirv_cross::RootConstants> root_constants;
            for (int i = 0; i < count; i++)
            {
                root_constants.push_back(
                    spirv_cross::RootConstants{
                        constants[i].start,
                        constants[i].end,
                        constants[i].binding,
                        constants[i].space});
            }

            auto compiler_hlsl = (spirv_cross::CompilerHLSL *)compiler;
            compiler_hlsl->set_root_constant_layouts(root_constants);
        } while (0);)
}

ScInternalResult sc_internal_compiler_msl_new(ScInternalCompilerMsl **compiler, const uint32_t *ir, const size_t size)
{
    INTERNAL_RESULT(*compiler = new spirv_cross::CompilerMSL(ir, size);)
}

ScInternalResult sc_internal_compiler_msl_compile(const ScInternalCompilerBase *compiler, const char **shader,
                                                  const spirv_cross::MSLVertexAttr *p_vat_overrides, const size_t vat_override_count,
                                                  const spirv_cross::MSLResourceBinding *p_res_overrides, const size_t res_override_count)
{
    INTERNAL_RESULT(
        do {
            std::vector<spirv_cross::MSLVertexAttr> vat_overrides;
            if (p_vat_overrides)
            {
                vat_overrides.insert(vat_overrides.end(), &p_vat_overrides[0], &p_vat_overrides[vat_override_count]);
            }

            std::vector<spirv_cross::MSLResourceBinding> res_overrides;
            if (p_res_overrides)
            {
                res_overrides.insert(res_overrides.end(), &p_res_overrides[0], &p_res_overrides[res_override_count]);
            }

            *shader = strdup(((spirv_cross::CompilerMSL *)compiler)->compile(&vat_overrides, &res_overrides).c_str());
        } while (0);)
}

ScInternalResult sc_internal_compiler_msl_set_options(const ScInternalCompilerMsl *compiler, const ScMslCompilerOptions *options)
{
    INTERNAL_RESULT(
        do {
            auto compiler_glsl = (spirv_cross::CompilerGLSL *)compiler;
            auto glsl_options = compiler_glsl->spirv_cross::CompilerGLSL::get_options();
            glsl_options.vertex.fixup_clipspace = options->vertex_transform_clip_space;
            glsl_options.vertex.flip_vert_y = options->vertex_invert_y;
            compiler_glsl->spirv_cross::CompilerGLSL::set_options(glsl_options);
        } while (0);)
}

ScInternalResult sc_internal_compiler_glsl_new(ScInternalCompilerGlsl **compiler, const uint32_t *ir, const size_t size)
{
    INTERNAL_RESULT(*compiler = new spirv_cross::CompilerGLSL(ir, size);)
}

ScInternalResult sc_internal_compiler_glsl_set_options(const ScInternalCompilerGlsl *compiler, const ScGlslCompilerOptions *options)
{
    INTERNAL_RESULT(
        do {
            auto compiler_glsl = (spirv_cross::CompilerGLSL *)compiler;
            auto glsl_options = compiler_glsl->spirv_cross::CompilerGLSL::get_options();
            glsl_options.version = options->version;
            glsl_options.es = options->es;
            glsl_options.vertex.fixup_clipspace = options->vertex_transform_clip_space;
            glsl_options.vertex.flip_vert_y = options->vertex_invert_y;
            compiler_glsl->spirv_cross::CompilerGLSL::set_options(glsl_options);
        } while (0);)
}

ScInternalResult sc_internal_compiler_glsl_build_combined_image_samplers(const ScInternalCompilerBase *compiler)
{
    INTERNAL_RESULT(
        do {
            ((spirv_cross::CompilerGLSL *)compiler)->build_combined_image_samplers();
        } while (0);)
}

ScInternalResult sc_internal_compiler_get_decoration(const ScInternalCompilerBase *compiler, uint32_t *result, const uint32_t id, const spv::Decoration decoration)
{
    INTERNAL_RESULT(*result = ((spirv_cross::Compiler *)compiler)->get_decoration(id, decoration);)
}

ScInternalResult sc_internal_compiler_set_decoration(const ScInternalCompilerBase *compiler, const uint32_t id, const spv::Decoration decoration, const uint32_t argument)
{
    INTERNAL_RESULT(((spirv_cross::Compiler *)compiler)->set_decoration(id, decoration, argument);)
}

ScInternalResult sc_internal_compiler_get_entry_points(const ScInternalCompilerBase *compiler, ScEntryPoint **entry_points, size_t *size)
{
    INTERNAL_RESULT(
        do {
            auto const &comp = *((spirv_cross::Compiler *)compiler);
            auto const &sc_entry_point_names = comp.get_entry_points();
            auto const sc_size = sc_entry_point_names.size();
            auto const &sc_entry_points = std::make_unique<spirv_cross::SPIREntryPoint[]>(sc_size);
            for (uint32_t i = 0; i < sc_size; i++)
            {
                sc_entry_points[i] = comp.get_entry_point(sc_entry_point_names[i]);
            }

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

ScInternalResult sc_internal_compiler_get_cleansed_entry_point_name(const ScInternalCompilerBase *compiler, const char *original_entry_point_name, const char **compiled_entry_point_name)
{
    INTERNAL_RESULT(
        do {
            *compiled_entry_point_name = strdup(
                (*((spirv_cross::Compiler *)compiler))
                    .get_cleansed_entry_point_name(std::string(original_entry_point_name))
                    .c_str());
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

ScInternalResult sc_internal_compiler_get_specialization_constants(const ScInternalCompilerBase *compiler, ScSpecializationConstant **constants, size_t *size)
{
    INTERNAL_RESULT(
        do {
            auto const sc_constants = ((const spirv_cross::Compiler *)compiler)->get_specialization_constants();
            auto const sc_size = sc_constants.size();

            *constants = (ScSpecializationConstant *)malloc(sc_size * sizeof(ScSpecializationConstant));
            *size = sc_size;
            for (uint32_t i = 0; i < sc_size; i++)
            {
                auto const &sc_constant = sc_constants[i];
                constants[i]->id = sc_constant.id;
                constants[i]->constant_id = sc_constant.constant_id;
            }

        } while (0);)
}

ScInternalResult sc_internal_compiler_set_scalar_constant(const ScInternalCompilerBase *compiler, const uint32_t id, const uint64_t constant)
{
    INTERNAL_RESULT(
        do {
            auto &sc_constant = ((spirv_cross::Compiler *)compiler)->get_constant(id);
            sc_constant.m.c[0].r[0].u64 = constant;
        } while (0);)
}

ScInternalResult sc_internal_compiler_get_type(const ScInternalCompilerBase *compiler, const uint32_t id, const ScType **spirv_type)
{
    INTERNAL_RESULT(
        do {
            auto const &type = ((spirv_cross::Compiler *)compiler)->get_type(id);
            auto const member_types_size = type.member_types.size();

            auto ty = (ScType *)malloc(sizeof(ScType));
            ty->type = type.basetype;
            ty->member_types_size = member_types_size;

            if (member_types_size > 0)
            {
                auto const &member_types = (uint32_t *)malloc(member_types_size * sizeof(uint32_t));

                for (auto i = 0; i < member_types_size; i++)
                {
                    member_types[i] = type.member_types[i];
                }

                ty->member_types = member_types;
            }

            *spirv_type = ty;
        } while (0);)
}

ScInternalResult sc_internal_compiler_get_member_name(const ScInternalCompilerBase *compiler, const uint32_t id, const uint32_t index, const char **name)
{
    INTERNAL_RESULT(
        do {
            auto const member_name = ((spirv_cross::Compiler *)compiler)->get_member_name(id, index);
            *name = strdup(member_name.c_str());
        } while (0);)
}

ScInternalResult sc_internal_compiler_get_member_decoration(const ScInternalCompilerBase *compiler, const uint32_t id, const uint32_t index, const spv::Decoration decoration, uint32_t *result)
{
    INTERNAL_RESULT(*result = ((spirv_cross::Compiler *)compiler)->get_member_decoration(id, index, decoration);)
}

ScInternalResult sc_internal_compiler_set_member_decoration(const ScInternalCompilerBase *compiler, const uint32_t id, const uint32_t index, const spv::Decoration decoration, const uint32_t argument)
{
    INTERNAL_RESULT(((spirv_cross::Compiler *)compiler)->set_member_decoration(id, index, decoration, argument);)
}

ScInternalResult sc_internal_compiler_compile(const ScInternalCompilerBase *compiler, const char **shader)
{
    INTERNAL_RESULT(*shader = strdup(((spirv_cross::Compiler *)compiler)->compile().c_str());)
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
