#include "math.h"
#include "stdio.h"
#include "stdlib.h"

// the number of points in original data set
#define BASE_RES 10

// the order of the polynomial we are interpolating
//#define ORDER 2
// increase the order to grow randomness
#define ORDER 4

//this struct will contain the output of interpolation process
typedef struct result
{
    float *xs; //array of floats for x points
    float *ys; //array of floats for y points
} result;

// given two sets of (x,y) points and a new x val,
// return the linear interpolated y value
// y = mx+b   == rise / run
// rise == (y1 - y0)
// run == (x1 - x0)
// we use an offset y0 + (x - x0) since we trying to find value as this x point
float interp(float *xs, float *ys, float x, int order)
{
    //first lagrange polynomial
    float y = 0, tmp;
    for (int i = 0; i < order; i++)
    {
        tmp = ys[i];
        for (int j = 0; j < order; j++)
        {
            if (i == j)
                continue;
            tmp *= (x - xs[j]) / (xs[i] - xs[j]);
        }
        y += tmp;
    }
    return y;
}

result interpolate(float *xs, float *ys, int n, int order)
{
    float delta;
    result res;
    res.xs = malloc(10 * (n) * sizeof(float));
    res.ys = malloc(10 * (n) * sizeof(float));
    for (int i = 0; i < n - 1; i++) //loop n-1 times
    {
        for (int j = 0; j < 10; j++)
        {
            delta = xs[i + 1] - xs[i];
            res.xs[10 * i + j] = xs[i] + 0.1 * ((float)j) * delta;
            // interpolate 10 points between x-y
            if (i < n - order)
            {
                res.ys[10 * i + j] = interp(xs + i, ys + i, res.xs[10 * i + j], order);
            }
            else
            {
                res.ys[10 * i + j] = interp(xs + n - order, ys + n - order, res.xs[10 * i + j], order);
            }
        }
    }
    return res;
}

//gcc polynomial_interpolation.c -lm -o poly_interp
int main()
{
    //generate input data (simple sine curve)
    float xs[BASE_RES];
    float ys[BASE_RES];
    for (int i = 0; i < BASE_RES; i++)
    {
        xs[i] = 4. * M_PI * ((float)i) / BASE_RES;
        ys[i] = sin(xs[i]);
    }
    result res = interpolate(xs, ys, BASE_RES, ORDER + 1);
    //interpolate to 10x higher resolution
    for (int i = 0; i < 10 * (BASE_RES - 1); i++)
    {
        printf("%e %e\n", res.xs[i], res.ys[i]);
    }
    return 0;
}
