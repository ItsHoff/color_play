#version 330

in vec2 v_tex_coords;

out vec4 color;

uniform sampler2D image;
uniform vec3 scale;

void main() {
    color = texture(image, v_tex_coords);
    for (int i = 0; i < 3; i++) {
        color[i] *= scale[i];
    }
}
