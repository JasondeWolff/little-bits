#include "nn.cl"
#include "common.cl"

__kernel void render(write_only image2d_t out,
    read_only image2d_t base_color_target,
    read_only image2d_t normal_target,
    read_only image2d_t mro_target,
    read_only image2d_t emission_target)
{
    const size_t x = get_global_id(0);
	const size_t y = get_global_id(1);
    const size_t width = get_global_size(0);
	const size_t height = get_global_size(1);

    write_imagef(out, (int2)(x, y), (float4)(x / (float)(width), y / (float)(height), 1.0));
}