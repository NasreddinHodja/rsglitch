#version 330 core

in vec2 fragTexCoord;

out vec4 fragColor;

uniform sampler2D texture0;
uniform vec2 mousePosition;

void main() {
    vec4 texColor = texture(texture0, fragTexCoord);
    vec4 convertedColor = vec4(texColor.b, texColor.g, texColor.r, texColor.a);

    float dist = distance(fragTexCoord, mousePosition);
    float radius = 0.01;

    if (dist < radius) {
        fragColor = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        fragColor = convertedColor;
    }
}
