#version 410

in float id;
in vec3 offset;
in vec3 position;

out vec3 color;

float rand(vec2 co) {
    return fract(sin(dot(co.xy, vec2(12.9898, 78.233))) * 43758.5453);
}

void main() {
    color = vec3(
        rand(offset.xy + offset.yz),
        rand(offset.yx + offset.zx),
        rand(offset.xz + offset.zy)
    );

    gl_Position = vec4(position + vec3(offset.xy * 2 - 1, 0.0), 1.0);
}
