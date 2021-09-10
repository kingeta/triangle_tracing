#version 450 core

uniform ivec4 viewport;


out vec4 Color;

void main()
{
    Color = vec4(vec3(1., 0., 0.), 1.);
}
