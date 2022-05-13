#version 300 es

layout (location = 0) in vec3 a_position;
layout (location = 1) in vec3 a_color;
layout (location = 2) in mat4 a_modelview;

uniform mat4 u_proj;
uniform mat4 u_modelview;
uniform mat4 u_view;

out vec4 vColor;

mat4 translation(vec3 delta)
{
    return mat4(
        vec4(1.0, 0.0, 0.0, 0.0),
        vec4(0.0, 1.0, 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(delta, 1.0));
}

void main() {
  vColor = vec4(vec3(a_color), 1.0);

  gl_Position = u_proj * u_view * a_modelview * vec4(a_position, 1.0);
  // gl_Position = u_proj * u_modelview * vec4(a_position, 1.0);
}
