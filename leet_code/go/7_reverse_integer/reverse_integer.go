package main

import (
	"fmt"
	"math"
)
/*
Given a 32-bit signed integer, reverse digits of an integer.

Example 1:

Input: 123
Output: 321

Example 2:

Input: -123
Output: -321

Example 3:

Input: 120
Output: 21

Note:
Assume we are dealing with an environment which could only store integers 
within the 32-bit signed integer range: [−231,  231 − 1]. 
For the purpose of this problem, assume that your function returns 0 when 
the reversed integer overflows.
*/

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

	fmt.Println("reverseThree(123)", reverseThree(123))                 //321
	fmt.Println("reverseThree(-123)", reverseThree(-123))               //-321
	fmt.Println("reverseThree(120)", reverseThree(120))                 //21
	fmt.Println("reverseThree(1534236469)", reverseThree(1534236469))   //should be 0
	fmt.Println("reverseThree(-2147483648)", reverseThree(-2147483648)) //should be 0
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


/*
handles (over/under)flow:
	1. If temp=rev⋅10+poptemp = \text{rev} \cdot 10 + \text{pop}temp=rev⋅10+pop causes overflow, then it must be that rev≥INTMAX10\text{rev} \geq \frac{INTMAX}{10}rev≥10INTMAX​
	2. If rev>INTMAX10\text{rev} > \frac{INTMAX}{10}rev>10INTMAX​, then temp=rev⋅10+poptemp = \text{rev} \cdot 10 + \text{pop}temp=rev⋅10+pop is guaranteed to overflow.
	3. If rev==INTMAX10\text{rev} == \frac{INTMAX}{10}rev==10INTMAX​, then temp=rev⋅10+poptemp = \text{rev} \cdot 10 + \text{pop}temp=rev⋅10+pop will overflow if and only if pop>7\text{pop} > 7pop>7

Time:  O(log(x))
Space: O(1)
*/
func reverseTwo(x int) int {
	var rev int
	for x != 0 {
		pop := x % 10 //modulo 10
		x /= 10 //extend back to 10ths..
		//check for overflow
		if (rev > math.MaxInt32/10) || ((rev == math.MaxInt32/10) && pop > 7) {
			return 0
		}
		//check for underflow
		if (rev < math.MinInt32/10) || ((rev == math.MinInt32/10) && pop < -8) {
			return 0
		}
		rev = rev*10 + pop
	}

	return rev
}

/*
-- Since we've divided input by 10 then multiplying the reversed number by 10 extends
	the value back to the digit place of the original number
-- Adding the last digit of original number to multiplied reversed number gives it
	the new tens place
*/
func reverseThree(x int) int {
	var rev int
	for x != 0 {
		rev = (rev * 10) + (x % 10)
		x /= 10
	}

	if (rev > math.MaxInt32) || (rev < math.MinInt32) {
		return 0
	}
	return rev
}
