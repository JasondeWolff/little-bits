typedef struct _NeuralNetwork
{
    int inputCount;
    int hiddenCount;
    int outputCount;
    int hiddenLayerCount;
} NeuralNetwork;

int InputHiddenNeuronWeight(__global NeuralNetwork* nn, int inputIndex, int hiddenIndex)
{
    return hiddenIndex + inputIndex * nn->hiddenCount;
}

int HiddenHiddenNeuronWeight(__global NeuralNetwork* nn, int hiddenIndex, int nextHiddenIndex, int hiddenLayerIndex)
{
    int inputWeightsOffset = nn->inputCount * nn->hiddenCount;
    int previousHiddenWeightsOffset = hiddenLayerIndex * nn->hiddenCount * nn->hiddenCount;
    return inputWeightsOffset + previousHiddenWeightsOffset + nextHiddenIndex + hiddenIndex * nn->hiddenCount;
}

int HiddenOutputNeuronWeight(__global NeuralNetwork* nn, int hiddenIndex, int outputIndex)
{
    int inputWeightsOffset = nn->inputCount * nn->hiddenCount;
    int hiddenWeightsOffset = nn->hiddenCount * nn->hiddenCount * nn->hiddenLayerCount;
    return inputWeightsOffset + hiddenWeightsOffset + outputIndex + hiddenIndex * nn->outputCount;
}

int InputNeuron(__global NeuralNetwork* nn, int inputIndex) 
{
    return inputIndex;
}

int HiddenNeuron(__global NeuralNetwork* nn, int hiddenIndex, int hiddenLayer) 
{
    return nn->inputCount + hiddenLayer * nn->hiddenCount + hiddenIndex;
}

int OutputNeuron(__global NeuralNetwork* nn, int outputIndex) 
{
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

void Forward(__global NeuralNetwork* nn, __global float* weights, __local float* cache)
{
    // Input -> Hidden
    for (int i = 0; i < nn->hiddenCount; i++)
    {
        float sum = 0.0f;
        for (int j = 0; j < nn->inputCount; j++)
        {
            sum += cache[InputNeuron(nn, j)] * weights[InputHiddenNeuronWeight(nn, j, i)];
        }
        cache[HiddenNeuron(nn, i, 0)] = Activation(sum);
    }

    // Hidden -> Hidden
    for (int l = 0; l < nn->hiddenLayerCount - 1; l++)
    {
        for (int i = 0; i < nn->hiddenCount; i++)
        {
            float sum = 0.0f;
            for (int j = 0; j < nn->hiddenCount; j++)
            {
                sum += cache[HiddenNeuron(nn, j, l)] * weights[HiddenHiddenNeuronWeight(nn, j, i, l)];
            }
            cache[HiddenNeuron(nn, i, l + 1)] = Activation(sum);
        }
    }

    // Hidden -> Output
    int l = nn->hiddenLayerCount - 1;
    for (int i = 0; i < nn->outputCount; i++)
    {
        float sum = 0.0f;
        for (int j = 0; j < nn->hiddenCount; j++)
        {
            sum += cache[HiddenNeuron(nn, j, l)] * weights[HiddenOutputNeuronWeight(nn, j, i)];
        }
        cache[OutputNeuron(nn, i)] = Activation(sum);
    }
}

void Backpropagate(__global NeuralNetwork* nn, __global float* in_weights, __global float* out_weights, __local float* cache, float learningRate)
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

    for (int j = 0; j < nn->hiddenCount; j++)
    {
        float aL1 = cache[HiddenNeuron(nn, j, l)];
        
        float sum = 0.0f;
        for (int i = 0; i < nn->outputCount; i++)
        {
            float error = 2.0f * cache[OutputNeuron(nn, i)];

            float wL = in_weights[HiddenOutputNeuronWeight(nn, j, i)];
            float actDev = DevActivation(wL * aL1); // + bL

            float influence = error * actDev * aL1;
            float delta = influence * learningRate;

            AtomicAddFloat(&out_weights[HiddenOutputNeuronWeight(nn, j, i)], delta);

            sum += error * in_weights[HiddenOutputNeuronWeight(nn, j, i)];
        }

        //cache[HiddenNeuron(nn, j, l)] = sum;
    }
}