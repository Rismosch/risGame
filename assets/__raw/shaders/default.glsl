#glsl_version 460

#define ONE 1.0
#define TWO 2.0

#include test_a
#include test_b

#layout vertex
vec3 invert_color(vec3 c) {
    return (1 - c) * 2;
}

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 color;

layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 view;
    mat4 proj;
    mat4 proj_view;
} ubo;

#layout io vertex fragment
layout(location = 0) OUT_IN vec3 f_color;

#layout fragment
layout(location = 0) out vec4 out_color;

#entry vertex
void main() {
    gl_Position = ubo.proj_view * vec4(position, 1.0);

    f_color = invert_color(color);
}

#entry fragment
void main() {
    out_color = vec4(f_color, 1.0);
}
