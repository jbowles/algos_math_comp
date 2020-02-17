package main

import "fmt"

// O(n)
func main() {
	var m [10]int
	var k int
	for k = 0; k < 10; k++ {
		m[k] = k * 200
		fmt.Printf("elem[%d]= %d\n", k, m[k])
	}
	fmt.Printf("m = %v\n", m)
}
