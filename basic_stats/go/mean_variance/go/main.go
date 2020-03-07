package main

import (
	"fmt"
	"math"
)

//THESE ARE GLOBAL
var (
	dist = []float64{1.0, 2.0, 3.0, 4.0, 5.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0}
	pmf  = FREQint32{1: 0.125, 4: 0.125, 2: 0.125, 3: 0.25, 5: 0.125, 6: 0.25}
)

func main() {
	fmt.Printf("for %v\n", dist)
	fmt.Printf("...testFreq()...\n")
	testFreq()
	fmt.Printf("...testMean()...\n")
	testMean(dist)
	fmt.Printf("...testSampleVariance()...\n")
	testSampleVariance(dist)
	fmt.Printf("...\n")
	fmt.Printf("for pmf %v\n", pmf)
	fmt.Printf("...testPMFMean()...\n")
	testPMFMean(pmf)
}

func testMean(d []float64) {
	fmt.Printf("mean[4.3636] == %f\n", mean(d))
}

func testSampleVariance(d []float64) {
	fmt.Printf("sample variance[4.0495] == %f\n", sample_variance(d))
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

func testFreq() {
	p := []int32{1, 4, 2, 3, 3, 5, 6, 6}
	fmt.Printf("for p %v\n", p)
	res := freq(p)
	fmt.Printf("freq() %v\n", res)
}

type FREQint32 map[int32]float64

func freq(s []int32) FREQint32 {
	res := make(FREQint32)
	for _, v := range s {
		if _, ok := res[v]; ok {
			res[v] += 1.0
		} else {
			res[v] = 1.0
		}
	}
	return res
}

/////////////////////////assume we have a proper PMF
func testPMFMean(ppmf FREQint32) {
	fmt.Printf("ppmf.sum()[1.0] == %v\n", ppmf.sum())
	fmt.Printf("ppmf.mean()[3.75] == %v\n", ppmf.mean())
}

func (f FREQint32) sum() float64 {
	var sum float64
	for _, v := range f {
		sum += v
	}
	return sum
}

func (f FREQint32) mean() float64 {
	var mean float64
	for k, v := range f {
		mean += float64(k) * v
	}
	return mean
}
