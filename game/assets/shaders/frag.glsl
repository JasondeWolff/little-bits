#version 330 core

precision mediump float;

in vec3 fragPositions;
in vec3 fragNormals;
in vec2 fragTexCoords;

out mediump vec4 FragColor;

void main()
{
	FragColor = vec4(fragNormals * 2.0 - 1.0, 1.0);
}