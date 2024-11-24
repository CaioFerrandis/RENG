#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec4 color;
layout (location = 2) in vec2 tex_coords;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

out vec3 FragPos;
out vec4 out_color;
out vec2 frag_texCoord;

void main()
{
    // Transform the position to world space
    FragPos = vec3(model * vec4(position, 1.));
    
    // Final position for rendering
    gl_Position = projection * view * model * vec4(position, 1.);
    
    // Pass color and texture coordinates
    out_color = color;
    frag_texCoord = tex_coords;
}
