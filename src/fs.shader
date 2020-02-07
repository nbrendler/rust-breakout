in vec2 TexCoords;

uniform sampler2D image;

out vec4 color;

void main() {
  color = texture(image, TexCoords);
}
