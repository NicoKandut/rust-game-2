#version 450

layout (set = 0, binding = 0, std430) restrict buffer WorldData {
    float[] nodes;
} world_data;

layout (local_size_x = 8, local_size_y = 8, local_size_z = 1) in;
void main() {

}