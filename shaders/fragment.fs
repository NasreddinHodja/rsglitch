#version 330 core

in vec2 fragTexCoord;
uniform sampler2D texture0;

out vec4 fragColor;

void main() {
    // Apply a simple color filter (e.g., invert the colors)
    vec4 texColor = texture(texture0, fragTexCoord);
    fragColor = vec4(texColor.b, texColor.g, texColor.r, texColor.a);
}
