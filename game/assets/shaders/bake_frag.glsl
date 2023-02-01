#version 330 core

precision mediump float;

in vec3 fragPosition;
in vec2 fragTexCoord;
in mat3 TBN;

layout(location = 0) out mediump vec3 BaseColor;
layout(location = 1) out mediump vec3 Normal;
layout(location = 2) out mediump vec3 MRO;
layout(location = 3) out mediump vec3 Emission;

uniform struct Material {
    vec4 baseColorFactor;
    float normalScale;
    float metallicFactor;
    float roughnessFactor;
    float occlusionStrength;
    vec3 emissiveFactor;

    bool hasBaseColorMap;
    bool hasNormalMap;
    bool hasMetallicRoughnessMap;
    bool hasOcclusionMap;
    bool hasEmissiveMap;

    sampler2D baseColorMap;
    sampler2D normalMap;
    sampler2D metallicRoughnessMap;
    sampler2D occlusionMap;
    sampler2D emissiveMap;
} material;

void main()
{
    vec3 albedo = material.baseColorFactor.rgb;
    if (material.hasBaseColorMap)
    {
        albedo = albedo * texture(material.baseColorMap, fragTexCoord).rgb;
    }

    vec3 N;
    if (material.hasNormalMap)
    {
        vec3 normal = texture(material.normalMap, fragTexCoord).rgb;
        normal = normal * 2.0 - 1.0;
        N = normalize(mix(TBN * normal, TBN[2], 1.0 - material.normalScale));
    }
    else
    {
        N = TBN[2];
    }

    float metallic = material.metallicFactor;
    float roughness = material.roughnessFactor;
    if (material.hasMetallicRoughnessMap)
    {
        vec2 metallicRoughness = texture(material.metallicRoughnessMap, fragTexCoord).gb;
	    metallic = metallic * metallicRoughness.y;
	    roughness = roughness * metallicRoughness.x;
    }

    float occlusion = 1.0;
    if (material.hasOcclusionMap)
    {
        occlusion = mix(texture(material.occlusionMap, fragTexCoord).r, 1.0, 1.0 - material.occlusionStrength);
    }

	vec3 emission = vec3(0.0, 0.0, 0.0);
    if (material.hasEmissiveMap)
    {
        emission = texture(material.emissiveMap, fragTexCoord).rgb * material.emissiveFactor;
    }

    BaseColor = albedo;
    Normal = N;
    MRO = vec3(metallic, roughness, occlusion);
    Emission = emission;
}