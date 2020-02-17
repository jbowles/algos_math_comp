package main

import "fmt"

func main() {
	fmt.Printf("parity_naive(4[1]) == %d\n", parity_naive(4))     //1
	fmt.Printf("parity_naive(183[0]) == %d\n", parity_naive(183)) //0
	fmt.Println("------")
	fmt.Printf("parity(4[1]) == %d\n", parity(4))             //1
	fmt.Printf("parity183[0]) == %d\n", parity(183))          //0
	fmt.Printf("parity_erase(4789[1]) == %d\n", parity(4789)) //1
}

func parity_naive(x int) int {
	result := 0
	for x > 0 {
		fmt.Printf("x = %v\n", x)
		result ^= (x & 1)
		// fmt.Printf("result = %v\n", result)
		x >>= 1
	}
	return result
}

func parity(x int) int {
	var result int
	for x > 0 {
		result ^= 1
		x &= (x - 1)
	}
	return result
}

/*
func parity_erase(x int) int {
	result := 0
	for x > 0 {
		fmt.Printf("x = %v\n", x)
		result ^= 1
		// fmt.Printf("result = %v\n", result)
		x &= (x - 1)
	}
	return result
}
*/
