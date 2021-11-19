#version 110


uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform vec3 lightPos;
uniform vec3 lightColor;
uniform vec3 objectColor;

varying vec3 vColor;
varying vec3 vNormal;
varying vec3 fragPos;
void main() {
    vec3 norm = normalize(vNormal);
    vec3 lightDir = normalize(lightPos - fragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;
    float strength = 0.1;
    vec3 ambient = strength * lightColor;
    vec3 result = (ambient + diffuse) * objectColor;

    gl_FragColor = vec4(result, 1.0);
}