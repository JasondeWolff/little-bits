typedef struct _NeuralNetwork
{
    int inputCount;
    int hiddenCount;
    int outputCount;
    int hiddenLayerCount;
} NeuralNetwork;

int InputHiddenNeuronWeight(__global NeuralNetwork* nn, int inputIndex, int hiddenIndex, bool* oc)
{
#ifdef DEBUG_MODE
    if (inputIndex < 0 || inputIndex >= nn->inputCount || hiddenIndex < 0 || hiddenIndex >= nn->hiddenCount)
    {
        if (*oc)
        {
            printf("[OpenCL][ERROR] inputIndex or hiddenIndex is out of range. (InputHiddenNeuronWeight)\n");
            if (inputIndex < 0 || inputIndex >= nn->inputCount)
            {
                printf("inputIndex = %i range = [0, %i>\n", inputIndex, nn->inputCount);
            }
            if (hiddenIndex < 0 || hiddenIndex >= nn->hiddenCount)
            {
                printf("hiddenIndex = %i range = [0, %i>\n", hiddenIndex, nn->hiddenCount);
            }

            *oc = false;
        }
    }
#endif

    return hiddenIndex + inputIndex * nn->hiddenCount;
}

int HiddenHiddenNeuronWeight(__global NeuralNetwork* nn, int hiddenIndex, int nextHiddenIndex, int hiddenLayerIndex, bool* oc)
{
#ifdef DEBUG_MODE
    if (hiddenIndex < 0 || hiddenIndex >= nn->hiddenCount || nextHiddenIndex < 0 || nextHiddenIndex >= nn->hiddenCount || hiddenLayerIndex < 0 || hiddenLayerIndex >= nn->hiddenLayerCount - 1)
    {
        if (*oc)
        {
            printf("[OpenCL][ERROR] hiddenIndex, nextHiddenIndex or hiddenLayerIndex is out of range. (HiddenHiddenNeuronWeight)\n");
            if (hiddenIndex < 0 || hiddenIndex >= nn->hiddenCount)
            {
                printf("hiddenIndex = %i range = [0, %i>\n", hiddenIndex, nn->hiddenCount);
            }
            if (nextHiddenIndex < 0 || nextHiddenIndex >= nn->hiddenCount)
            {
                printf("nextHiddenIndex = %i range = [0, %i>\n", nextHiddenIndex, nn->hiddenCount);
            }
            if (hiddenLayerIndex < 0 || hiddenLayerIndex >= nn->hiddenLayerCount - 1)
            {
                printf("hiddenLayerIndex = %i range = [0, %i>\n", hiddenLayerIndex, nn->hiddenLayerCount - 1);
            }

            *oc = false;
        }
    }
#endif

    int inputWeightsOffset = nn->inputCount * nn->hiddenCount;
    int previousHiddenWeightsOffset = hiddenLayerIndex * nn->hiddenCount * nn->hiddenCount;
    return inputWeightsOffset + previousHiddenWeightsOffset + nextHiddenIndex + hiddenIndex * nn->hiddenCount;
}

int HiddenOutputNeuronWeight(__global NeuralNetwork* nn, int hiddenIndex, int outputIndex, bool* oc)
{
#ifdef DEBUG_MODE
    if (hiddenIndex < 0 || hiddenIndex >= nn->hiddenCount || outputIndex < 0 || outputIndex >= nn->outputCount)
    {
        if (*oc)
        {
            printf("[OpenCL][ERROR] hiddenIndex or outputIndex is out of range. (HiddenOutputNeuronWeight)\n");
            if (hiddenIndex < 0 || hiddenIndex >= nn->hiddenCount)
            {
                printf("hiddenIndex = %i range = [0, %i>\n", hiddenIndex, nn->hiddenCount);
            }
            if (outputIndex < 0 || outputIndex >= nn->outputCount)
            {
                printf("outputIndex = %i range = [0, %i>\n", outputIndex, nn->outputCount);
            }

            *oc = false;
        }
    }
#endif

    int inputWeightsOffset = nn->inputCount * nn->hiddenCount;
    int hiddenWeightsOffset = nn->hiddenCount * nn->hiddenCount * nn->hiddenLayerCount - 1;
    return inputWeightsOffset + hiddenWeightsOffset + outputIndex + hiddenIndex * nn->outputCount;
}

int InputNeuron(__global NeuralNetwork* nn, int inputIndex, bool* oc) 
{
#ifdef DEBUG_MODE
    if (inputIndex < 0 || inputIndex >= nn->inputCount)
    {
        if (*oc)
        {
            printf("[OpenCL][ERROR] inputIndex is out of range. (InputNeuron)\n");
            printf("inputIndex = %i range = [0, %i>\n", inputIndex, nn->inputCount);

            *oc = false;
        }
    }
#endif

    return inputIndex;
}

int HiddenNeuron(__global NeuralNetwork* nn, int hiddenIndex, int hiddenLayer, bool* oc) 
{
#ifdef DEBUG_MODE
    if (hiddenIndex < 0 || hiddenIndex >= nn->hiddenCount || hiddenLayer < 0 || hiddenLayer >= nn->hiddenLayerCount)
    {
        if (*oc)
        {
            printf("[OpenCL][ERROR] hiddenIndex or hiddenLayer is out of range. (HiddenNeuron)\n");
            if (hiddenIndex < 0 || hiddenIndex >= nn->hiddenCount)
            {
                printf("hiddenIndex = %i range = [0, %i>\n", hiddenIndex, nn->hiddenCount);
            }
            if (hiddenLayer < 0 || hiddenLayer >= nn->hiddenLayerCount)
            {
                printf("hiddenLayer = %i range = [0, %i>\n", hiddenLayer, nn->hiddenLayerCount);
            }

            *oc = false;
        }
    }
#endif

    return nn->inputCount + hiddenLayer * nn->hiddenCount + hiddenIndex;
}

int OutputNeuron(__global NeuralNetwork* nn, int outputIndex, bool* oc) 
{
#ifdef DEBUG_MODE
    if (outputIndex < 0 || outputIndex >= nn->outputCount)
    {
        if (*oc)
        {
            printf("[OpenCL][ERROR] outputIndex is out of range. (OutputNeuron)\n");
            printf("outputIndex = %i range = [0, %i>\n", outputIndex, nn->outputCount);

            *oc = false;
        }
    }
#endif

    return nn->inputCount + nn->hiddenLayerCount * nn->hiddenCount + outputIndex;
}

int InputNeuronDelta(__global NeuralNetwork* nn, int inputIndex, bool* oc) 
{
#ifdef DEBUG_MODE
    if (inputIndex < 0 || inputIndex >= nn->inputCount)
    {
        if (*oc)
        {
            printf("[OpenCL][ERROR] inputIndex is out of range. (InputNeuronDelta)\n");
            printf("inputIndex = %i range = [0, %i>\n", inputIndex, nn->inputCount);

            *oc = false;
        }
    }
#endif

    int neuronOffset = nn->inputCount + nn->hiddenLayerCount * nn->hiddenCount + nn->outputCount;
    return neuronOffset + inputIndex;
}

int HiddenNeuronDelta(__global NeuralNetwork* nn, int hiddenIndex, int hiddenLayer, bool* oc) 
{
#ifdef DEBUG_MODE
    if (hiddenIndex < 0 || hiddenIndex >= nn->hiddenCount || hiddenLayer < 0 || hiddenLayer >= nn->hiddenLayerCount)
    {
        if (*oc)
        {
            printf("[OpenCL][ERROR] hiddenIndex or hiddenLayer is out of range. (HiddenNeuronDelta)\n");
            if (hiddenIndex < 0 || hiddenIndex >= nn->hiddenCount)
            {
                printf("hiddenIndex = %i range = [0, %i>\n", hiddenIndex, nn->hiddenCount);
            }
            if (hiddenLayer < 0 || hiddenLayer >= nn->hiddenLayerCount)
            {
                printf("hiddenLayer = %i range = [0, %i>\n", hiddenLayer, nn->hiddenLayerCount);
            }

            *oc = false;
        }
    }
#endif

    int neuronOffset = nn->inputCount + nn->hiddenLayerCount * nn->hiddenCount + nn->outputCount;
    return neuronOffset + nn->inputCount + hiddenLayer * nn->hiddenCount + hiddenIndex;
}

int OutputNeuronDelta(__global NeuralNetwork* nn, int outputIndex, bool* oc) 
{
#ifdef DEBUG_MODE
    if (outputIndex < 0 || outputIndex >= nn->outputCount)
    {
        if (*oc)
        {
            printf("[OpenCL][ERROR] outputIndex is out of range. (OutputNeuronDelta)\n");
            printf("outputIndex = %i range = [0, %i>\n", outputIndex, nn->outputCount);

            *oc = false;
        }
    }
#endif

    int neuronOffset = nn->inputCount + nn->hiddenLayerCount * nn->hiddenCount + nn->outputCount;
    return neuronOffset + nn->inputCount + nn->hiddenLayerCount * nn->hiddenCount + outputIndex;
}

int TargetValue(__global NeuralNetwork* nn, int targetIndex, bool *oc)
{
#ifdef DEBUG_MODE
    if (targetIndex < 0 || targetIndex >= nn->outputCount)
    {
        if (*oc)
        {
            printf("[OpenCL][ERROR] targetIndex is out of range. (TargetValue)\n");
            printf("targetIndex = %i range = [0, %i>\n", targetIndex, nn->outputCount);

            *oc = false;
        }
    }
#endif

    int neuronOffset = (nn->inputCount + nn->hiddenLayerCount * nn->hiddenCount + nn->outputCount) * 2;
    return neuronOffset + targetIndex;
}

inline float ReLU(float x)
{
    return max(0.0f, x);
}

inline float DevReLU(float x)
{
    if (x < 0.0f) return 0.0f;
    return 1.0f;
}

float Sigmoid(float x)
{
    if (x < -100.0f) return 0.0f;
    if (x > 100.0f) return 1.0f;
    return 1.0f / (1.0f + exp(-x));
}

float inline DevSigmoid(float x)
{
	return x * (1.0f - x);
}

float Activation(float x)
{
    return ReLU(x);
}

float DevActivation(float x)
{
    return DevReLU(x);
}

void Forward(bool* oc, __global NeuralNetwork* nn, __global float* in_weights, __local float* cache)
{
    // Input -> Hidden
    for (int i = 0; i < nn->hiddenCount; i++)
    {
        float activation = 0.0f;
        for (int j = 0; j < nn->inputCount; j++)
        {
            activation += cache[InputNeuron(nn, j, oc)] * in_weights[InputHiddenNeuronWeight(nn, j, i, oc)];
        }

        cache[HiddenNeuron(nn, i, 0, oc)] = Activation(activation);
    }

    // Hidden -> Hidden
    for (int l = 0; l < nn->hiddenLayerCount - 1; l++)
    {
        for (int i = 0; i < nn->hiddenCount; i++)
        {
            float activation = 0.0f;
            for (int j = 0; j < nn->hiddenCount; j++)
            {
                activation += cache[HiddenNeuron(nn, j, l, oc)] * in_weights[HiddenHiddenNeuronWeight(nn, j, i, l, oc)];
            }

            cache[HiddenNeuron(nn, i, l + 1, oc)] = Activation(activation);
        }
    }

    // Hidden -> Output
    int l = nn->hiddenLayerCount - 1;
    for (int i = 0; i < nn->outputCount; i++)
    {
        float activation = 0.0f;
        for (int j = 0; j < nn->hiddenCount; j++)
        {
            activation += cache[HiddenNeuron(nn, j, l, oc)] * in_weights[HiddenOutputNeuronWeight(nn, j, i, oc)];
        }

        cache[OutputNeuron(nn, i, oc)] = activation;
    }
}

float AddMomentum(bool* oc, float delta, int idx, __global float* in_momentum, __global float* out_momentum, float beta1, float beta2, double epsilon, float learningRate)
{
#ifdef ADA_GRAD
    float oldMomentum = in_momentum[idx];
    float momentum = beta1 * oldMomentum + (1.0f - beta1) * delta;
    AtomicAddFloat(&out_momentum[idx], -out_momentum[idx] + momentum);
    delta = learningRate * momentum;
#elif defined(RMSP)
    float oldMomentum = in_momentum[idx * 2];
    float momentum = beta2 * oldMomentum + (1.0f - beta2) * (delta * delta);
    AtomicAddFloat(&out_momentum[idx * 2], -out_momentum[idx * 2] + momentum);
    delta = learningRate / (float)(sqrt((double)(momentum) + epsilon)) * delta;
#elif defined(ADAM)
    float oldMomentumV = in_momentum[idx];
    float momentumV = beta1 * oldMomentumV + (1.0f - beta1) * delta;
    AtomicAddFloat(&out_momentum[idx], -out_momentum[idx] + momentumV);

    float oldMomentumM = in_momentum[idx * 2];
    float momentumM = beta2 * oldMomentumM + (1.0f - beta2) * (delta * delta);
    AtomicAddFloat(&out_momentum[idx * 2], -out_momentum[idx * 2] + momentumM);
                
    momentumM = momentumM / (1.0f - beta1);
    momentumV = momentumV / (1.0f - beta2);

    delta = momentumM * (learningRate / (float)sqrt((double)(momentumV) + epsilon));
#endif

    return delta;
}

void Backpropagate(bool* oc, __global NeuralNetwork* nn, __global float* in_weights, __global float* out_weights, __global float* in_momentum, __global float* out_momentum, float beta1, float beta2, double epsilon, __local float* cache, float learningRate, float avgFactor, float L2reg, __global float* globalLoss)
{
    // Calculate L2 penalty
    float weightSum = 0.0f;
    float weightSumDerivative = 0.0f;
    {
        int weightsSize = nn->inputCount * nn->hiddenCount + nn->hiddenCount * nn->hiddenCount * (nn->hiddenLayerCount - 1) + nn->hiddenCount * nn->outputCount;
        for (int i = 0; i < weightsSize; i++)
        {
            float weight = in_weights[i];
            weightSum += weight * weight;
            weightSumDerivative += weight;
        }
    }

    // Calculate deltas
    {
        // Output deltas
        for (int i = 0; i < nn->outputCount; i++)
        {
            float outputNeuron = cache[OutputNeuron(nn, i, oc)];

#ifdef MSE
            float loss = (cache[TargetValue(nn, i, oc)] - outputNeuron) * (cache[TargetValue(nn, i, oc)] - outputNeuron) + L2reg * weightSum;
            float derivativeLoss = 2.0f * (cache[TargetValue(nn, i, oc)] - outputNeuron) + 2.0f * L2reg * weightSumDerivative;
#else
            float loss = (cache[TargetValue(nn, i, oc)] - outputNeuron) + L2reg * weightSum;
            float derivativeLoss = (cache[TargetValue(nn, i, oc)] - outputNeuron) + 2.0f * L2reg * weightSumDerivative;
#endif

            cache[OutputNeuronDelta(nn, i, oc)] = derivativeLoss * avgFactor;
            AtomicAddFloat(&globalLoss[0], loss * avgFactor);
        }

        // Hidden deltas
        for (int l = nn->hiddenLayerCount - 1; l >= 0; l--)
        {
            for (int i = 0; i < nn->hiddenCount; i++)
            {
                float error = 0.0f;
                if (l == nn->hiddenLayerCount - 1)
                {
                    for (int j = 0; j < nn->outputCount; j++)
                    {
                        error += in_weights[HiddenOutputNeuronWeight(nn, i, j, oc)] * cache[OutputNeuronDelta(nn, j, oc)];
                    }
                }
                else
                {
                    for (int j = 0; j < nn->hiddenCount; j++)
                    {
                        error += in_weights[HiddenHiddenNeuronWeight(nn, i, j, l, oc)] * cache[HiddenNeuronDelta(nn, j, l + 1, oc)];
                    }
                }

                cache[HiddenNeuronDelta(nn, i, l, oc)] = error * DevActivation(cache[HiddenNeuron(nn, i, l, oc)]);
            }
        }

        // Input deltas
        for (int i = 0; i < nn->inputCount; i++)
        {
            float error = 0.0f;
            for (int j = 0; j < nn->hiddenCount; j++)
            {
                error += in_weights[InputHiddenNeuronWeight(nn, i, j, oc)] * cache[HiddenNeuronDelta(nn, j, 0, oc)];
            }

            cache[InputNeuronDelta(nn, i, oc)] = error * DevActivation(cache[InputNeuron(nn, i, oc)]);
        }
    }

    // Update weights
    {
        // Hidden <- Output
        for (int i = 0; i < nn->hiddenCount; i++)
        {
            for (int j = 0; j < nn->outputCount; j++)
            {
                float delta = cache[OutputNeuronDelta(nn, j, oc)] * cache[HiddenNeuron(nn, i, nn->hiddenLayerCount - 1, oc)];
#ifdef MOMENTUM
    	        AddMomentum(oc, delta, HiddenOutputNeuronWeight(nn, i, j, oc), in_momentum, out_momentum, beta1, beta2, epsilon, learningRate);
#endif
                AtomicAddFloat(&out_weights[HiddenOutputNeuronWeight(nn, i, j, oc)], delta);
            }
        }

        // Hidden <- Hidden weights
        for (int l = 0; l < nn->hiddenLayerCount - 1; l++)
        {
            for (int i = 0; i < nn->hiddenCount; i++)
            {
                for (int j = 0; j < nn->hiddenCount; j++)
                {
                    float delta = cache[HiddenNeuronDelta(nn, j, l + 1, oc)] * cache[HiddenNeuron(nn, i, l, oc)];
#ifdef MOMENTUM
                    AddMomentum(oc, delta, HiddenHiddenNeuronWeight(nn, i, j, l, oc), in_momentum, out_momentum, beta1, beta2, epsilon, learningRate);
#endif
                    AtomicAddFloat(&out_weights[HiddenHiddenNeuronWeight(nn, i, j, l, oc)], delta);
                }
            }
        }

        // Input <- Hidden weights
        for (int i = 0; i < nn->inputCount; i++)
        {
            for (int j = 0; j < nn->hiddenCount; j++)
            {
                float delta = cache[HiddenNeuronDelta(nn, j, 0, oc)] * cache[InputNeuron(nn, i, oc)];
#ifdef MOMENTUM
                AddMomentum(oc, delta, InputHiddenNeuronWeight(nn, i, j, oc), in_momentum, out_momentum, beta1, beta2, epsilon, learningRate);
#endif
                AtomicAddFloat(&out_weights[InputHiddenNeuronWeight(nn, i, j, oc)], delta);
            }
        }
    }
}