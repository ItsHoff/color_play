#version 330

in vec2 v_tex_coords;

out vec4 color;

uniform vec3 permutation;
uniform sampler2D image;

void main() {
    color = texture(image, v_tex_coords);
}
