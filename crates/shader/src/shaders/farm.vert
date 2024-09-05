#version 450

layout(binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

layout (location = 0) in vec4 vertex;
layout (location = 0) out float frag_material;

void main() {
    vec3 vertex_position = vertex.xyz;
    frag_material = vertex.w;
    gl_Position = ubo.proj * ubo.view * ubo.model * vec4(vertex_position, 1.0);
}