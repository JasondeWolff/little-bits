typedef struct _RandState
{
	ulong4 state;
} RandState;

ulong SplitMix(ulong* state)
{
	ulong result = (*state += 0x9E3779B97f4A7C15);
	result = (result ^ (result >> 30)) * 0xBF58476D1CE4E5B9;
	result = (result ^ (result >> 27)) * 0x94D049BB133111EB;
	return result ^ (result >> 31);
}

ulong Rol(ulong x, int k)
{
	return (x << k) | (x >> (64 - k));
}

RandState RandStateFromSeed(ulong seed)
{
	RandState rState;
	ulong smstate = seed;
	rState.state.x = SplitMix(&smstate);
	rState.state.y = SplitMix(&smstate);
	rState.state.z = SplitMix(&smstate);
	rState.state.w = SplitMix(&smstate);
	return rState;
}

ulong RandULong(RandState* rState)
{
	const ulong result = rState->state.x + rState->state.w;
	const ulong t = rState->state.y << 17;

	rState->state.z ^= rState->state.x;
	rState->state.w ^= rState->state.y;
	rState->state.y ^= rState->state.z;
	rState->state.x ^= rState->state.w;

	rState->state.z ^= t;
	rState->state.w = Rol(rState->state.w, 45);
	return result;
}

float RandFloat(RandState* rState)
{
	return (float)(RandULong(rState)) / (float)(18446744073709551615UL);
}

float RandFloatRanged(RandState* rState, float min, float max)
{
	return min + RandFloat(rState) * (max - min);
}