#version 330 core

precision mediump float;

in layout(location = 0) vec3 positions;
in layout(location = 1) vec3 normals;
in layout(location = 2) vec2 texCoords;
in layout(location = 3) vec4 tangents;

out vec3 fragPosition;
out vec2 fragTexCoord;
out mat3 TBN;

#define EPSILON 0.0001

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
	fragPosition = vec3(model * vec4(positions, 1.0));
	fragTexCoord = texCoords;

	vec3 N = normalize((model * vec4(normals, 0.0)) + EPSILON).xyz;
	vec3 T = normalize((model * vec4(tangents.xyz, 0.0)) + EPSILON).xyz;
	vec3 B = normalize(cross(T, N) + EPSILON) * tangents.w;
	TBN = mat3(T, B, N);

	gl_Position = projection * view * vec4(fragPosition, 1.0);
}