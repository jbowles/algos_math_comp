package main

import (
	"fmt"
	"math"
)

func main() {
	// t1 := []float64{310.0, 310.0, 275.0, 275.0, 260.0, 260.0, 260.0, 230.0, 230.0, 230.0}
	t1 := []float64{310, 315, 275, 295, 260, 270, 290, 230, 255, 250}
	// t1 := []float64{310.0, 275.0}
	fmt.Printf("profit=[%v]\n", sell(t1)) //30
}

// NOT WORKING...
func sell(prices []float64) float64 {
	minPrice, maxProfit := math.MaxFloat64, 0.0
	// fmt.Printf("minPrice:%v\n", minPrice) //32767
	for _, price := range prices {
		// fmt.Printf("price:%v\n", price)
		// fmt.Printf("IND:%d === price: %v, minPrice:%v diff=%v\n", i, price, minPrice, (price - minPrice)) //32767
		sell := (price - minPrice)
		maxProfit = math.Max(maxProfit, sell)
		// fmt.Printf("BIGGIES: minPrice=%v, price=%v %t\n\n", minPrice, price, minPrice < price)
		// fmt.Printf("sell:%v, minPrice=%v, maxProfit=%v\n\n", sell, minPrice, maxProfit)
		minPrice = math.Min(minPrice, price)
	}
	return maxProfit
}

/*
func max(i, j float64) float64 {
	if i > j {
		return i
	}
	return j
}
func min(i, j float64) float64 {
	if i < j {
		return i
	}
	return j
}
*/
