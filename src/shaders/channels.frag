#version 330

in vec2 v_tex_coords;

out vec4 color;

uniform sampler2D r;
uniform sampler2D g;
uniform sampler2D b;

void main() {
    float r = texture(r, v_tex_coords).r;
    float g = texture(g, v_tex_coords).g;
    float b = texture(b, v_tex_coords).b;
    color = vec4(r, g, b, 1.0);
}
