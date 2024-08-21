#version 330 core
layout (location = 0) in vec3 aPos;

uniform vec2 scale;

void main()
{
    gl_Position = vec4(aPos.x * scale.x, aPos.y * scale.y, aPos.z, 1.0);
}
