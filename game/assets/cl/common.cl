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
    float3 invDirection;
} Ray;

typedef struct _AABB
{
    float3 low;
    float3 high;
} AABB;

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

float RayAABBIntersection(Ray* ray, __global AABB* aabb)
{
    float3 lo = aabb->low;
    float3 hi = aabb->high;
    
    float tx1 = (lo.x - ray->origin.x) * ray->invDirection.x;
    float tx2 = (hi.x - ray->origin.x) * ray->invDirection.x;

    float tmin = min(tx1, tx2);
    float tmax = max(tx1, tx2);

    float ty1 = (lo.y - ray->origin.y) * ray->invDirection.y;
    float ty2 = (hi.y - ray->origin.y) * ray->invDirection.y;

    tmin = max(tmin, min(ty1, ty2));
    tmax = min(tmax, max(ty1, ty2));

    float tz1 = (lo.z - ray->origin.z) * ray->invDirection.z;
    float tz2 = (hi.z - ray->origin.z) * ray->invDirection.z;

    tmin = max(tmin, min(tz1, tz2));
    tmax = min(tmax, max(tz1, tz2));

    if (tmax >= max(0.0f, tmin) && tmin < 99999.9f)
        return tmin;
    return -1.0f;
}

bool PointAABBIntersection(float3 p, __global AABB* aabb)
{
    float3 lo = aabb->low;
    float3 hi = aabb->high;
    
    if (p.x < lo.x || p.x > hi.x || p.y < lo.y || p.y > hi.y || p.z < lo.z || p.z > hi.z)
        return false;
    return true;
}