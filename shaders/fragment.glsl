#version 330

in vec3 vColor;
out vec4 color;

void main() {
    color = vec4(vColor, 1.0);
}
