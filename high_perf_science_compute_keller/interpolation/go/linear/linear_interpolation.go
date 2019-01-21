package main

import (
	"fmt"
	"math"
)

const (
	// number of points in original dataset
	BASE_RES = 10
)

//contians results of the interpolation process
type result struct {
	xs []float64
	ys []float64
}

//interp finds slope with an offset multiplier to find value at x
func interp(y0, y1, x0, x1, x float64) float64 {
	return y0 + (x-x0)*(y1-y0)/(x1-x0)
}

/* slope finds the ratio of a ratio of vertical line.
http://mathworld.wolfram.com/Slope.html
Slope:
	(1). 	m(x) = (dy)/(dx)
	(2). 	m = (delta(y))/(delta(x)) = tan(theta)
	y = mx + b
	m = delta(y2,y1) / delta(x2,x1)
	etc...
Also known as rise-over-run, finds the direction and steepness of a line.
*/
func slope(y0, y1, x0, x1 float64) float64 {
	return (y1 - y0) / (x1 - x0)
}
func offset(y0, x0, x float64) float64 {
	return y0 + (x - x0)
}

/* interpV2 NOT ACCURATE takes two sets of (x,y) points and a new value x,
returns the linear interpolated value y.

Note: this _should_  be the less performant way but... SEE benchmarks!!
Note:
	we use offset since we trying to find value as this x point
*/
func interpV2(y0, y1, x0, x1, x float64) float64 {
	return offset(y0, x0, x) * slope(y0, y1, x0, x1)
}

/* interpolate for result, with n number of initial values and x,y coordinates to fit.

allocate result x,y slices 10 * number of intial size -1
loop initial value size
loop interploated value size
set index to 10 * initial value + interpolated value
set delta to initial value X+1 - X (e.g, 1)
set result X initial value X + 0.1 * interoplated value * delta (0 + 0.1 + 0 * 1 == 1.1)
set result Y interp()
*/
func interpolate(xFit, yFit []float64, n int) result {
	//allocate slice size and cap of array on the stack
	scap := 10 * (n - 1)
	res := result{
		xs: make([]float64, scap),
		ys: make([]float64, scap),
	}
	//fmt.Printf("res.xs: %d, res.ys: %d\n", len(res.xs), len(res.ys))
	for i := 0; i < (n - 1); i++ {
		for j := 0; j < 10; j++ {
			delta := xFit[i+1] - xFit[i]
			idx := (10 * i) + j
			//fmt.Printf("i: %d, j: %d, index: %d, delta: %f\n", i, j, idx, delta)
			res.xs[idx] = xFit[i] + 0.1*float64(j)*delta
			res.ys[idx] = interp(yFit[i], yFit[i+1], xFit[i], xFit[i+1], res.xs[idx])
		}
	}
	return res
}

/*
	go build linear_interpolation.go
	./linear_interpolation > linplot_out
	./plot_interp.py go/linear/linplot_out

*/
func main() {
	xs := make([]float64, BASE_RES)
	ys := make([]float64, BASE_RES)
	for i := 0; i < BASE_RES; i++ {
		xs[i] = 4.0 * math.Pi * float64(i) / float64(BASE_RES)
		ys[i] = math.Sin(xs[i])
	}
	res := interpolate(xs, ys, BASE_RES)

	for i := 0; i < 10*(BASE_RES-1); i++ {
		fmt.Printf("%e %e\n", res.xs[i], res.ys[i])
	}
}
