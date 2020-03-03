package main

import (
	"fmt"
	"math"
)

var (
	dist = []float64{1.0, 2.0, 3.0, 4.0, 5.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0}
)

func main() {
	fmt.Printf("for %v\n", dist)
	fmt.Println(" ")
	testMean()
	fmt.Println("...")
	testSampleVariance()
}

func testMean() {
	fmt.Printf("mean[4.3636] == %f\n", mean(dist))
}

func testSampleVariance() {
	fmt.Printf("sample variance[4.0495] == %f\n", sample_variance(dist))
}

/*
	julia: sum((p .- m).^2) / length(p)

	1/n Σᵢ(xᵢ - x̄)
*/
func sample_variance(s []float64) float64 {
	var sum float64
	xbar := mean(s)
	for _, xi := range s {
		sum += math.Pow(xi-xbar, 2)
		// sum += (xi - xbar) * (xi - xbar)
	}
	return (1.0 / float64(len(s))) * sum
	// return sum / float64(len(s))
}

/*
	1/n Σᵢxᵢ
*/
func mean(s []float64) float64 {
	var sum float64
	for _, v := range s {
		sum += v
	}
	return (1.0 / float64(len(s))) * sum
	// return sum / float64(len(s))
}
