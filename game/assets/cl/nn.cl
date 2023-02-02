typedef struct NeuralNetwork
{
    int inputCount;
    int hiddenCount;
    int outputCount;
    
    int hiddenLayerCount;

    float weights[];
};

int InputNeuron(__global struct NeuralNetwork* nn, int i)
{
    return i;
}

int HiddenNeuron(__global struct NeuralNetwork* nn, int i, int j)
{
    return nn->inputCount + nn->hiddenCount * j + i;
}

int OutputNeuron(__global struct NeuralNetwork* nn, int i)
{
    return nn->inputCount + nn->hiddenCount * nn->hiddenLayerCount + i;
}