#version 450

void main(void) {
    vec2 uv = vec2((gl_VertexIndex << 1) & 2, gl_VertexIndex & 2);
    gl_Position = vec4(vec2(uv.x * 2.0 - 1.0, -uv.y * 2.0 + 1.0), 0.0, 1.0);
}
