#version 330

in vec2 tex_coords;

out vec2 v_tex_coords;

void main() {
    v_tex_coords = tex_coords;
    vec2 pos = 2.0 * tex_coords - 1.0;
    gl_Position = vec4(pos, 0.0, 1.0);
}
