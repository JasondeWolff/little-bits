typedef struct _MultiHashGridMeta
{
    int resolutionLayers;
    int maxEntries;
    int featuresPerEntry;

    int minResolution;
    int maxResolution;

    int width;
    int height;
    int depth;
} MutliHashGridMeta;

inline float Resolution(__global MutliHashGridMeta* mhg, int layer)
{
    return lerp((float)(mhg->minResolution), (float)(mhg->maxResolution), (float)(layer) / (float)(mhg->resolutionLayers - 1)); // BREAKS IF resolutionLayers <= 1
}

// Source: https://www.researchgate.net/publication/2909661_Optimized_Spatial_Hashing_for_Collision_Detection_of_Deformable_Objects
inline int SpatialHash(int3 pos, int T)
{
    return ((pos.x * 73856093) ^ (pos.y * 19349663) ^ (pos.z * 83492791)) % T;
}

int GridIndex(__global MutliHashGridMeta* mhg, int layer, int feature, int3 pos, bool* oc)
{
    int layerOffset = layer * mhg->maxEntries * mhg->featuresPerEntry;
    int featureOffset = mhg->maxEntries * feature;

    int resolution = (int)(Resolution(mhg, layer));

    int index;
    // Encode if maxEntries is exceeded
    if ((float)(resolution) >= cbrt((float)(mhg->maxEntries)))
    {
        index = layerOffset + featureOffset + SpatialHash(pos, mhg->maxEntries);
    }
    else
    {
        int xOffset = pos.x * resolution * resolution;
        int yOffset = pos.y * resolution;
        int zOffset = pos.z;
        index = layerOffset + featureOffset + xOffset + yOffset + zOffset;
    }

 #ifdef DEBUG_MODE
    if (layer < 0 || layer >= mhg->resolutionLayers || feature < 0 || feature >= mhg->featuresPerEntry)
    {
        if (*oc)
        {
            printf("[OpenCL][ERROR] layer or feature is out of range. (GridIndex)\n");
            if (layer < 0 || layer >= mhg->resolutionLayers)
            {
                printf("layer = %i range = [0, %i>\n", layer,  mhg->resolutionLayers);
            }
            if (feature < 0 || feature >= mhg->featuresPerEntry)
            {
                printf("feature = %i range = [0, %i>\n", feature, mhg->featuresPerEntry);
            }

            *oc = false;
        }
    }

    if (pos.x < 0 || pos.x >= mhg->width || pos.y < 0 || pos.y >= mhg->height || pos.z < 0 || pos.z >= mhg->depth)
    {
        if (*oc)
        {
            printf("[OpenCL][ERROR] pos is out of range. (GridIndex)\n");
            if (pos.x < 0 || pos.x >= mhg->width)
            {
                printf("pos.x = %i range = [0, %i>\n", pos.x,  mhg->width);
            }
            if (pos.y < 0 || pos.y >= mhg->height)
            {
                printf("pos.y = %i range = [0, %i>\n", pos.y, mhg->height);
                printf("r = %i maxEntries = %i\nindex = %i range = [0, %i>\nlayer = %i range = [0, %i>\nfeature = %i range = [0, %i>\npos.y = %i range = [0, %i>\npos.y = %i range = [0, %i>\npos.z = %i range = [0, %i>\n\n", resolution, mhg->maxEntries, index, mhg->resolutionLayers * mhg->maxEntries * mhg->featuresPerEntry, layer, mhg->resolutionLayers, feature, mhg->featuresPerEntry, pos.x, mhg->width, pos.y, mhg->height, pos.z, mhg->depth);
            }
            if (pos.z < 0 || pos.z >= mhg->depth)
            {
                printf("pos.z = %i range = [0, %i>\n", pos.z, mhg->depth);
            }

            *oc = false;
        }
    }

    if (index >= mhg->resolutionLayers * mhg->maxEntries * mhg->featuresPerEntry)
    {
        if (*oc)
        {
            printf("[OpenCL][ERROR] index is out of range. (GridIndex)\n");
            printf("r = %i maxEntries = %i\nindex = %i range = [0, %i>\nlayer = %i range = [0, %i>\nfeature = %i range = [0, %i>\npos.y = %i range = [0, %i>\npos.y = %i range = [0, %i>\npos.z = %i range = [0, %i>\n\n", resolution, mhg->maxEntries, index, mhg->resolutionLayers * mhg->maxEntries * mhg->featuresPerEntry, layer, mhg->resolutionLayers, feature, mhg->featuresPerEntry, pos.x, mhg->width, pos.y, mhg->height, pos.z, mhg->depth);

            *oc = false;
        }
    }
#endif

    return index;
}

float GetGridValue(__global MutliHashGridMeta* mhg, __global float* mghElems, int layer, int feature, float3 pos, bool* oc)
{
    pos *= (float3)(1000.0f, 1000.0f, 1000.0f);

    float resolution = Resolution(mhg, layer);
    float xresInv = resolution / (float)(mhg->width);
    float yresInv = resolution / (float)(mhg->height);
    float zresInv = resolution / (float)(mhg->depth);

    int3 unitPos = (int3)(round(pos.x * xresInv), round(pos.y * yresInv), round(pos.z * zresInv));
    return mghElems[GridIndex(mhg, layer, feature, unitPos, oc)];
}

void SetGridValue(__global MutliHashGridMeta* mhg, __global float* mghElems, int layer, int feature, float3 pos, float value, bool* oc)
{
    pos *= (float3)(1000.0f, 1000.0f, 1000.0f);

    float resolution = Resolution(mhg, layer);
    float xresInv = resolution / (float)(mhg->width);
    float yresInv = resolution / (float)(mhg->height);
    float zresInv = resolution / (float)(mhg->depth);

    int3 unitPos = (int3)(round(pos.x * xresInv), round(pos.y * yresInv), round(pos.z * zresInv));
    mghElems[GridIndex(mhg, layer, feature, unitPos, oc)] = value;
}

float GetGridSampleValue(__global MutliHashGridMeta* mhg, __global float* mghElems, int layer, int feature, float3 pos, bool* oc)
{
    pos *= (float3)(1000.0f, 1000.0f, 1000.0f);

#ifdef DEBUG_MODE
    if ((int)(pos.x) < 0 || (int)(pos.x) >= mhg->width || (int)(pos.y) < 0 || (int)(pos.y) >= mhg->height || (int)(pos.z) < 0 || (int)(pos.z) >= mhg->depth)
    {
        if (*oc)
        {
            printf("[OpenCL][ERROR] pos is out of range. (GetGridSampleValue)\n");
            if ((int)(pos.x) < 0 || (int)(pos.x) >= mhg->width)
            {
                printf("(GetGridSampleValue) pos.x = %i range = [0, %i>\n", (int)(pos.x),  mhg->width);
            }
            if ((int)(pos.y) < 0 || (int)(pos.y) >= mhg->height)
            {
                printf("(GetGridSampleValue) pos.y = %i range = [0, %i>\n", (int)(pos.y), mhg->height);
            }
            if ((int)(pos.z) < 0 || (int)(pos.z) >= mhg->depth)
            {
                printf("(GetGridSampleValue) pos.z = %i range = [0, %i>\n", (int)(pos.z), mhg->depth);
            }

            *oc = false;
        }
    }
#endif

    // float invResolution = 1.0f / Resolution(mhg, layer);
    // float xresInv = 1.0f / ((float)(mhg->width) * invResolution);
    // float yresInv = 1.0f / ((float)(mhg->height) * invResolution);
    // float zresInv = 1.0f / ((float)(mhg->depth) * invResolution);

    float resolution = Resolution(mhg, layer);
    float xresInv = resolution / (float)(mhg->width);
    float yresInv = resolution / (float)(mhg->height);
    float zresInv = resolution / (float)(mhg->depth);

    int3 lbf = (int3)(floor(pos.x * xresInv), floor(pos.y * yresInv), floor(pos.z * zresInv));
    int3 lbb = (int3)(floor(pos.x * xresInv), floor(pos.y * yresInv), ceil(pos.z * zresInv));
    int3 rbf = (int3)(ceil(pos.x * xresInv), floor(pos.y * yresInv), floor(pos.z * zresInv));
    int3 rbb = (int3)(ceil(pos.x * xresInv), floor(pos.y * yresInv), ceil(pos.z * zresInv));
    int3 ltf = (int3)(floor(pos.x * xresInv), ceil(pos.y * yresInv), floor(pos.z * zresInv));
    int3 ltb = (int3)(floor(pos.x * xresInv), ceil(pos.y * yresInv), ceil(pos.z * zresInv));
    int3 rtf = (int3)(ceil(pos.x * xresInv), ceil(pos.y * yresInv), floor(pos.z * zresInv));
    int3 rtb = (int3)(ceil(pos.x * xresInv), ceil(pos.y * yresInv), ceil(pos.z * zresInv));

    float lbfValue = mghElems[GridIndex(mhg, layer, feature, lbf, oc)];
    float lbbValue = mghElems[GridIndex(mhg, layer, feature, lbb, oc)];
    float rbfValue = mghElems[GridIndex(mhg, layer, feature, rbf, oc)];
    float rbbValue = mghElems[GridIndex(mhg, layer, feature, rbb, oc)];
    float ltfValue = mghElems[GridIndex(mhg, layer, feature, ltf, oc)];
    float ltbValue = mghElems[GridIndex(mhg, layer, feature, ltb, oc)];
    float rtfValue = mghElems[GridIndex(mhg, layer, feature, rtf, oc)];
    float rtbValue = mghElems[GridIndex(mhg, layer, feature, rtb, oc)];

    float xaxis = ceil(pos.x) - pos.x;
    float yaxis = ceil(pos.y) - pos.y;
    float zaxis = ceil(pos.z) - pos.z;

    float lbValue = lerp(lbfValue, lbbValue, zaxis);
    float rbValue = lerp(rbfValue, lbbValue, zaxis);
    float ltValue = lerp(ltfValue, ltbValue, zaxis);
    float rtValue = lerp(rtfValue, rtbValue, zaxis);
    float lValue = lerp(lbValue, ltValue, yaxis);
    float rValue = lerp(rbValue, rtValue, yaxis);
    float value = lerp(lValue, rValue, xaxis);

    return value;

    return 0.0f;
}

void SetGridSampleValue(__global MutliHashGridMeta* mhg, __global float* mghElems, int layer, int feature, float3 pos, float value, bool* oc)
{
    pos *= (float3)(1000.0f, 1000.0f, 1000.0f);

#ifdef DEBUG_MODE
    if ((int)(pos.x) < 0 || (int)(pos.x) >= mhg->width || (int)(pos.y) < 0 || (int)(pos.y) >= mhg->height || (int)(pos.z) < 0 || (int)(pos.z) >= mhg->depth)
    {
        if (*oc)
        {
            printf("[OpenCL][ERROR] pos is out of range. (SetGridSampleValue)\n");
            if ((int)(pos.x) < 0 || (int)(pos.x) >= mhg->width)
            {
                printf("pos.x = %i range = [0, %i>\n", (int)(pos.x),  mhg->width);
            }
            if ((int)(pos.y) < 0 || (int)(pos.y) >= mhg->height)
            {
                printf("pos.y = %i range = [0, %i>\n", (int)(pos.y), mhg->height);
            }
            if ((int)(pos.z) < 0 || (int)(pos.z) >= mhg->depth)
            {
                printf("pos.z = %i range = [0, %i>\n", (int)(pos.z), mhg->depth);
            }

            *oc = false;
        }
    }
#endif

    float resolution = Resolution(mhg, layer);
    float xresInv = resolution / (float)(mhg->width);
    float yresInv = resolution / (float)(mhg->height);
    float zresInv = resolution / (float)(mhg->depth);

    int3 lbf = (int3)(floor(pos.x * xresInv), floor(pos.y * yresInv), floor(pos.z * zresInv));
    int3 lbb = (int3)(floor(pos.x * xresInv), floor(pos.y * yresInv), ceil(pos.z * zresInv));
    int3 rbf = (int3)(ceil(pos.x * xresInv), floor(pos.y * yresInv), floor(pos.z * zresInv));
    int3 rbb = (int3)(ceil(pos.x * xresInv), floor(pos.y * yresInv), ceil(pos.z * zresInv));
    int3 ltf = (int3)(floor(pos.x * xresInv), ceil(pos.y * yresInv), floor(pos.z * zresInv));
    int3 ltb = (int3)(floor(pos.x * xresInv), ceil(pos.y * yresInv), ceil(pos.z * zresInv));
    int3 rtf = (int3)(ceil(pos.x * xresInv), ceil(pos.y * yresInv), floor(pos.z * zresInv));
    int3 rtb = (int3)(ceil(pos.x * xresInv), ceil(pos.y * yresInv), ceil(pos.z * zresInv));

    float xaxis = ceil(pos.x) - pos.x;
    float yaxis = ceil(pos.y) - pos.y;
    float zaxis = ceil(pos.z) - pos.z;

    float lValue = value * (1.0 - xaxis);
    float rValue = value * xaxis;
    float lbValue = lValue * (1.0 - yaxis);
    float rbValue = lValue * yaxis;
    float ltValue = rValue * (1.0 - yaxis);
    float rtValue = rValue * yaxis;

    mghElems[GridIndex(mhg, layer, feature, lbf, oc)] = lbValue * (1.0 - zaxis);
    mghElems[GridIndex(mhg, layer, feature, lbb, oc)] = lbValue * zaxis;
    mghElems[GridIndex(mhg, layer, feature, rbf, oc)] = rbValue * (1.0 - zaxis);
    mghElems[GridIndex(mhg, layer, feature, rbb, oc)] = rbValue * zaxis;
    mghElems[GridIndex(mhg, layer, feature, ltf, oc)] = ltValue * (1.0 - zaxis);
    mghElems[GridIndex(mhg, layer, feature, ltb, oc)] = ltValue * zaxis;
    mghElems[GridIndex(mhg, layer, feature, rtf, oc)] = rtValue * (1.0 - zaxis);
    mghElems[GridIndex(mhg, layer, feature, rtb, oc)] = rtValue * zaxis;
}

void AtomicAddGridSampleValue(__global MutliHashGridMeta* mhg, __global float* mghElems, int layer, int feature, float3 pos, float value, bool* oc)
{
    pos *= (float3)(1000.0f, 1000.0f, 1000.0f);

#ifdef DEBUG_MODE
    if ((int)(pos.x) < 0 || (int)(pos.x) >= mhg->width || (int)(pos.y) < 0 || (int)(pos.y) >= mhg->height || (int)(pos.z) < 0 || (int)(pos.z) >= mhg->depth)
    {
        if (*oc)
        {
            printf("[OpenCL][ERROR] pos is out of range. (AtomicAddGridSampleValue)\n");
            if ((int)(pos.x) < 0 || (int)(pos.x) >= mhg->width)
            {
                printf("(AtomicAddGridSampleValue) pos.x = %i range = [0, %i>\n", (int)(pos.x),  mhg->width);
            }
            if ((int)(pos.y) < 0 || (int)(pos.y) >= mhg->height)
            {
                printf("(AtomicAddGridSampleValue) pos.y = %i range = [0, %i>\n", (int)(pos.y), mhg->height);
            }
            if ((int)(pos.z) < 0 || (int)(pos.z) >= mhg->depth)
            {
                printf("(AtomicAddGridSampleValue) pos.z = %i range = [0, %i>\n", (int)(pos.z), mhg->depth);
            }

            *oc = false;
        }
    }
#endif

    float resolution = Resolution(mhg, layer);
    float xresInv = resolution / (float)(mhg->width);
    float yresInv = resolution / (float)(mhg->height);
    float zresInv = resolution / (float)(mhg->depth);

    int3 lbf = (int3)(floor(pos.x * xresInv), floor(pos.y * yresInv), floor(pos.z * zresInv));
    int3 lbb = (int3)(floor(pos.x * xresInv), floor(pos.y * yresInv), ceil(pos.z * zresInv));
    int3 rbf = (int3)(ceil(pos.x * xresInv), floor(pos.y * yresInv), floor(pos.z * zresInv));
    int3 rbb = (int3)(ceil(pos.x * xresInv), floor(pos.y * yresInv), ceil(pos.z * zresInv));
    int3 ltf = (int3)(floor(pos.x * xresInv), ceil(pos.y * yresInv), floor(pos.z * zresInv));
    int3 ltb = (int3)(floor(pos.x * xresInv), ceil(pos.y * yresInv), ceil(pos.z * zresInv));
    int3 rtf = (int3)(ceil(pos.x * xresInv), ceil(pos.y * yresInv), floor(pos.z * zresInv));
    int3 rtb = (int3)(ceil(pos.x * xresInv), ceil(pos.y * yresInv), ceil(pos.z * zresInv));

    float xaxis = ceil(pos.x) - pos.x;
    float yaxis = ceil(pos.y) - pos.y;
    float zaxis = ceil(pos.z) - pos.z;

    float lValue = value * (1.0 - xaxis);
    float rValue = value * xaxis;
    float lbValue = lValue * (1.0 - yaxis);
    float rbValue = lValue * yaxis;
    float ltValue = rValue * (1.0 - yaxis);
    float rtValue = rValue * yaxis;
    
    AtomicAddFloat(&mghElems[GridIndex(mhg, layer, feature, lbf, oc)], lbValue * (1.0 - zaxis));
    AtomicAddFloat(&mghElems[GridIndex(mhg, layer, feature, lbb, oc)], lbValue * zaxis);
    AtomicAddFloat(&mghElems[GridIndex(mhg, layer, feature, rbf, oc)], rbValue * (1.0 - zaxis));
    AtomicAddFloat(&mghElems[GridIndex(mhg, layer, feature, rbb, oc)], rbValue * zaxis);
    AtomicAddFloat(&mghElems[GridIndex(mhg, layer, feature, ltf, oc)], ltValue * (1.0 - zaxis));
    AtomicAddFloat(&mghElems[GridIndex(mhg, layer, feature, ltb, oc)], ltValue * zaxis);
    AtomicAddFloat(&mghElems[GridIndex(mhg, layer, feature, rtf, oc)], rtValue * (1.0 - zaxis));
    AtomicAddFloat(&mghElems[GridIndex(mhg, layer, feature, rtb, oc)], rtValue * zaxis);
}