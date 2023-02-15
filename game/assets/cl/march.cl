#include "common.cl"
#include "nn.cl"

float3 Trace(Ray* ray,
     __global NeuralNetwork* nn,
    __global float* weights,
    __local float* cache)
{
    for (int i = 0; i < 1024; i++)
    {
        
    }
}

__kernel void render(write_only image2d_t out,
    __global Camera* camera,
    __global NeuralNetwork* nn,
    __global float* weights,
    __local float* cache, int cacheSize)
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

        float2 uv = (float2)(x, y) / (float2)(width, height);
        cache[InputNeuron(nn, 0, &oc)] = uv.x;
        cache[InputNeuron(nn, 1, &oc)] = uv.y;
    }
}