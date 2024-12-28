#version 330

in vec3 vertexPosition;  // Position of the vertex
in vec2 vertexTexCoord;  // Texture coordinates of the vertex

out vec2 fragTexCoord;   // Pass texture coordinates to fragment shader

uniform mat4 mvp;        // Model-View-Projection matrix

void main() {
    fragTexCoord = vertexTexCoord;         // Pass texture coordinates to fragment shader
    gl_Position = mvp * vec4(vertexPosition, 1.0); // Transform vertex position
}
