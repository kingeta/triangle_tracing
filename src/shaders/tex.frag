#version 430 core

in vec2 texcoord;

out vec4 colour;

uniform sampler2D tex;

void main() {
    //color = vec4(1.);
    //colour = vec4(vec3(texcoord.y), 1.);
    colour = texture(tex, texcoord);
}