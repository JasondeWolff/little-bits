#pragma OPENCL EXTENSION cl_intel_printf : enable

#define DEBUG_MODE

#include "common.cl"
#include "nn.cl"

__kernel void render(write_only image2d_t out,
    read_only image2d_t base_color_target,
    read_only image2d_t normal_target,
    read_only image2d_t mro_target,
    read_only image2d_t emission_target,
    __global Camera* camera,
    __global NeuralNetwork* nn,
    __global float* in_weights,
    __global float* out_weights,
    __local float* cache, int cacheSize,
    __global float* loss)
{
    // Zero cache
    for (int i = 0; i < cacheSize; i++)
    {
        cache[i] = 0.0;
    }

    // Get kernel info
    const size_t x = get_global_id(0);
	const size_t y = get_global_id(1);
    const size_t width = get_global_size(0);
	const size_t height = get_global_size(1);
    const float unit = 1.0f / (width * height);

    float learningRate = 1.0f;

    // Allows a single printf per kernel
    bool oc = true;

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

    // Set neural network inputs
    {
        cache[InputNeuron(nn, 0, &oc)] = ray.origin.x;
        cache[InputNeuron(nn, 1, &oc)] = ray.origin.y;
        cache[InputNeuron(nn, 2, &oc)] = ray.origin.z;
        cache[InputNeuron(nn, 3, &oc)] = ray.direction.x;
        cache[InputNeuron(nn, 4, &oc)] = ray.direction.y;
    }

    Forward(&oc, nn, in_weights, cache);
    float3 color = (float3)(cache[OutputNeuron(nn, 0, &oc)], cache[OutputNeuron(nn, 1, &oc)], cache[OutputNeuron(nn, 2, &oc)]);
    // if (cache[OutputNeuron(nn, 3)] < 0.5)
    //     color = (float3)0;

    // Calculate errors
    {
        float4 target = read_imagef(base_color_target, (int2)(x, y));
        cache[OutputNeuron(nn, 0, &oc)] = cache[OutputNeuron(nn, 0, &oc)] - target.r;
        cache[OutputNeuron(nn, 1, &oc)] = cache[OutputNeuron(nn, 1, &oc)] - target.g;
        cache[OutputNeuron(nn, 2, &oc)] = cache[OutputNeuron(nn, 2, &oc)] - target.b;
        //cache[OutputNeuron(nn, 3)] = cache[OutputNeuron(nn, 3)] - target.a;
    }

    // Store loss
    {
        float localLoss = cache[OutputNeuron(nn, 0, &oc)] * cache[OutputNeuron(nn, 0, &oc)] + cache[OutputNeuron(nn, 1, &oc)] * cache[OutputNeuron(nn, 1, &oc)] + cache[OutputNeuron(nn, 2, &oc)] * cache[OutputNeuron(nn, 2, &oc)];
        AtomicAddFloat(&loss[0], localLoss);
        //loss[0] = localLoss;
    }

    Backpropagate(&oc, nn, in_weights, out_weights, cache, learningRate * unit);

    color = read_imagef(base_color_target, (int2)(x, y)).xyz;

    write_imagef(out, (int2)(x, y), (float4)(color, 1.0));
}