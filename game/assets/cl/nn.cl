typedef struct _NeuralNetwork
{
    int inputCount;
    int hiddenCount;
    int outputCount;
    
    int hiddenLayerCount;

    float weights[];
} NeuralNetwork;

int InputNeuron(__global NeuralNetwork* nn, int i)
{
    return i;
}

int HiddenNeuron(__global NeuralNetwork* nn, int i, int j)
{
    return nn->inputCount + nn->hiddenCount * j + i;
}

int OutputNeuron(__global NeuralNetwork* nn, int i)
{
    return nn->inputCount + nn->hiddenCount * nn->hiddenLayerCount + i;
}