#version 110

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 lightPos;
uniform vec3 lightColor;
uniform vec3 objectColor;

attribute vec3 position;
attribute vec3 color;
attribute vec3 normal;

varying vec3 vColor;
varying vec3 vNormal;
varying vec3 fragPos;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.0);
    fragPos = vec3(model * vec4(position, 1.0));
    vColor = color;
    vNormal = normal;

}