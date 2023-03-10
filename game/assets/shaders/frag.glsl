#version 330 core

precision mediump float;

in vec3 fragPosition;
in vec2 fragTexCoord;
in mat3 TBN;

out mediump vec4 FragColor;

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

uniform vec3 viewPos;

const float PI = 3.14159265359;
// ----------------------------------------------------------------------------
float DistributionGGX(vec3 N, vec3 H, float roughness)
{
    float a = roughness*roughness;
    float a2 = a*a;
    float NdotH = max(dot(N, H), 0.0);
    float NdotH2 = NdotH*NdotH;

    float nom   = a2;
    float denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return nom / denom;
}
// ----------------------------------------------------------------------------
float GeometrySchlickGGX(float NdotV, float roughness)
{
    float r = (roughness + 1.0);
    float k = (r*r) / 8.0;

    float nom   = NdotV;
    float denom = NdotV * (1.0 - k) + k;

    return nom / denom;
}
// ----------------------------------------------------------------------------
float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness)
{
    float NdotV = max(dot(N, V), 0.0);
    float NdotL = max(dot(N, L), 0.0);
    float ggx2 = GeometrySchlickGGX(NdotV, roughness);
    float ggx1 = GeometrySchlickGGX(NdotL, roughness);

    return ggx1 * ggx2;
}
// ----------------------------------------------------------------------------
vec3 fresnelSchlick(float cosTheta, vec3 F0)
{
    return F0 + (1.0 - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}
// ----------------------------------------------------------------------------
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

    vec3 V = normalize(viewPos - fragPosition);
	float ao = 0.1;

    // calculate reflectance at normal incidence; if dia-electric (like plastic) use F0 
    // of 0.04 and if it's a metal, use the albedo color as F0 (metallic workflow)    
    vec3 F0 = vec3(0.04); 
    F0 = mix(F0, albedo, metallic);

    // reflectance equation
    vec3 Lo = vec3(0.0);
    for(int i = 0; i < 1; ++i) 
    {
        // calculate per-light radiance
        vec3 L = normalize(-vec3(0.1, -1.0, 0.0));
        vec3 H = normalize(V + L);
        vec3 radiance = vec3(1.0, 1.0, 1.0) * 1.1;

        // Cook-Torrance BRDF
        float NDF = DistributionGGX(N, H, roughness);   
        float G   = GeometrySmith(N, V, L, roughness);      
        vec3 F    = fresnelSchlick(clamp(dot(H, V), 0.0, 1.0), F0);
           
        vec3 numerator    = NDF * G * F; 
        float denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0) + 0.0001; // + 0.0001 to prevent divide by zero
        vec3 specular = numerator / denominator;
        
        // kS is equal to Fresnel
        vec3 kS = F;
        // for energy conservation, the diffuse and specular light can't
        // be above 1.0 (unless the surface emits light); to preserve this
        // relationship the diffuse component (kD) should equal 1.0 - kS.
        vec3 kD = vec3(1.0) - kS;
        // multiply kD by the inverse metalness such that only non-metals 
        // have diffuse lighting, or a linear blend if partly metal (pure metals
        // have no diffuse light).
        kD *= 1.0 - metallic;	  

        // scale light by NdotL
        float NdotL = max(dot(N, L), 0.0);        

        // add to outgoing radiance Lo
        Lo += (kD * albedo / PI + specular) * radiance * NdotL;  // note that we already multiplied the BRDF by the Fresnel (kS) so we won't multiply by kS again
    }   
    
    // ambient lighting (note that the next IBL tutorial will replace 
    // this ambient lighting with environment lighting).
    vec3 ambient = vec3(0.03) * albedo * ao;

    vec3 color = (ambient + Lo) * occlusion + emission;

    // HDR tonemapping
    color = color / (color + vec3(1.0));
    // gamma correct
    color = pow(color, vec3(1.0/2.2)); 

    FragColor = vec4(color, 1.0);
}