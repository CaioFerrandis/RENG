#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec4 color;
layout (location = 2) in vec2 tex_coords;
layout (location = 3) in vec3 normal;
layout (location = 4) in mat4 instance_model;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

out vec3 FragPos;
out vec3 transformedNormal;
out vec4 out_color;
out vec2 frag_texCoord;

void main()
{
    FragPos = vec3(instance_model * vec4(position, 1.));

    transformedNormal = mat3(transpose(inverse(instance_model))) * normal;

    gl_Position = projection * view * instance_model * vec4(position, 1.);
    
    out_color = color;
    frag_texCoord = tex_coords;
}
