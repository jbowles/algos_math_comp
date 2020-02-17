#include <iostream>
short parity_naive(unsigned long long x)
{
    short result = 0;
    while (x)
    {
        result ^= (x & 1);
        x >>= 1;
    }
    return result;
}
short parity(unsigned long long x)
{
    short result = 0;
    while (x)
    {
        result ^= 1;
        x &= (x - 1);
    }
    return result;
}

int main()
{
    printf("parity_naive(4) == %d\n", parity_naive(4));                           //1
    printf("parity_naive(183) == %d\n", parity_naive(183));                       //0
    printf("parity_naive(12347389723921) == %d\n", parity_naive(12347389723921)); //0

    printf("parity(4) == %d\n", parity(4));                           //1
    printf("parity(183) == %d\n", parity(183));                       //0
    printf("parity(12347389723921) == %d\n", parity(12347389723921)); //0
    printf("parity(4789) == %d\n", parity(4789));                     //1
    return 0;
}