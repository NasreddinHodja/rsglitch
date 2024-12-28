#version 330 core

in vec2 fragTexCoord;
uniform sampler2D texture0;

out vec4 fragColor;

void main() {
    // Apply a simple color filter (e.g., invert the colors)
    vec4 texColor = texture(texture0, fragTexCoord);
    // fragColor = vec4(1.0 - texColor.rgb, texColor.a); // Invert colors
    // fragColor = vec4(1.0, 1.0, 0.0, 1.0);
    fragColor = texture(texture0, fragTexCoord);
}
