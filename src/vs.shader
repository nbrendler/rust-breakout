// those are our vertex attributes
in vec2 position;
in vec2 texture_coords;

uniform mat4 model;
uniform mat4 world;

out vec2 TexCoords;

void main() {
  TexCoords = texture_coords;
  gl_Position =  world * model * vec4(position, 0., 1.);
}
