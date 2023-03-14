typedef struct _MultiHashGridMeta
{
    int resolutionLayers;
    int maxEntries;
    int featuresPerEntry;

    int minResolution;
    int maxResolution;

    float width;
    float height;
    float depth;
} MutliHashGridMeta;

inline float ln(float x)
{
    return log10(x) / log10(2.71828f);
}

inline float Resolution(__global MutliHashGridMeta* mhg, int layer)
{
    if (mhg->resolutionLayers <= 1)
    {
        return mhg->minResolution;
    }

    return round((float)(mhg->minResolution) * exp((ln((float)(mhg->maxResolution)) - ln((float)(mhg->minResolution))) / ((float)(layer + 1))));
}

// Source: https://www.researchgate.net/publication/2909661_Optimized_Spatial_Hashing_for_Collision_Detection_of_Deformable_Objects
inline uint SpatialHash(int3 pos, int T)
{
    return (((uint)(pos.x) * 73856093) ^ ((uint)(pos.y) * 19349663) ^ ((uint)(pos.z) * 83492791)) % (uint)(T);
    //return (((uint)(pos.x) * 1) ^ ((uint)(pos.y) * 2654435761) ^ ((uint)(pos.z) * 805459861)) % (uint)(T);
}

int GridIndex(__global MutliHashGridMeta* mhg, int layer, int feature, int3 pos, bool* oc, float res, float invx, float invy, float invz, float3 pp)
{
    int layerOffset = layer * mhg->maxEntries * mhg->featuresPerEntry;
    int featureOffset = mhg->maxEntries * feature;

    int index;
    // Encode if maxEntries is exceeded
    if (res >= cbrt((float)(mhg->maxEntries)))
    {
        index = layerOffset + featureOffset + SpatialHash(pos, mhg->maxEntries);
    }
    else
    {
        int _res = (int)(res);
        int xOffset = pos.x * _res * _res;
        int yOffset = pos.y * _res;
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

    if (pos.x < 0 || pos.x >= (int)(res) || pos.y < 0 || pos.y >= (int)(res) || pos.z < 0 || pos.z >= (int)(res))
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
                printf("r = %f xyz = %f %f %f whd = %f %f %f pos = %f %f %f\nr = %i maxEntries = %i\nindex = %i range = [0, %i>\nlayer = %i range = [0, %i>\nfeature = %i range = [0, %i>\npos.y = %i range = [0, %i>\npos.y = %i range = [0, %i>\npos.z = %i range = [0, %i>\n\n", res, invx, invy, invz, mhg->width, mhg->height, mhg->depth, pp.x, pp.y, pp.z, (int)(res), mhg->maxEntries, index, mhg->resolutionLayers * mhg->maxEntries * mhg->featuresPerEntry, layer, mhg->resolutionLayers, feature, mhg->featuresPerEntry, pos.x, mhg->width, pos.y, mhg->height, pos.z, mhg->depth);
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
            printf("r = %i maxEntries = %i\nindex = %i range = [0, %i>\nlayer = %i range = [0, %i>\nfeature = %i range = [0, %i>\npos.y = %i range = [0, %i>\npos.y = %i range = [0, %i>\npos.z = %i range = [0, %i>\n\n", (int)(res), mhg->maxEntries, index, mhg->resolutionLayers * mhg->maxEntries * mhg->featuresPerEntry, layer, mhg->resolutionLayers, feature, mhg->featuresPerEntry, pos.x, mhg->width, pos.y, mhg->height, pos.z, mhg->depth);

            *oc = false;
        }
    }
#endif

    return index;
}

float GetGridValue(__global MutliHashGridMeta* mhg, __global float* mghElems, int layer, int feature, float3 pos, bool* oc)
{
    float resolution = Resolution(mhg, layer);
    float xresInv = resolution / (float)(mhg->width);
    float yresInv = resolution / (float)(mhg->height);
    float zresInv = resolution / (float)(mhg->depth);

    int3 unitPos = (int3)(round(pos.x * xresInv), round(pos.y * yresInv), round(pos.z * zresInv));
    return mghElems[GridIndex(mhg, layer, feature, unitPos, oc, resolution,
        xresInv, yresInv, zresInv, pos)];
}

void SetGridValue(__global MutliHashGridMeta* mhg, __global float* mghElems, int layer, int feature, float3 pos, float value, bool* oc)
{
    float resolution = Resolution(mhg, layer);
    float xresInv = resolution / (float)(mhg->width);
    float yresInv = resolution / (float)(mhg->height);
    float zresInv = resolution / (float)(mhg->depth);

    int3 unitPos = (int3)(round(pos.x * xresInv), round(pos.y * yresInv), round(pos.z * zresInv));
    mghElems[GridIndex(mhg, layer, feature, unitPos, oc, resolution, xresInv, yresInv, zresInv, pos)] = value;
}

inline float inclceil(float x)
{
    return floor(x + 1.0f);
}

float GetGridSampleValue(__global MutliHashGridMeta* mhg, __global float* mghElems, int layer, int feature, float3 pos, bool* oc, int offset)
{
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

    float resolution = Resolution(mhg, layer);
    float xresInv = resolution / (float)(mhg->width);
    float yresInv = resolution / (float)(mhg->height);
    float zresInv = resolution / (float)(mhg->depth);

    int x0 = (int)(pos.x * xresInv);
    int x1 = x0 + 1;
    int y0 = (int)(pos.y * yresInv);
    int y1 = y0 + 1;
    int z0 = (int)(pos.z * zresInv);
    int z1 = z0 + 1;

    x1 = clamp(x1, x0, (int)(resolution));
    y1 = clamp(y1, y0, (int)(resolution));
    z1 = clamp(z1, z0, (int)(resolution));

    int3 lbf = (int3)(x0, y0, z0);
    int3 lbb = (int3)(x0, y0, z1);
    int3 rbf = (int3)(x1, y0, z0);
    int3 rbb = (int3)(x1, y0, z1);
    int3 ltf = (int3)(x0, y1, z0);
    int3 ltb = (int3)(x0, y1, z1);
    int3 rtf = (int3)(x1, y1, z0);
    int3 rtb = (int3)(x1, y1, z1);

    float lbfValue = mghElems[GridIndex(mhg, layer, feature, lbf, oc, resolution, xresInv, yresInv, zresInv, pos) + offset];
    float lbbValue = mghElems[GridIndex(mhg, layer, feature, lbb, oc, resolution, xresInv, yresInv, zresInv, pos) + offset];
    float rbfValue = mghElems[GridIndex(mhg, layer, feature, rbf, oc, resolution, xresInv, yresInv, zresInv, pos) + offset];
    float rbbValue = mghElems[GridIndex(mhg, layer, feature, rbb, oc, resolution, xresInv, yresInv, zresInv, pos) + offset];
    float ltfValue = mghElems[GridIndex(mhg, layer, feature, ltf, oc, resolution, xresInv, yresInv, zresInv, pos) + offset];
    float ltbValue = mghElems[GridIndex(mhg, layer, feature, ltb, oc, resolution, xresInv, yresInv, zresInv, pos) + offset];
    float rtfValue = mghElems[GridIndex(mhg, layer, feature, rtf, oc, resolution, xresInv, yresInv, zresInv, pos) + offset];
    float rtbValue = mghElems[GridIndex(mhg, layer, feature, rtb, oc, resolution, xresInv, yresInv, zresInv, pos) + offset];

    float xaxis = fmod(pos.x, xresInv) / xresInv;
    float yaxis = fmod(pos.y, yresInv) / yresInv;
    float zaxis = fmod(pos.z, zresInv) / zresInv;

    float lbValue = lerp(lbfValue, lbbValue, zaxis);
    float rbValue = lerp(rbfValue, lbbValue, zaxis);
    float ltValue = lerp(ltfValue, ltbValue, zaxis);
    float rtValue = lerp(rtfValue, rtbValue, zaxis);
    float lValue = lerp(lbValue, ltValue, yaxis);
    float rValue = lerp(rbValue, rtValue, yaxis);
    float value = lerp(lValue, rValue, xaxis);

    return value;
}

void SetGridSampleValue(__global MutliHashGridMeta* mhg, __global float* mghElems, int layer, int feature, float3 pos, float value, bool* oc)
{
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

    int x0 = (int)(pos.x * xresInv);
    int x1 = x0 + 1;
    int y0 = (int)(pos.y * yresInv);
    int y1 = y0 + 1;
    int z0 = (int)(pos.z * zresInv);
    int z1 = z0 + 1;

    x1 = clamp(x1, x0, (int)(resolution));
    y1 = clamp(y1, y0, (int)(resolution));
    z1 = clamp(z1, z0, (int)(resolution));

    int3 lbf = (int3)(x0, y0, z0);
    int3 lbb = (int3)(x0, y0, z1);
    int3 rbf = (int3)(x1, y0, z0);
    int3 rbb = (int3)(x1, y0, z1);
    int3 ltf = (int3)(x0, y1, z0);
    int3 ltb = (int3)(x0, y1, z1);
    int3 rtf = (int3)(x1, y1, z0);
    int3 rtb = (int3)(x1, y1, z1);

    float xaxis = fmod(pos.x, xresInv) / xresInv;
    float yaxis = fmod(pos.y, yresInv) / yresInv;
    float zaxis = fmod(pos.z, zresInv) / zresInv;

    float lValue = value * (1.0 - xaxis);
    float rValue = value * xaxis;
    float lbValue = lValue * (1.0 - yaxis);
    float ltValue = lValue * yaxis;
    float rbValue = rValue * (1.0 - yaxis);
    float rtValue = rValue * yaxis;

    mghElems[GridIndex(mhg, layer, feature, lbf, oc, resolution, xresInv, yresInv, zresInv, pos)] = lbValue * (1.0 - zaxis);
    mghElems[GridIndex(mhg, layer, feature, lbb, oc, resolution, xresInv, yresInv, zresInv, pos)] = lbValue * zaxis;
    mghElems[GridIndex(mhg, layer, feature, rbf, oc, resolution, xresInv, yresInv, zresInv, pos)] = rbValue * (1.0 - zaxis);
    mghElems[GridIndex(mhg, layer, feature, rbb, oc, resolution, xresInv, yresInv, zresInv, pos)] = rbValue * zaxis;
    mghElems[GridIndex(mhg, layer, feature, ltf, oc, resolution, xresInv, yresInv, zresInv, pos)] = ltValue * (1.0 - zaxis);
    mghElems[GridIndex(mhg, layer, feature, ltb, oc, resolution, xresInv, yresInv, zresInv, pos)] = ltValue * zaxis;
    mghElems[GridIndex(mhg, layer, feature, rtf, oc, resolution, xresInv, yresInv, zresInv, pos)] = rtValue * (1.0 - zaxis);
    mghElems[GridIndex(mhg, layer, feature, rtb, oc, resolution, xresInv, yresInv, zresInv, pos)] = rtValue * zaxis;
}

void AtomicAddGridSampleValue(__global MutliHashGridMeta* mhg, __global float* mghElems, int layer, int feature, float3 pos, float value, bool* oc, int offset)
{
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

    int x0 = (int)(pos.x * xresInv);
    int x1 = x0 + 1;
    int y0 = (int)(pos.y * yresInv);
    int y1 = y0 + 1;
    int z0 = (int)(pos.z * zresInv);
    int z1 = z0 + 1;

    x1 = clamp(x1, x0, (int)(resolution));
    y1 = clamp(y1, y0, (int)(resolution));
    z1 = clamp(z1, z0, (int)(resolution));

    int3 lbf = (int3)(x0, y0, z0);
    int3 lbb = (int3)(x0, y0, z1);
    int3 rbf = (int3)(x1, y0, z0);
    int3 rbb = (int3)(x1, y0, z1);
    int3 ltf = (int3)(x0, y1, z0);
    int3 ltb = (int3)(x0, y1, z1);
    int3 rtf = (int3)(x1, y1, z0);
    int3 rtb = (int3)(x1, y1, z1);

    float xaxis = fmod(pos.x, xresInv) / xresInv;
    float yaxis = fmod(pos.y, yresInv) / yresInv;
    float zaxis = fmod(pos.z, zresInv) / zresInv;

    float lValue = value * (1.0 - xaxis);
    float rValue = value * xaxis;
    float lbValue = lValue * (1.0 - yaxis);
    float ltValue = lValue * yaxis;
    float rbValue = rValue * (1.0 - yaxis);
    float rtValue = rValue * yaxis;
    
    AtomicAddFloat(&mghElems[GridIndex(mhg, layer, feature, lbf, oc, resolution, xresInv, yresInv, zresInv, pos) + offset], lbValue * (1.0 - zaxis));
    AtomicAddFloat(&mghElems[GridIndex(mhg, layer, feature, lbb, oc, resolution, xresInv, yresInv, zresInv, pos) + offset], lbValue * zaxis);
    AtomicAddFloat(&mghElems[GridIndex(mhg, layer, feature, rbf, oc, resolution, xresInv, yresInv, zresInv, pos) + offset], rbValue * (1.0 - zaxis));
    AtomicAddFloat(&mghElems[GridIndex(mhg, layer, feature, rbb, oc, resolution, xresInv, yresInv, zresInv, pos) + offset], rbValue * zaxis);
    AtomicAddFloat(&mghElems[GridIndex(mhg, layer, feature, ltf, oc, resolution, xresInv, yresInv, zresInv, pos) + offset], ltValue * (1.0 - zaxis));
    AtomicAddFloat(&mghElems[GridIndex(mhg, layer, feature, ltb, oc, resolution, xresInv, yresInv, zresInv, pos) + offset], ltValue * zaxis);
    AtomicAddFloat(&mghElems[GridIndex(mhg, layer, feature, rtf, oc, resolution, xresInv, yresInv, zresInv, pos) + offset], rtValue * (1.0 - zaxis));
    AtomicAddFloat(&mghElems[GridIndex(mhg, layer, feature, rtb, oc, resolution, xresInv, yresInv, zresInv, pos) + offset], rtValue * zaxis);
}