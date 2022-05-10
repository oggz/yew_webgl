precision mediump float;

attribute vec3 a_position;
attribute vec3 a_color;
attribute vec3 a_translation;

uniform mat4 u_proj;
uniform mat4 u_modelview;

varying vec4 vColor;

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

    vec3 position = a_position + a_translation;
  gl_Position = u_proj * u_modelview * vec4(position, 1.0);

  // gl_Position = u_proj * u_modelview * vec4(a_position, 1.0);
}
