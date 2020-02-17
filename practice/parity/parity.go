package main

import "fmt"

func main() {
	fmt.Printf("parity(4) == %d\n", parity(4))       //1
	fmt.Printf("parity(183) == %d\n", parity(183))   //0
	fmt.Printf("parity(4789) == %d\n", parity(4789)) //1

	showOps()
}

func parity(n int) int {
	c := 0
	for n > 0 {
		c ^= 1
		n &= (n - 1)
	}
	return c
}

/*
// O(n)
func parity(x int) int {
	count := 0
	for x > 0 {
		count ^= 1 //xor operator
		x &= (x - 1)
	}
	return count
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
*/

func showOps() {
	var a uint = 60 /* 60 = 0011 1100 */
	var b uint = 13 /* 13 = 0000 1101 */
	var c uint

	c = a & b /* 12 = 0000 1100 */
	fmt.Printf("'60&13' Line 1 - Value of c is %d\n", c)
	//Line 1 - Value of c is 12

	c = a | b /* 61 = 0011 1101 */
	fmt.Printf("'60|13' Line 2 - Value of c is %d\n", c)
	//Line 2 - Value of c is 61

	c = a ^ b /* 49 = 0011 0001 */
	fmt.Printf("'60^13' Line 3 - Value of c is %d\n", c)
	//Line 3 - Value of c is 49

	c = a << 2 /* 240 = 1111 0000 */
	fmt.Printf("'60<<2' Line 4 - Value of c is %d\n", c)
	//Line 4 - Value of c is 240

	c = a >> 2 /* 15 = 0000 1111 */
	fmt.Printf("'60>>2' Line 5 - Value of c is %d\n", c)
	//Line 5 - Value of c is 15
}

/*
Bitwise Operators
work on bits and perform bit-by-bit operation. The truth tables for &, |, and ^ are as follows −
	p 	q 	p & q 	p | q 	p ^ q
	0 	0 	  0 	  0 	  0
	0 	1 	  0 	  1 	  1
	1 	1 	  1 	  1 	  0
	1 	0 	  0 	  1 	  1

*/
