#include "math.h"
#include "stdio.h"
#include "stdlib.h"

// the number of points in original data set
#define BASE_RES 10

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
float interp(float y0, float y1, float x0, float x1, float x)
{
    //first lagrange polynomial
    return y0 + (x - x0) * (y1 - y0) / (x1 - x0);
}

result interpolate(float *xs, float *ys, int n)
{
    //unsigned long
    //printf("malloc: %lu\n\n", (10 * (n - 1) * sizeof(float)));
    //printf("sizeof(float): %lu\n\n", sizeof(float));
    float delta;
    result res;
    res.xs = malloc(10 * (n - 1) * sizeof(float));
    res.ys = malloc(10 * (n - 1) * sizeof(float));
    for (int i = 0; i < n - 1; i++) //loop n-1 times
    {
        for (int j = 0; j < 10; j++)
        {
            delta = xs[i + 1] - xs[i];
            //printf("   i:%d, j:%d;   10 * i + j == %d\n", i, j, 10 * i + j);
            res.xs[10 * i + j] = xs[i] + 0.1 * ((float)j) * delta;
            // interpolate 10 points between x-y
            res.ys[10 * i + j] = interp(ys[i], ys[i + 1], xs[i], xs[i + 1], res.xs[10 * i + j]);
        }
    }
    return res;
}

/*
 *  gcc linear_interpolation.c -lm -o linear
 *  ./linear > linplot_out
 *  ./plot_interp.py c/linplot_out
 */
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

    result res = interpolate(xs, ys, BASE_RES);
    //interpolate to 10x higher resolution
    for (int i = 0; i < 10 * (BASE_RES - 1); i++)
    {
        printf("%e %e\n", res.xs[i], res.ys[i]);
    }
    return 0;
}
