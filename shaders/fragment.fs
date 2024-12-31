#version 330 core

in vec2 fragTexCoord;

out vec4 fragColor;

uniform sampler2D texture0;
uniform vec2 mousePosition;
uniform vec2 mouseKeys;
uniform float time;

float random(vec2 st) {
    return fract(sin(dot(st.xy, vec2(12.9898,78.233))) * 43758.5453123);
}

vec4 flip(vec4 color) {
    return vec4(color.b, color.g, color.r, color.a);
}

vec4 aberrationsAroundMouse() {
    float dist = distance(fragTexCoord, mousePosition);

    float radius = 0.3;

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
        redChannel.r,
        greenChannel.g, 
        blueChannel.b, 
        texture(texture0, fragTexCoord).a
    );

    return finalColor;
}

vec4 sharpen(vec4 color) {
    mat3 sharpenKernel = mat3(
        -2.5, -2.5, -2.5,
        -2.5, 20.9, -2.5,
        -2.5, -2.5, -2.5
    );


    vec2 texelSize = 1.0 / vec2(textureSize(texture0, 0));
    vec4 result = vec4(0.0);
    
    for (int x = -1; x <= 1; ++x) {
        for (int y = -1; y <= 1; ++y) {
            vec2 offset = vec2(x, y) * texelSize;
            vec4 neighborColor = texture(texture0, fragTexCoord + offset);
            vec4 flipped = flip(neighborColor);
            result += flipped * sharpenKernel[x + 1][y + 1];
        }
    }

    return result;
}

vec4 applyThreshold(vec4 color, float threshold) {
    float luminance = dot(color.rgb, vec3(0.299, 0.587, 0.114));

    if (luminance < threshold) {
        return vec4(0.0, 0.0, 0.0, color.a);
    }

    return color;
}

vec4 pixelize() {
    float pixelSize = 0.01;
    vec2 texCoord = floor(fragTexCoord / pixelSize) * pixelSize;
    return texture(texture0, texCoord);
}

vec4 flicker(vec4 texColor) {
    float flickerAmount = random(vec2(time * 0.1, 0.0)) * 0.10;
    vec2 offset = vec2(sin(time * 0.5) * flickerAmount, cos(time * 0.3) * flickerAmount);
    
    vec2 distortedTexCoord = fragTexCoord + offset;
    vec4 flickeredColor = texture(texture0, distortedTexCoord);
    
    return flickeredColor;
}

vec4 staticNoise(vec4 texColor) {
    float noise = random(vec2(fragTexCoord.x + time, fragTexCoord.y + time));
    float noiseIntensity = smoothstep(0.25, 0.01, noise);
    return texColor * (1.0 - noiseIntensity) + vec4(noiseIntensity);
}

vec4 waveDistortion(vec4 texColor) {
    float waveFactor = sin(time + fragTexCoord.y * 20.0) * 0.50;

    texColor.r += waveFactor * texColor.r;
    texColor.g += waveFactor * texColor.g;
    texColor.b += waveFactor * texColor.b;
    
    return texColor;
}

vec4 streak(vec4 texColor) {
    float streakFactor = sin(time * 0.5 + fragTexCoord.x * 10.0) * 0.9;  // Horizontal streaking

    texColor.r += streakFactor * texColor.r;
    texColor.g += streakFactor * texColor.g;
    texColor.b += streakFactor * texColor.b;

    return texColor;
}

vec4 horizontalSlice(vec4 texColor) {
    float sliceHeight = 0.05;
    int sliceIndex = int(fragTexCoord.y / sliceHeight);
    float shift = random(vec2(sliceIndex, time)) * 0.6 - 0.05;

    vec2 newCoord = fragTexCoord + vec2(0.0, shift);
    
    vec4 newTexColor = texture(texture0, newCoord);

    return newTexColor;
}

vec4 wavywavy(vec4 texColor) {
    float lineShift = sin(time + fragTexCoord.y * 50.0) * 0.02;
    vec2 newCoord = fragTexCoord + vec2(lineShift, 0.0);
    
    vec4 newTexColor = texture(texture0, newCoord);

    return newTexColor;
}

vec4 overexposure(vec4 texColor) {
    float randomBright = random(vec2(fragTexCoord.x + time, fragTexCoord.y + time));
    float intensity = smoothstep(0.9, 1.0, randomBright);
    return texColor * (1.0 - intensity) + vec4(intensity);
}

void main() {
    vec4 texColor = texture(texture0, fragTexCoord);

    // vec4 finalColor = aberrationsAroundMouse();
    // vec4 finalColor = pixelize();
    // vec4 finalColor = flicker(texColor);
    // vec4 finalColor = staticNoise(texColor);
    // vec4 finalColor = waveDistortion(texColor);
    // vec4 finalColor = streak(texColor);
    // vec4 finalColor = horizontalSlice(texColor);
    vec4 finalColor = wavywavy(texColor);
    finalColor = flip(finalColor);

    if (mouseKeys[0] == 1) {
        finalColor = sharpen(finalColor);
    }
    if (mouseKeys[1] == 1) {
        finalColor = applyThreshold(finalColor, 0.50);
    }

    fragColor = finalColor;
}
