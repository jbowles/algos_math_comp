package main

import (
	"fmt"
	"math"
)

const (
	//BASERES number of points in original dataset
	BASERES = 10
	//ORDER of polynomial, increase to grow randomness
	ORDER = 4
)

type result struct {
	xs []float64
	ys []float64
}

/*
	allocate y,tmp float
	loop through order i
	set tmp to ys[i] value
	loop through loop order j
	skip if i==j
	multiply tmp by x-xs[i], xs[i]-xs[j]
	increment y by tmp
	return y
*/
func interp(xs, ys []float64, x float64, order int) float64 {
	//var tmp float64
	//y := 0.0
	var y, tmp float64
	for i := 0; i < order; i++ {
		tmp = ys[i]
		for j := 0; j < order; j++ {
			if i == j {
				continue
			}
			tmp *= (x - xs[j]) / (xs[i] - xs[j])
		}
		y += tmp
	}
	return y
}

func interpolate(xs, ys []float64, n, order int) result {
	res := result{}
	scap := 10 * n //size and capacity
	res.xs = make([]float64, scap)
	res.ys = make([]float64, scap)
	for i := 0; i < n-1; i++ {

		for j := 0; j < 10; j++ {

			delta := xs[i+1] - xs[i]
			idx := 10*i + j
			res.xs[idx] = xs[i] + 0.1*float64(j)*delta
			if i < n-order {

				res.ys[idx] = interp(xs[i:], ys[i:], res.xs[idx], order)
			} else {
				res.ys[idx] = interp(xs[n-order:], ys[n-order:], res.xs[idx], order)
			}
		}
	}
	return res

}

func main() {
	xs := make([]float64, BASERES)
	ys := make([]float64, BASERES)
	for i := 0; i < BASERES; i++ {
		xs[i] = 4.0 * math.Pi * float64(i) / float64(BASERES)
		ys[i] = math.Sin(xs[i])
	}
	/*
		for _, i := range xs {
			fmt.Printf("%e \n", i)
		}
	*/

	res := interpolate(xs, ys, BASE_RES, ORDER+1)

	for i := 0; i < 10*(BASE_RES-1); i++ {
		fmt.Printf("%e %e\n", res.xs[i], res.ys[i])
	}
}
