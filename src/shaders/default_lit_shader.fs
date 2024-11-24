#version 330 core

out vec4 Result;

in vec3 FragPos;
in vec3 transformedNormal;
in vec2 frag_texCoord;
in vec4 out_color;

uniform vec3 lightColor[32];
uniform vec3 lightPos[32];
uniform sampler2D textureSampler;
uniform vec3 viewPos;

void main()
{
    vec4 texColor = texture(textureSampler, frag_texCoord);
    if(texColor.a * out_color.a < 0.1)
        discard;
    
    vec3 ambient = vec3(0.);
    vec3 diffuse = vec3(0.);
    vec3 specular = vec3(0.);

    for (int i = 0; i < 32; i++){
        // Ambient
        float ambientStrength = 0.1;
        ambient += lightColor[i] * ambientStrength;
        
        // Diffuse
        vec3 norm = normalize(transformedNormal);
        vec3 lightDir = normalize(lightPos[i] - FragPos);
        float diff = abs(dot(norm, lightDir));
        diffuse += lightColor[i] * diff;
        
        // Specular
        vec3 viewDir    = normalize(viewPos - FragPos);
        vec3 halfwayDir = normalize(lightDir + viewDir);

        float shininess = 0.3;
        float spec = pow(max(dot(norm, halfwayDir), 0.0), shininess);
        vec3 specular = lightColor[i] * spec;
    }

    // Combine all lighting effects
    Result = vec4(ambient + diffuse + specular, 1.0) * texColor * out_color;
}
