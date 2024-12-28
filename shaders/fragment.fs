#version 330 core

in vec2 fragTexCoord;

out vec4 fragColor;

uniform sampler2D texture0;
uniform vec2 mousePosition;
uniform float time;

float random(vec2 st) {
    return fract(sin(dot(st.xy, vec2(12.9898,78.233))) * 43758.5453123);
}

void main() {
    vec4 texColor = texture(texture0, fragTexCoord);

    float dist = distance(fragTexCoord, mousePosition);

    float radius = 0.1;

    float strength = smoothstep(radius, 0.0, dist);

    float offsetScale = 0.01 * strength;
    vec2 redOffset = vec2(
        random(vec2(time * 0.1, 0.0)) * 2.0 - 1.0,
        random(vec2(0.0, time * 1.0)) * 2.0 - 1.0
    ) * offsetScale;
    vec2 blueOffset = vec2(
        random(vec2(time * 0.1 + 42.0, 0.0)) * 2.0 - 1.0,
        random(vec2(0.0, time * 0.1 + 42.0)) * 2.0 - 1.0
    ) * offsetScale;

    vec4 redChannel = texture(texture0, fragTexCoord + redOffset);
    vec4 greenChannel = texture(texture0, fragTexCoord);
    vec4 blueChannel = texture(texture0, fragTexCoord + blueOffset);

    vec4 finalColor = vec4(
        mix(texture(texture0, fragTexCoord).r, redChannel.r, strength),
        mix(texture(texture0, fragTexCoord).g, greenChannel.g, strength),
        mix(texture(texture0, fragTexCoord).b, blueChannel.b, strength),
        texture(texture0, fragTexCoord).a
    );

    fragColor = vec4(finalColor.b, finalColor.g, finalColor.r, finalColor.a);
}
