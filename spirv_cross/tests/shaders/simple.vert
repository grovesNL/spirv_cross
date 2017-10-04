#version 310 es

layout(std140) uniform uniform_buffer_object
{
    uniform mat4 u_model_view_projection;
};

layout(location = 0) in vec4 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 0) out vec3 v_normal;

void main()
{
    v_normal = a_normal;
    gl_Position = u_model_view_projection * a_position;
}
