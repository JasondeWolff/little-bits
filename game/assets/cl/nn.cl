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
    if (hiddenIndex < 0 || hiddenIndex >= nn->hiddenCount || nextHiddenIndex < 0 || nextHiddenIndex >= nn->hiddenCount || hiddenLayerIndex < 0 || hiddenLayerIndex >= nn->hiddenLayerCount)
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
            if (hiddenLayerIndex < 0 || hiddenLayerIndex >= nn->hiddenLayerCount)
            {
                printf("hiddenLayerIndex = %i range = [0, %i>\n", hiddenLayerIndex, nn->hiddenLayerCount);
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
    int hiddenWeightsOffset = nn->hiddenCount * nn->hiddenCount * nn->hiddenLayerCount;
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

//#define RELU

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

void Forward(bool* oc, __global NeuralNetwork* nn, __global float* weights, __local float* cache)
{
    // Input -> Hidden
    for (int i = 0; i < nn->hiddenCount; i++)
    {
        float sum = 0.0f;
        for (int j = 0; j < nn->inputCount; j++)
        {
            sum += cache[InputNeuron(nn, j, oc)] * weights[InputHiddenNeuronWeight(nn, j, i, oc)];
        }
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
                sum += cache[HiddenNeuron(nn, j, l, oc)] * weights[HiddenHiddenNeuronWeight(nn, j, i, l, oc)];
            }
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
            sum += cache[HiddenNeuron(nn, j, l, oc)] * weights[HiddenOutputNeuronWeight(nn, j, i, oc)];
        }
        cache[OutputNeuron(nn, i, oc)] = Activation(sum);
    }
}

void Backpropagate(bool* oc, __global NeuralNetwork* nn, __global float* in_weights, __global float* out_weights, __local float* cache, float learningRate)
{
    int l = nn->hiddenLayerCount - 1;
    // for (int j = 0; j < nn->hiddenCount; j++)
    // {
    //     float neuronValue = cache[HiddenNeuron(nn, j, l)];
    //     neuronValue = DevActivation(neuronValue);
        
    //     float sum = 0.0f;
    //     for (int i = 0; i < nn->outputCount; i++)
    //     {
    //         float error = cache[OutputNeuron(nn, i)];

    //         float multiplied = neuronValue * error;
    //         float dot = multiplied * in_weights[HiddenOutputNeuronWeight(nn, j, i)];
    //         float delta = learningRate * dot;

    //         AtomicAddFloat(&out_weights[HiddenOutputNeuronWeight(nn, j, i)], delta);

    //         sum += error * in_weights[HiddenOutputNeuronWeight(nn, j, i)];
    //     }

    //     cache[HiddenNeuron(nn, j, l)] = sum;
    // }



    // for (int j = 0; j < nn->hiddenCount; j++)
    // {
    //     float aL1 = cache[HiddenNeuron(nn, j, l)];
        
    //     float sum = 0.0f;
    //     for (int i = 0; i < nn->outputCount; i++)
    //     {
    //         float error = 2.0f * cache[OutputNeuron(nn, i)];

    //         float wL = in_weights[HiddenOutputNeuronWeight(nn, j, i)];
    //         float actDev = DevActivation(wL * aL1); // + bL

    //         float influence = error * actDev * aL1;
    //         float delta = influence * learningRate;
    //         AtomicAddFloat(&out_weights[HiddenOutputNeuronWeight(nn, j, i)], delta);

    //         sum += error * in_weights[HiddenOutputNeuronWeight(nn, j, i)];
    //     }

    //     //cache[HiddenNeuron(nn, j, l)] = sum;
    // }


    for (int j = 0; j < nn->hiddenCount; j++)
    {
        float aL1 = cache[HiddenNeuron(nn, j, l, oc)];
        
        float influence = 0.0f;
        for (int i = 0; i < nn->outputCount; i++)
        {
            float error = 2.0f * cache[OutputNeuron(nn, i, oc)];

            float wL = in_weights[HiddenOutputNeuronWeight(nn, j, i, oc)];
            float actDev = DevActivation(wL * aL1); // + bL

            influence += error * actDev * aL1;
        }

        float delta = influence * learningRate;
        for (int i = 0; i < nn->outputCount; i++)
        {
            AtomicAddFloat(&out_weights[HiddenOutputNeuronWeight(nn, j, i, oc)], delta);
        }
    }
}