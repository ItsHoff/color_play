#version 330

in vec2 v_tex_coords;

out vec4 color;

uniform sampler2D image;
uniform vec2 shift;

void main() {
    color = texture(image, v_tex_coords + shift);
}
