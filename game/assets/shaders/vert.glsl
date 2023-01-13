#version 330 core

precision mediump float;

in layout(location = 0) vec3 positions;
in layout(location = 1) vec3 normals;
in layout(location = 2) vec2 texCoords;

out vec3 fragPositions;
out vec3 fragNormals;
out vec2 fragTexCoords;

uniform mat4 model;
//uniform mat4 modelInvTrans;
uniform mat4 view;
uniform mat4 projection;

void main()
{ 
	fragPositions = vec3(model * vec4(positions, 1.0));
	fragNormals = normals;//normalize(mat3(modelInvTrans) * normals);
	fragTexCoords = texCoords;

	gl_Position = projection * view * vec4(fragPositions, 1.0);
}