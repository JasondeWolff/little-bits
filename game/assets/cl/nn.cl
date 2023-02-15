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

int HiddenNeuronBias(__global NeuralNetwork* nn, int hiddenIndex, int hiddenLayer, bool* oc) 
{
#ifdef DEBUG_MODE
    if (hiddenIndex < 0 || hiddenIndex >= nn->hiddenCount || hiddenLayer < 0 || hiddenLayer >= nn->hiddenLayerCount)
    {
        if (*oc)
        {
            printf("[OpenCL][ERROR] hiddenIndex or hiddenLayer is out of range. (HiddenNeuronBias)\n");
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

    int weightsOffset = nn->inputCount * nn->hiddenCount + nn->hiddenCount * nn->hiddenCount * (nn->hiddenLayerCount - 1) + nn->hiddenCount * nn->outputCount;
    return weightsOffset + hiddenLayer * nn->hiddenCount + hiddenIndex;
}

int OutputNeuronBias(__global NeuralNetwork* nn, int outputIndex, bool* oc) 
{
#ifdef DEBUG_MODE
    if (outputIndex < 0 || outputIndex >= nn->outputCount)
    {
        if (*oc)
        {
            printf("[OpenCL][ERROR] outputIndex is out of range. (OutputNeuronBias)\n");
            printf("outputIndex = %i range = [0, %i>\n", outputIndex, nn->outputCount);

            *oc = false;
        }
    }
#endif

    int weightsOffset = nn->inputCount * nn->hiddenCount + nn->hiddenCount * nn->hiddenCount * (nn->hiddenLayerCount - 1) + nn->hiddenCount * nn->outputCount;
    return weightsOffset + nn->hiddenLayerCount * nn->hiddenCount + outputIndex;
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

inline float ReLU(float x)
{
    return max(0.0f, x);
}

inline float DevReLU(float x)
{
    return clamp(x * 999999.9f, 0.0f, 1.0f);
}

float Sigmoid(float x)
{
    //return x / (1.0f + fabs(x));

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
#ifdef RELU
    return ReLU(x);
#else
    return Sigmoid(x);
#endif
}

float DevActivation(float x)
{
#ifdef RELU
    return DevReLU(x);
#else
    return DevSigmoid(x);
#endif
}

void Forward(bool* oc, __global NeuralNetwork* nn, __global float* in_weights, __local float* cache)
{
    // Input -> Hidden
    for (int i = 0; i < nn->hiddenCount; i++)
    {
        float sum = 0.0f;
        for (int j = 0; j < nn->inputCount; j++)
        {
            sum += cache[InputNeuron(nn, j, oc)] * in_weights[InputHiddenNeuronWeight(nn, j, i, oc)];
        }
        sum += in_weights[HiddenNeuronBias(nn, i, 0, oc)];
        cache[HiddenNeuron(nn, i, 0, oc)] = Activation(sum);
    }

    // Hidden -> Hidden
    for (int l = 0; l < nn->hiddenLayerCount - 1; l++)
    {
        for (int i = 0; i < nn->hiddenCount; i++)
        {
            float sum = 0.0f;
            for (int j = 0; j < nn->hiddenCount; j++)
            {
                sum += cache[HiddenNeuron(nn, j, l, oc)] * in_weights[HiddenHiddenNeuronWeight(nn, j, i, l, oc)];
            }
            sum += in_weights[HiddenNeuronBias(nn, i, l + 1, oc)];
            cache[HiddenNeuron(nn, i, l + 1, oc)] = Activation(sum);
        }
    }

    // Hidden -> Output
    int l = nn->hiddenLayerCount - 1;
    for (int i = 0; i < nn->outputCount; i++)
    {
        float sum = 0.0f;
        for (int j = 0; j < nn->hiddenCount; j++)
        {
            sum += cache[HiddenNeuron(nn, j, l, oc)] * in_weights[HiddenOutputNeuronWeight(nn, j, i, oc)];
        }
        sum += in_weights[OutputNeuronBias(nn, i, oc)];
        cache[OutputNeuron(nn, i, oc)] = Activation(sum);
    }
}

void Backpropagate(bool* oc, __global NeuralNetwork* nn, __global float* in_weights, __global float* out_weights, __local float* cache, float learningRate)
{
    // Calculate deltas
    {
        // Output deltas
        for (int i = 0; i < nn->outputCount; i++)
        {
            float error = cache[OutputNeuron(nn, i, oc)];
            cache[OutputNeuronDelta(nn, i, oc)] = DevActivation(error);
        }

        // Hidden deltas
        for (int l = nn->hiddenLayerCount - 1; l >= 0; l--)
        {
            for (int i = 0; i < nn->hiddenCount; i++)
            {
                float error = 0.0;
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
                cache[HiddenNeuronDelta(nn, i, l, oc)] = DevActivation(error);
            }
        }
    }

    // Adjust weights
    {
        // Input <- Hidden weights
        for (int i = 0; i < nn->inputCount; i++)
        {
            for (int j = 0; j < nn->hiddenCount; j++)
            {
                float delta = learningRate * cache[HiddenNeuronDelta(nn, j, 0, oc)] * cache[InputNeuron(nn, i, oc)];
                AtomicAddFloat(&out_weights[InputHiddenNeuronWeight(nn, i, j, oc)], delta);
            }
        }

        // Hidden <- Hidden weights
        for (int l = 0; l < nn->hiddenLayerCount - 1; l++)
        {
            for (int i = 0; i < nn->hiddenCount; i++)
            {
                for (int j = 0; j < nn->hiddenCount; j++)
                {
                    float delta = learningRate * cache[HiddenNeuronDelta(nn, j, l + 1, oc)] * cache[HiddenNeuron(nn, i, l, oc)];
                    AtomicAddFloat(&out_weights[HiddenHiddenNeuronWeight(nn, i, j, l, oc)], delta);
                }
            }
        }

        // Hidden <- Output weights
        for (int i = 0; i < nn->hiddenCount; i++)
        {
            for (int j = 0; j < nn->outputCount; j++)
            {
                float delta = learningRate * cache[OutputNeuronDelta(nn, j, oc)] * cache[HiddenNeuron(nn, i, nn->hiddenLayerCount - 1, oc)];
                AtomicAddFloat(&out_weights[HiddenOutputNeuronWeight(nn, i, j, oc)], delta);
            }
        }
    }

    // Adjust biases
    {
        // Hidden biases
        for (int l = 0; l < nn->hiddenLayerCount; l++)
        {
            for (int i = 0; i < nn->hiddenCount; i++)
            {
                float delta = learningRate * cache[HiddenNeuronDelta(nn, i, l, oc)];
                AtomicAddFloat(&out_weights[HiddenNeuronBias(nn, i, l, oc)], delta);
            }
        }

        // Output biases
        for (int i = 0; i < nn->outputCount; i++)
        {
            float delta = learningRate * cache[OutputNeuronDelta(nn, i, oc)];
            AtomicAddFloat(&out_weights[OutputNeuronBias(nn, i, oc)], delta);
        }
    }
}