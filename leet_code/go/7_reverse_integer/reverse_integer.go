package main

import (
	"fmt"
	"math"
)

func main() {
	fmt.Printf("math.MaxInt32 == %d\n", math.MaxInt32)
	fmt.Printf("math.MinInt32 == %d\n", math.MinInt32)
	fmt.Printf("math.MaxInt32/10 == %d\n", math.MaxInt32/10)
	fmt.Printf("math.MinInt32/10 == %d\n", math.MinInt32/10)
	fmt.Println(reverse(123))         //321
	fmt.Println(reverse(-123))        //-321
	fmt.Println(reverse(120))         //21
	fmt.Println(reverse(1534236469))  //should be 0
	fmt.Println(reverse(-2147483648)) //should be 0

	fmt.Println("reverseTwo(123)", reverseTwo(123))                 //321
	fmt.Println("reverseTwo(-123)", reverseTwo(-123))               //-321
	fmt.Println("reverseTwo(120)", reverseTwo(120))                 //21
	fmt.Println("reverseTwo(1534236469)", reverseTwo(1534236469))   //should be 0
	fmt.Println("reverseTwo(-2147483648)", reverseTwo(-2147483648)) //should be 0
}

// Time:  O(log(x)) ... roughly log_10(x) digits in x
// Space: O(1)
func reverse(x int) int {
	var rev int
	for x != 0 {
		pop := x % 10 //modulo 10
		x /= 10
		rev = rev*10 + pop
	}

	if (rev > math.MaxInt32) || (rev < math.MinInt32) {
		return 0
	}
	return rev
}

func reverseTwo(x int) int {
	var rev int
	for x != 0 {
		pop := x % 10 //modulo 10
		x /= 10
		if (rev > math.MaxInt32/10) || ((rev == math.MaxInt32/10) && pop > 7) {
			return 0
		}
		if (rev < math.MinInt32/10) || ((rev == math.MinInt32/10) && pop < -8) {
			return 0
		}
		rev = rev*10 + pop
	}

	return rev
}

/*
fmt.Printf("%v \n", (x %= 10)) // *=, /=, -=, +=
//syntax error: unexpected %=, expecting )
//syntax error: unexpected *=, expecting comma or )
//...
*/
