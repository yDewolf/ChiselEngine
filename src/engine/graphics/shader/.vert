#version 330 core

layout (location = 0) in uint Entity_id;
layout (location = 1) in vec3 Position;
layout (location = 2) in vec3 aNormal;

uniform vec2 u_resolution;

uniform mat4 u_model_matrix[3];
uniform mat4 u_view_matrix;
uniform mat4 u_projection_matrix;

out vec3 FragPos;
out vec3 Normal;

void main() {
    vec4 uv = u_projection_matrix * u_view_matrix * u_model_matrix[Entity_id] * vec4(Position, 1.0);

    if (u_resolution.x > u_resolution.y) {
        uv.x *= u_resolution.y / u_resolution.x;
    } else {
        uv.y *= u_resolution.x / u_resolution.y;
    }

    gl_Position = uv;

    FragPos = Position;
    Normal = aNormal;
}