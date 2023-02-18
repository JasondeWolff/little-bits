#pragma OPENCL EXTENSION cl_intel_printf : enable

//#define RELU
//#define CLAMP_DELTAS
//#define USE_BIASES

//#define DEBUG_MODE
//#define PROGRESS_COUNTER

#include "rand.cl"
#include "common.cl"
#include "multi_hash_grid.cl"
#include "nn.cl"

__kernel void render(write_only image2d_t out,
    read_only image2d_t position_target,
    read_only image2d_t base_color_target,
    read_only image2d_t normal_target,
    read_only image2d_t mro_target,
    read_only image2d_t emission_target,
    __global Camera* camera,
    __global NeuralNetwork* nn,
    __global float* in_weights,
    __global float* out_weights,
    __local float* cache, int cacheSize,
    __global MutliHashGridMeta* mhgMeta,
    __global float* in_mhgElems,
    __global float* out_mhgElems,
    __global AABB* aabb,
    __global float* loss,
    __global float* errors)
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

    float learningRate = 0.3f;

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
        ray.direction = normalize(llc + uv.x * horizontal + uv.y * vertical - E);
        ray.invDirection = (float3)(1.0 / ray.direction.x, 1.0 / ray.direction.y, 1.0 / ray.direction.z);
    }

    float tAABB = RayAABBIntersection(&ray, aabb);
    if (tAABB < 0.0f)
    {
        write_imagef(out, (int2)(x, y), (float4)(0.0, 0.0, 0.0, 1.0));
        return;
    }
    // ray.origin += ray.direction * tAABB;

    // Travel distance along ray (use ray marching)
    float t = 0.01f;
    {
        t = read_imagef(position_target, (int2)(x, y)).w;
        if (t < 0.01f)
        {
            write_imagef(out, (int2)(x, y), (float4)(1.0, 0.0, 0.0, 1.0));
            return;
        }

        // COORDINATES DONT MATCH UP WITH RASTERIZER!!!
        // float3 pc = read_imagef(position_target, (int2)(x, y)).xyz;
        // if (length(pc) > 0.01f)
        // {
        //     float3 p = ray.origin + ray.direction * t;
        //     printf("True Hit: %f %f %f\nRay Hit: %f %f %f\n\n", pc.x, pc.y, pc.z, p.x, p.y, p.z);
        // }
    }

    if (PointAABBIntersection(ray.origin + ray.direction * t, aabb))
    {
        // Set neural network inputs
        {
            for (int l = 0; l < mhgMeta->resolutionLayers; l++)
            {
                float3 offset = -aabb->low;
                float sampleValue = GetGridSampleValue(mhgMeta, in_mhgElems, l, 0, offset + (ray.origin + ray.direction * t), &oc);
                cache[InputNeuron(nn, l, &oc)] = sampleValue;
            }

            // Give angle to learn mipmaps
            cache[InputNeuron(nn, mhgMeta->resolutionLayers, &oc)] = fabs(ray.direction.y) + fabs(ray.direction.x);
        }

        Forward(&oc, nn, in_weights, cache);
        float3 color = (float3)(cache[OutputNeuron(nn, 0, &oc)], cache[OutputNeuron(nn, 1, &oc)], cache[OutputNeuron(nn, 2, &oc)]);

        float4 target = read_imagef(base_color_target, (int2)(x, y));

        // Calculate errors
        {
            float error = target.x - cache[OutputNeuron(nn, 0, &oc)];
            float sign = error > 0.0 ? 1.0 : -1.0;
            cache[OutputNeuron(nn, 0, &oc)] = error * error * -sign;

            error = target.y - cache[OutputNeuron(nn, 1, &oc)];
            sign = error > 0.0 ? 1.0 : -1.0;
            cache[OutputNeuron(nn, 1, &oc)] = error * error * -sign;

            error = target.z - cache[OutputNeuron(nn, 2, &oc)];
            sign = error > 0.0 ? 1.0 : -1.0;
            cache[OutputNeuron(nn, 2, &oc)] = error * error * -sign;

            AtomicAddFloat(&errors[0], -error * unit);
        }

        // Store loss
        {
            float localLoss = cache[OutputNeuron(nn, 0, &oc)] * cache[OutputNeuron(nn, 0, &oc)] + cache[OutputNeuron(nn, 1, &oc)] * cache[OutputNeuron(nn, 1, &oc)] + cache[OutputNeuron(nn, 2, &oc)] * cache[OutputNeuron(nn, 2, &oc)];
            AtomicAddFloat(&loss[0], localLoss);
        }

        Backpropagate(&oc, nn, in_weights, out_weights, cache, learningRate, unit);

        // Backpropagate mhg
        for (int l = 0; l < mhgMeta->resolutionLayers; l++)
        {
            float delta = learningRate * cache[InputNeuronDelta(nn, l, &oc)] * width * height;
            //delta = clamp(delta, -0.05f, 0.05f);

            float3 pos = -aabb->low + (ray.origin + ray.direction * t);
            AtomicAddGridSampleValue(mhgMeta, out_mhgElems, l, 0, pos, delta, &oc);
        }

        write_imagef(out, (int2)(x, y), (float4)(color, 1.0));
        //write_imagef(out, (int2)(x, y), (float4)(target.xyz, 1.0));
    }
    else
    {
        write_imagef(out, (int2)(x, y), (float4)(1.0, 0.0, 0.0, 1.0));
    }
}

__kernel void train(read_only image2d_t position_target,
    __global Camera* camera,
    __global NeuralNetwork* nn,
    __global float* in_weights,
    __global float* out_weights,
    __local float* cache, int cacheSize,
    __global MutliHashGridMeta* mhgMeta,
    __global float* in_mhgElems,
    __global float* out_mhgElems,
    __global AABB* aabb,
    __global float* errors,
    float time)
{
    return;
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

#ifdef PROGRESS_COUNTER
    if (x == 0)
    {
        printf("out of %i\n", width);
    }
#endif

    float learningRate = 0.3f;

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
        ray.direction = normalize(llc + uv.x * horizontal + uv.y * vertical - E);
        ray.invDirection = (float3)(1.0 / ray.direction.x, 1.0 / ray.direction.y, 1.0 / ray.direction.z);
    }

    float tAABB = RayAABBIntersection(&ray, aabb);
    if (tAABB < 0.0f)
    {
        return;
    }
    // ray.origin += ray.direction * tAABB;

    // Travel distance along ray (use ray marching)
    float t = 0.01f;
    {
        t = read_imagef(position_target, (int2)(x, y)).w;
        if (t < 0.01f)
        {
            return;
        }
    }

    if (PointAABBIntersection(ray.origin + ray.direction * t, aabb))
    {
        // Set neural network inputs
        {
            for (int l = 0; l < mhgMeta->resolutionLayers; l++)
            {
                float3 offset = -aabb->low;
                float sampleValue = GetGridSampleValue(mhgMeta, in_mhgElems, l, 0, offset + (ray.origin + ray.direction * t), &oc);
                cache[InputNeuron(nn, l, &oc)] = sampleValue;

                // if (oc && time > 5.0f)
                // {
                //     printf("s: %f\n", sampleValue);
                //     oc = false;
                // }
            }

            // Give angle to learn mipmaps
            cache[InputNeuron(nn, mhgMeta->resolutionLayers, &oc)] = fabs(ray.direction.y) + fabs(ray.direction.x);
        }

        Forward(&oc, nn, in_weights, cache);
        float3 color = (float3)(cache[OutputNeuron(nn, 0, &oc)], cache[OutputNeuron(nn, 0, &oc)], cache[OutputNeuron(nn, 0, &oc)]);

        // Set errors
        {
            cache[OutputNeuron(nn, 0, &oc)] = errors[0];
        }

        Backpropagate(&oc, nn, in_weights, out_weights, cache, learningRate, unit);

        // Backpropagate mhg
        for (int l = 0; l < mhgMeta->resolutionLayers; l++)
        {
            float delta = learningRate * cache[InputNeuronDelta(nn, l, &oc)] * 10000.0;
            //delta = clamp(delta, -5000.0f, 5000.0f);

            float3 pos = -aabb->low + (ray.origin + ray.direction * t);
            AtomicAddGridSampleValue(mhgMeta, out_mhgElems, l, 0, pos, delta, &oc);
        }
    }
}