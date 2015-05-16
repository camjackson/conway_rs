#version 330

uniform mat4 transform;

in vec2 position;
in vec3 color;
in vec2 world_position;
in float scale;

out vec3 vColor;

void main() {
    gl_Position = transform * vec4(world_position + scale * position, 0.0, 1.0);
    vColor = color;
}
