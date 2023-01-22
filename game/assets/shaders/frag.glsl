#version 330 core

precision mediump float;

in vec3 fragPositions;
in vec3 fragNormals;
in vec2 fragTexCoords;

out mediump vec4 FragColor;

uniform sampler2D baseColor;

void main()
{
	FragColor = vec4(texture(baseColor, fragTexCoords).rgb, 1.0);
}