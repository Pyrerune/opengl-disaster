#version 140

in vec3 position;
in vec2 tex_coords;
out vec2 v_tex_coords;
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position =  projection * view * model * vec4(position, 1.0);
    v_tex_coords = tex_coords;
}
