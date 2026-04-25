#version 140

in vec2 position;

uniform mat4 transform;

out vec2 v_pos;

void main() {
  v_pos = position;
  gl_Position = transform * vec4(position, 0.0, 1.0);
}
