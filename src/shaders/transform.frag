#version 330

in vec2 v_tex_coords;

out vec4 color;

uniform sampler2D image;
uniform mat4 transform;

void main() {
    color = transform * texture(image, v_tex_coords);
}
