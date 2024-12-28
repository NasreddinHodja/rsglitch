#version 330 core

in vec2 fragTexCoord;

out vec4 fragColor;

uniform sampler2D texture0;
// uniform int keyboardKeys[256];
uniform vec2 mousePosition;
// uniform int mouseKeys[2];

void main() {
    vec4 texColor = texture(texture0, fragTexCoord);
    vec4 convertedColor = vec4(texColor.b, texColor.g, texColor.r, texColor.a);

    vec4 tint = vec4(mousePosition.x / 1920.0, mousePosition.y / 1080.0, 0.5, 1.0);
    fragColor = convertedColor * tint;
}
