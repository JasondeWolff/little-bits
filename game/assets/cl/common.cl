typedef struct _Camera
{
    float4 position;
	float4 lowerLeftCorner;
	float4 horizontal;
	float4 vertical;
} Camera;

typedef struct _Ray
{
    float3 origin;
    float3 direction;
} Ray;

inline void AtomicAddFloat(volatile __global float* source, const float operand)
{
    union { unsigned int intVal; float floatVal; } newVal;
    union { unsigned int intVal; float floatVal; } prevVal;
    
    do {
        prevVal.floatVal = *source;
        newVal.floatVal = prevVal.floatVal + operand;
    } 
    while (atomic_cmpxchg((volatile __global unsigned int*)source, prevVal.intVal, newVal.intVal) != prevVal.intVal);
}

inline float lerp(float a, float b, float t)
{
    return a + t * (b - a);
}

inline float3 lerp3(float3 a, float3 b, float t)
{
    float3 _t = (float3)(t, t, t);
    return a + _t * (b - a);
}