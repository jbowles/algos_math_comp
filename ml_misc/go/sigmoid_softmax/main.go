package main

import (
	"fmt"
	"math"

	"gonum.org/v1/gonum/mat"
)

func runSigmoid() {
	res := sigmoid(7.0)
	expect := 0.9990889488055994
	fmt.Printf("Sigmoid %t. expect '%g', got '%g'\n", res == expect, expect, res)
}

/*
	in numpy
	1.0 / (1.0 + math.exp(-x))

*/
func sigmoid(x float64) float64 {
	return 1.0 / (1.0 + math.Exp(-x))
}

/*
	in numpy
	np.exp(x) / np.sum(np.exp(x), axis=0)
*/
func softmax(x *mat.Dense) *mat.Dense {
	m := mat.DenseCopyOf(x)
	m.Apply(func(i, j int, v float64) float64 { return math.Exp(v) }, m)

	r, c := m.Dims()
	col := make([]float64, c)
	axis0 := make([]float64, c)
	for j := 0; j < c; j++ {
		var sum float64
		mat.Col(col, j, m)
		for _, v := range col {
			sum += v
		}
		axis0[j] = sum
	}

	for clm, v := range axis0 {
		for row := 0; row < r; row++ {
			set := m.At(row, clm)
			m.Set(row, clm, set/v)
		}
	}

	return m
}

func run_softmax() {
	// Initialize a matrix a with some data.
	a := mat.NewDense(4, 4, []float64{
		1, 2, 3, 6,
		2, 4, 5, 6,
		3, 8, 7, 6,
		3, 8, 7, 6,
	})
	compare := mat.NewDense(4, 4, []float64{
		0.05406459218899647, 0.001226622652767516, 0.008504460356397616, 0.25,
		0.14696279851039795, 0.009063583593518306, 0.06283993466455373, 0.25,
		0.3994863046503028, 0.4948548968768571, 0.4643278024895243, 0.25,
		0.3994863046503028, 0.4948548968768571, 0.4643278024895243, 0.25,
	})

	res := softmax(a)

	fm := mat.Formatted(res, mat.Prefix("    "), mat.Squeeze())
	fmt.Printf("r = %.3f\n\n", fm)
	fc := mat.Formatted(compare, mat.Prefix("    "), mat.Squeeze())
	fmt.Printf("c = %.3f\n", fc)
}

func main() {
	runSigmoid()
	run_softmax()
}

/*
func softmax_debug(x *mat.Dense) mat.Dense {
	fx := mat.Formatted(x, mat.Prefix("    "), mat.Squeeze())
	fmt.Printf("\n\nx = %.4f\n\n", fx)
	//var m mat.Dense
	//m.Exp(x)
	m := mat.DenseCopyOf(x)
	m.Apply(func(i, j int, v float64) float64 { return math.Exp(v) }, m)

	fm := mat.Formatted(m, mat.Prefix("    "), mat.Squeeze())
	fmt.Printf("\n\nfm = %.4f\n\n", fm)
	_, c := m.Dims()
	col := make([]float64, c)
	axis0 := make([]float64, c)
	for j := 0; j < c; j++ {
		var sum float64
		mat.Col(col, j, m)
		fmt.Printf("\n\nCol %d = %v\n\n", j, col)
		for _, v := range col {
			sum += v
		}
		axis0[j] = sum
	}
	fmt.Printf("\n\n axis0 = %v\n\n", axis0)
	//sumNorm := mat.Min(&n)
	//fmt.Printf("%f\n", sumNorm)
	//m.DivElem(&m, &n)
	return *m
}
*/
