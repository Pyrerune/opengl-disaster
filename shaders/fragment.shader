#version 110


uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 lightPos;
uniform vec3 lightColor;
uniform vec3 objectColor;
uniform sampler2D tex;


varying vec2 vTexCoords;
varying vec3 vNormal;
varying vec3 fragPos;
void main() {
    vec3 norm = normalize(vNormal);
    vec3 lightDir = normalize(lightPos - fragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec4 diffuse = diff * vec4(lightColor, 1.0);
    float strength = 0.1;
    vec4 ambient = vec4(strength * lightColor, 1.0);
    vec4 color = texture2D(tex, vTexCoords);
    vec4 result = (ambient + diffuse) * color;

    gl_FragColor = result;
}