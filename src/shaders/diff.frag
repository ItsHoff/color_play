#version 330

in vec2 v_tex_coords;

out vec4 color;

uniform sampler2D tex1;
uniform sampler2D tex2;
uniform bool use_abs;

void main() {
    vec4 c1 = texture(tex1, v_tex_coords);
    vec4 c2 = texture(tex2, v_tex_coords);
    vec4 diff = c1 - c2;
    if (use_abs) {
        color = abs(diff);
    } else {
        color = 0.5 * diff + 0.5;
    }
}
