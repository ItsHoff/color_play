#version 330

in vec2 v_tex_coords;

out vec4 color;

uniform sampler2D image;
uniform uvec3 permutation;

void main() {
    vec4 tex_color = texture(image, v_tex_coords);
    for (int i = 0; i < 3; i++) {
        color[i] = tex_color[permutation[i]];
    }
    color.a = 1.0;
}
