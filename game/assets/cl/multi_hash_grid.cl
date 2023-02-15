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

int GridIndex(__global MutliHashGridMeta* mhg, int layer, int feature, int3 pos)
{
    int layerOffset = layer * mhg->maxEntries * mhg->featuresPerEntry;
    int featureOffset = mhg->maxEntries * feature;
    int xOffset = pos.x * mhg->height * mhg->depth;
    int yOffset = pos.y * mhg->depth;
    int zOffset = pos.z;
    return layerOffset + featureOffset + xOffset + yOffset + zOffset;
}

float UnitSize(__global MutliHashGridMeta* mhg, int layer)
{
    return ((float)(mhg->maxResolution) - (float)(mhg->minResolution)) / (float)(mhg->resolutionLayers) * (float)(layer);
}

float GetGridValue(__global MutliHashGridMeta* mhg, __global float* mghElems, int layer, int feature, float3 pos)
{
    float unitSize = 1.0 / UnitSize(mhg, layer);
    int3 unitPos = (int3)(round(pos.x * unitSize), round(pos.y * unitSize), round(pos.z * unitSize));
    return mghElems[GridIndex(mhg, layer, feature, unitPos)];
}

float SetGridValue(__global MutliHashGridMeta* mhg, __global float* mghElems, int layer, int feature, float3 pos, float value)
{
    float unitSize = 1.0 / UnitSize(mhg, layer);
    int3 unitPos = (int3)(round(pos.x * unitSize), round(pos.y * unitSize), round(pos.z * unitSize));
    mghElems[GridIndex(mhg, layer, feature, unitPos)] = value;
}

float GetGridSampleValue(__global MutliHashGridMeta* mhg, __global float* mghElems, int layer, int feature, float3 pos)
{
    float unitSize = 1.0 / UnitSize(mhg, layer);
    int3 lbf = (int3)(floor(pos.x * unitSize), floor(pos.y * unitSize), floor(pos.z * unitSize));
    int3 lbb = (int3)(floor(pos.x * unitSize), floor(pos.y * unitSize), ceil(pos.z * unitSize));
    int3 rbf = (int3)(ceil(pos.x * unitSize), floor(pos.y * unitSize), floor(pos.z * unitSize));
    int3 rbb = (int3)(ceil(pos.x * unitSize), floor(pos.y * unitSize), ceil(pos.z * unitSize));
    int3 ltf = (int3)(floor(pos.x * unitSize), ceil(pos.y * unitSize), floor(pos.z * unitSize));
    int3 ltb = (int3)(floor(pos.x * unitSize), ceil(pos.y * unitSize), ceil(pos.z * unitSize));
    int3 rtf = (int3)(ceil(pos.x * unitSize), ceil(pos.y * unitSize), floor(pos.z * unitSize));
    int3 rtb = (int3)(ceil(pos.x * unitSize), ceil(pos.y * unitSize), ceil(pos.z * unitSize));

    float lbfValue = mghElems[GridIndex(mhg, layer, feature, lbf)];
    float lbbValue = mghElems[GridIndex(mhg, layer, feature, lbb)];
    float rbfValue = mghElems[GridIndex(mhg, layer, feature, rbf)];
    float rbbValue = mghElems[GridIndex(mhg, layer, feature, rbb)];
    float ltfValue = mghElems[GridIndex(mhg, layer, feature, ltf)];
    float ltbValue = mghElems[GridIndex(mhg, layer, feature, ltb)];
    float rtfValue = mghElems[GridIndex(mhg, layer, feature, rtf)];
    float rtbValue = mghElems[GridIndex(mhg, layer, feature, rtb)];

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
}

float SetGridSampleValue(__global MutliHashGridMeta* mhg, __global float* mghElems, int layer, int feature, float3 pos, float value)
{
    float unitSize = 1.0 / UnitSize(mhg, layer);
    int3 lbf = (int3)(floor(pos.x * unitSize), floor(pos.y * unitSize), floor(pos.z * unitSize));
    int3 lbb = (int3)(floor(pos.x * unitSize), floor(pos.y * unitSize), ceil(pos.z * unitSize));
    int3 rbf = (int3)(ceil(pos.x * unitSize), floor(pos.y * unitSize), floor(pos.z * unitSize));
    int3 rbb = (int3)(ceil(pos.x * unitSize), floor(pos.y * unitSize), ceil(pos.z * unitSize));
    int3 ltf = (int3)(floor(pos.x * unitSize), ceil(pos.y * unitSize), floor(pos.z * unitSize));
    int3 ltb = (int3)(floor(pos.x * unitSize), ceil(pos.y * unitSize), ceil(pos.z * unitSize));
    int3 rtf = (int3)(ceil(pos.x * unitSize), ceil(pos.y * unitSize), floor(pos.z * unitSize));
    int3 rtb = (int3)(ceil(pos.x * unitSize), ceil(pos.y * unitSize), ceil(pos.z * unitSize));

    float xaxis = ceil(pos.x) - pos.x;
    float yaxis = ceil(pos.y) - pos.y;
    float zaxis = ceil(pos.z) - pos.z;

    float lValue = value * (1.0 - xaxis);
    float rValue = value * xaxis;
    float lbValue = lValue * (1.0 - yaxis);
    float rbValue = lValue * yaxis;
    float ltValue = rValue * (1.0 - yaxis);
    float rtValue = rValue * yaxis;

    mghElems[GridIndex(mhg, layer, feature, lbf)] = lbValue * (1.0 - zaxis);
    mghElems[GridIndex(mhg, layer, feature, lbb)] = lbValue * zaxis;
    mghElems[GridIndex(mhg, layer, feature, rbf)] = rbValue * (1.0 - zaxis);
    mghElems[GridIndex(mhg, layer, feature, rbb)] = rbValue * zaxis;
    mghElems[GridIndex(mhg, layer, feature, ltf)] = ltValue * (1.0 - zaxis);
    mghElems[GridIndex(mhg, layer, feature, ltb)] = ltValue * zaxis;
    mghElems[GridIndex(mhg, layer, feature, rtf)] = rtValue * (1.0 - zaxis);
    mghElems[GridIndex(mhg, layer, feature, rtb)] = rtValue * zaxis;
}

float AddGridSampleValue(__global MutliHashGridMeta* mhg, __global float* mghElems, int layer, int feature, float3 pos, float value)
{
    float unitSize = 1.0 / UnitSize(mhg, layer);
    int3 lbf = (int3)(floor(pos.x * unitSize), floor(pos.y * unitSize), floor(pos.z * unitSize));
    int3 lbb = (int3)(floor(pos.x * unitSize), floor(pos.y * unitSize), ceil(pos.z * unitSize));
    int3 rbf = (int3)(ceil(pos.x * unitSize), floor(pos.y * unitSize), floor(pos.z * unitSize));
    int3 rbb = (int3)(ceil(pos.x * unitSize), floor(pos.y * unitSize), ceil(pos.z * unitSize));
    int3 ltf = (int3)(floor(pos.x * unitSize), ceil(pos.y * unitSize), floor(pos.z * unitSize));
    int3 ltb = (int3)(floor(pos.x * unitSize), ceil(pos.y * unitSize), ceil(pos.z * unitSize));
    int3 rtf = (int3)(ceil(pos.x * unitSize), ceil(pos.y * unitSize), floor(pos.z * unitSize));
    int3 rtb = (int3)(ceil(pos.x * unitSize), ceil(pos.y * unitSize), ceil(pos.z * unitSize));

    float xaxis = ceil(pos.x) - pos.x;
    float yaxis = ceil(pos.y) - pos.y;
    float zaxis = ceil(pos.z) - pos.z;

    float lValue = value * (1.0 - xaxis);
    float rValue = value * xaxis;
    float lbValue = lValue * (1.0 - yaxis);
    float rbValue = lValue * yaxis;
    float ltValue = rValue * (1.0 - yaxis);
    float rtValue = rValue * yaxis;
    
    mghElems[GridIndex(mhg, layer, feature, lbf)] += lbValue * (1.0 - zaxis);
    mghElems[GridIndex(mhg, layer, feature, lbb)] += lbValue * zaxis;
    mghElems[GridIndex(mhg, layer, feature, rbf)] += rbValue * (1.0 - zaxis);
    mghElems[GridIndex(mhg, layer, feature, rbb)] += rbValue * zaxis;
    mghElems[GridIndex(mhg, layer, feature, ltf)] += ltValue * (1.0 - zaxis);
    mghElems[GridIndex(mhg, layer, feature, ltb)] += ltValue * zaxis;
    mghElems[GridIndex(mhg, layer, feature, rtf)] += rtValue * (1.0 - zaxis);
    mghElems[GridIndex(mhg, layer, feature, rtb)] += rtValue * zaxis;
}