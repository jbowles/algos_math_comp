#include <iostream>
#include <cmath>

// using std::cin;
// using std::cout;
// using std::endl;

void bitwops()
{
    int x1 = (6 & 4);   //4
    int x2 = (1 | 2);   //3
    int x3 = (8 >> 2);  //2
    int x4 = (1 << 10); //1024
    int x5 = (~1);      //-2
    int x6 = (15 ^ 3);  //12
    printf("bitwops():\n %d\n %d\n %d\n %d\n %d\n %d\n", x1, x2, x3, x4, x5, x6);
}

void nulim()
{
    int imin = std::numeric_limits<int>::min();
    int imax = std::numeric_limits<int>::max();
    int iinf = std::numeric_limits<int>::infinity();

    float fmin = std::numeric_limits<float>::min();
    float fmax = std::numeric_limits<float>::max();
    float finf = std::numeric_limits<float>::infinity();

    printf("nulim(<int>): imin: %d, imax: %d, iinf: %d\n", imin, imax, iinf);
    printf("nulim(<float>): fmin: %f, fmax: %f, finf: %f\n", fmin, fmax, finf);
}

void cmathops()
{

    printf(
        "cmathops():\n abs(-34)=%d\n fabs(-3.14)=%f\n floor(3.14)=%f\n ceil(3.14)=%f\n",
        abs(-34),
        fabs(-3.14),
        floor(3.14),
        ceil(3.14));
}

int main()
{
    // bitwops();
    // nulim();
    cmathops();
    return 0;
}