#version 330 core

out vec4 Result;

in vec3 FragPos;
in vec2 frag_texCoord;
in vec4 out_color;

uniform sampler2D textureSampler;
uniform vec3 viewPos;

void main()
{
    vec4 texColor = texture(textureSampler, frag_texCoord);
    if(texColor.a * out_color.a < 0.1)
        discard;

    Result = texColor * out_color;
}
