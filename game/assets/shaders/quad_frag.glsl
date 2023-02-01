#version 330 core

precision mediump float;

in vec2 tex_coord;

uniform sampler2D tex;

out mediump vec4 FragColor;

void main()
{
    FragColor = vec4(texture(tex, tex_coord).rgb, 1.0);
}