#version 300 es

precision mediump float;

uniform float u_time;

in vec4 vColor;
out vec4 color;

void main() {
  // float r = sin(u_time * 0.0003);
  // float g = sin(u_time * 0.0005);
  // float b = sin(u_time * 0.0007);

  float r = vColor.x * sin(u_time * 0.0019) + 0.50;
  float g = vColor.y * sin(u_time * 0.0015) + 0.50;
  float b = vColor.z * sin(u_time * 0.0017) + 0.50;
  
  color = vec4(r, g, b, 1.0);
}
