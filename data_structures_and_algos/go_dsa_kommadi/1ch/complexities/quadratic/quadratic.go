package main

import "fmt"

/*
processing time proportional to the square of inputs
*/

// O(n^2)
func main() {
	var k, l int
	for k = 1; k <= 10; k++ {
		fmt.Printf("TableK: %v\n", k)
		for l = 1; l <= 10; l++ {
			x := l * k
			fmt.Printf("x=%d\n", x)
		}
	}
}
