#version 410

in float id;
in vec3 offset;

void main() {
    gl_Position = vec4(vec3(offset.xy * 2 - 1, 0.0), 1.0);
}
