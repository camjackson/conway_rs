#version 140

uniform mat4 view_transform;

in vec2 vertex_position;
in vec2 model_position;
in float model_scale;

in vec3 vertex_color;
out vec3 vColor;

void main() {
    gl_Position = view_transform * vec4(model_position + model_scale * vertex_position, 0.0, 1.0);
    vColor = vertex_color;
}
