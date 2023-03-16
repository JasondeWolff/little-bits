#pragma OPENCL EXTENSION cl_intel_printf : enable

#define MOMENTUM
//#define ADA_GRAD
//#define RMSP
#define ADAM

#define MSE

//#define USE_BIASES

//#define DEBUG_MODE

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
    __global float* in_momentum,
    __global float* out_momentum,
    __local float* cache, int cacheSize,
    __global MutliHashGridMeta* mhgMeta,
    __global float* in_mhgElems,
    __global float* out_mhgElems,
    __global AABB* aabb,
    __global float* loss,
    __global float* errors)
{
    // Get kernel info
    const size_t x = get_global_id(0);
	const size_t y = get_global_id(1);
    const size_t width = get_global_size(0);
	const size_t height = get_global_size(1);
    const float unit = 1.0f / (width * height);

    float learningRate = 0.003f;
    float l2reg = 0.000001f;
    float beta1 = 0.9f;
    float beta2 = 0.999f;
    //double epsilon = 0.00000001f;
    double epsilon = 0.000000000000001f;

    // Allows a single printf per kernel
    bool oc = true;

    // Construct ray from camera
    Ray ray;
    {
        // const float3 E = camera->position.xyz;
        // const float3 llc = camera->lowerLeftCorner.xyz;
        // const float3 horizontal = camera->horizontal.xyz;
        // const float3 vertical = camera->vertical.xyz;

        // float2 uv = (float2)(x, y) / (float2)(width, height);

        // ray.origin = E;
        // ray.direction = normalize(llc + uv.x * horizontal + uv.y * vertical - E);
        // ray.invDirection = (float3)(1.0 / ray.direction.x, 1.0 / ray.direction.y, 1.0 / ray.direction.z);

        // Can cheat because the hit position is already known, otherwise use code above
        ray.origin = camera->position.xyz;
        ray.direction = normalize(read_imagef(position_target, (int2)(x, y)).xyz - ray.origin);
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

        float3 pc = read_imagef(position_target, (int2)(x, y)).xyz;
        if (length(pc) > 0.01f)
        {
            float3 p = ray.origin + ray.direction * t;
            if (distance(pc, p) > 0.05f)
            {
                printf("True Hit: %f %f %f\nRay Hit: %f %f %f\ndir: %f %f %f\norig: %f %f %f\nt: %f\n\n", pc.x, pc.y, pc.z, p.x, p.y, p.z, ray.direction.x, ray.direction.y, ray.direction.z, ray.origin.x, ray.origin.y, ray.origin.z, t);
            }
        }
    }

    if (PointAABBIntersection(ray.origin + ray.direction * t, aabb))
    {
        float3 pos = -aabb->low + (ray.origin + ray.direction * t);

        // Set neural network inputs
        {
            for (int l = 0; l < mhgMeta->resolutionLayers; l++)
            {
                for (int f = 0; f < mhgMeta->featuresPerEntry; f++)
                {
                    float sampleValue = GetGridSampleValue(mhgMeta, in_mhgElems, l, f, pos, &oc, 0);
                    cache[InputNeuron(nn, f + l * mhgMeta->featuresPerEntry, &oc)] = sampleValue;
                }
            }

            // Give angle to learn mipmaps
            //cache[InputNeuron(nn, mhgMeta->resolutionLayers, &oc)] = fabs(ray.direction.y) + fabs(ray.direction.x);
        }

        Forward(&oc, nn, in_weights, cache);
        float3 color = (float3)(cache[OutputNeuron(nn, 0, &oc)], cache[OutputNeuron(nn, 1, &oc)], cache[OutputNeuron(nn, 2, &oc)]);

        // Calculate errors
        float4 target = read_imagef(normal_target, (int2)(x, y));

        // if (target.x < 0.0f || target.x > 1.0f || isnan(target.x) == 1) {
        //     printf("x: %f\n", target.x);
        // } if (target.y < 0.0f || target.y > 1.0f || isnan(target.y) == 1) {
        //     printf("y: %f\n", target.y);
        // } if (target.z < 0.0f || target.z > 1.0f || isnan(target.z) == 1) {
        //     printf("z: %f\n", target.z);
        // }

        cache[TargetValue(nn, 0, &oc)] = target.x;
        cache[TargetValue(nn, 1, &oc)] = target.y;
        cache[TargetValue(nn, 2, &oc)] = target.z;

        Backpropagate(&oc, nn, in_weights, out_weights, in_momentum, out_momentum, beta1, beta2, epsilon, cache, learningRate, unit, l2reg, loss);

        // Backpropagate mhg
        for (int l = 0; l < mhgMeta->resolutionLayers; l++)
        {
            for (int f = 0; f < mhgMeta->featuresPerEntry; f++)
            {
                // int weightsSize = nn->inputCount * nn->hiddenCount + nn->hiddenCount * nn->hiddenCount * (nn->hiddenLayerCount - 1) + nn->hiddenCount * nn->outputCount;
                // int mhgSize = mhgMeta->resolutionLayers * mhgMeta->featuresPerEntry * mhgMeta->maxEntries;

                // float oldMomentumV = GetGridSampleValue(mhgMeta, in_momentum, l, f, pos, &oc, weightsSize * 2);
                // float momentumV = beta1 * oldMomentumV + (1.0f - beta1) * delta;
                // // if (oc)
                // // {
                // //     printf("mv = %f\n d = %f\n\n", momentumV, delta);
                // //     oc = false;
                // // }
                // AtomicAddGridSampleValue(mhgMeta, out_momentum, l, f, pos, -GetGridSampleValue(mhgMeta, out_momentum, l, f, pos, &oc, weightsSize * 2) + momentumV, &oc, weightsSize * 2);

                // float oldMomentumM = GetGridSampleValue(mhgMeta, in_momentum, l, f, pos, &oc, weightsSize * 2 + mhgSize);
                // float momentumM = beta2 * oldMomentumM + (1.0f - beta2) * (delta * delta);
                // AtomicAddGridSampleValue(mhgMeta, out_momentum, l, f, pos, -GetGridSampleValue(mhgMeta, out_momentum, l, f, pos, &oc, weightsSize * 2 + mhgSize) + momentumM, &oc, weightsSize * 2 + mhgSize);

                // momentumM = momentumM / (1.0f - beta1);
                // momentumV = momentumV / (1.0f - beta2);

                // delta = momentumM * (learningRate / (float)sqrt((double)(momentumV) + epsilon));

                float delta = learningRate * cache[InputNeuronDelta(nn, f + l * mhgMeta->featuresPerEntry, &oc)] * width * height;
                AtomicAddGridSampleValue(mhgMeta, out_mhgElems, l, f, pos, delta, &oc, 0);
            }
        }

        write_imagef(out, (int2)(x, y), (float4)(color, 1.0));
    }
    else
    {
        write_imagef(out, (int2)(x, y), (float4)(0.0, 0.0, 0.0, 1.0));
    }
}