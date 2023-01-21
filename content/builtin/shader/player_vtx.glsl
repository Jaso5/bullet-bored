#version 330

in vec2 pos;

uniform vec4 color;

uniform mat4 model_matrix;
uniform mat4 view_matrix;

const int const_buzz[256] = int[](186, 139, 54, 199, 28, 107, 173, 232, 33, 224, 249, 81, 229, 203, 41, 202, 147, 209, 184, 148, 53, 155, 186, 255, 222, 38, 201, 33, 110, 128, 129, 248, 77, 77, 114, 15, 253, 206, 215, 87, 205, 91, 56, 135, 183, 68, 144, 175, 163, 25, 202, 225, 224, 72, 72, 75, 174, 232, 45, 120, 239, 89, 206, 200, 205, 244, 18, 95, 148, 147, 215, 158, 150, 86, 205, 236, 239, 42, 173, 174, 242, 127, 242, 103, 196, 73, 142, 77, 81, 217, 116, 9, 156, 156, 210, 65, 247, 224, 171, 148, 210, 102, 173, 59, 215, 51, 88, 103, 100, 183, 100, 81, 90, 65, 235, 73, 148, 181, 174, 215, 64, 144, 10, 74, 225, 42, 229, 153, 195, 86, 95, 37, 240, 26, 198, 127, 107, 256, 123, 50, 205, 184, 210, 97, 211, 187, 104, 199, 57, 124, 220, 127, 133, 11, 44, 55, 92, 15, 86, 194, 9, 152, 28, 181, 68, 65, 200, 54, 109, 30, 138, 38, 160, 67, 186, 142, 39, 2, 93, 63, 249, 170, 14, 108, 106, 67, 43, 222, 204, 242, 63, 175, 129, 6, 242, 165, 137, 15, 246, 152, 97, 256, 48, 197, 197, 74, 215, 105, 239, 91, 94, 220, 230, 205, 198, 186, 29, 37, 183, 33, 191, 222, 241, 112, 231, 113, 134, 55, 218, 186, 63, 230, 103, 237, 39, 62, 126, 94, 32, 200, 112, 176, 32, 202, 52, 71, 140, 5, 119, 157, 183, 218, 105, 189, 140, 192);
uniform float buzz_mag;
uniform int buzz_r;

out vec4 v_color;

float buzz(int offset, float mag) {
  return float(const_buzz[(gl_VertexID + buzz_r + offset) % 256]) * mag;
}

vec2 buzz2(float mag) {
  return vec2(buzz(0, mag), buzz(10, mag));
}

void main() {
  vec2 buzz = buzz2(buzz_mag);
  
  vec4 viewport_pos = view_matrix * model_matrix * vec4(pos + buzz, 0, 1);
  
  // Calculate depth of vtx from view space
  //float depth = viewport_pos.z;
  
  // Reduce buzzing on objects based on depth, so far away objects buzz less
  //vec2 buzz = buzz2(buzz_mag / (depth / 4));
  
  gl_Position = viewport_pos; // + vec4(buzz,0,0);
  
  v_color = color;
}