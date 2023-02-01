__kernel void render(write_only image2d_t out)
{
    const size_t x = get_global_id(0);
	const size_t y = get_global_id(1);
    const size_t width = get_global_size(0);
	const size_t height = get_global_size(1);
    
    write_imagef(out, (int2)(x, y), (float4)(x / width, y / height, 1.0, 1.0));
}

// __kernel void render()
// {
//     const size_t x = get_global_id(0);
// 	const size_t y = get_global_id(1);
//     const size_t width = get_global_size(0);
// 	const size_t height = get_global_size(1);
// }