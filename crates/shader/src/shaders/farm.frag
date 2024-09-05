#version 450

layout (location = 0) in float frag_material;
layout (location = 0) out vec4 outColor;

layout (binding = 1) uniform sampler2D palette_sampler;

void main() {
    outColor = texture(palette_sampler, vec2(frag_material, 0.5));
}