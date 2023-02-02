#include "nn.cl"
#include "common.cl"

__kernel void render(write_only image2d_t out,
    read_only image2d_t base_color_target,
    read_only image2d_t normal_target,
    read_only image2d_t mro_target,
    read_only image2d_t emission_target,
    __global Camera* camera)
{
    const size_t x = get_global_id(0);
	const size_t y = get_global_id(1);
    const size_t width = get_global_size(0);
	const size_t height = get_global_size(1);

    // Construct ray from camera
    Ray ray;
    {
        const float3 E = camera->position.xyz;
        const float3 llc = camera->lowerLeftCorner.xyz;
        const float3 horizontal = camera->horizontal.xyz;
        const float3 vertical = camera->vertical.xyz;

        float2 uv = (float2)(x, y) / (float2)(width, height);
        uv.y = 1.0f - uv.y;

        ray.origin = E;
        ray.direction = llc + uv.x * horizontal + uv.y * vertical - E;
    }

    float3 color = read_imagef(base_color_target, (int2)(x, y)).xyz;

    write_imagef(out, (int2)(x, y), (float4)(color, 1.0));
}