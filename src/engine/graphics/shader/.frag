#version 330 core

in vec3 FragPos;
in vec3 Normal;

out vec4 Color;

void main() {
    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * vec3(1.0, 1.0, 1.0);

    vec3 lightPos = vec3(0.0, 10.0, 0.0);

    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);

    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * vec3(0.6, 0.6, 0.6);

    vec3 result = (ambient + diffuse) * vec3(0.75, 0.0, 0.75);
    Color = vec4(result, 1.0);
}