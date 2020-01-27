package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"
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
Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−231,  231 − 1]. For the purpose of this problem, assume that your function returns 0 when the reversed integer overflows.

*/


func main() {
	fmt.Println(reverseNaive(123))         //321
	fmt.Println(reverseNaive(-123))        //-321
	fmt.Println(reverseNaive(120))         //21
	fmt.Println(reverseNaive(1534236469))  //0
	fmt.Println(reverseNaive(-2147483648)) //0
}

//does not deal with signedness, so must handle manually
//does not deal with overflow, must handle manually
// time: O(n/2)
// space: ?
func reverseNaive(x int) int {
	s := strings.Split(strconv.Itoa(x), "")
	start := 0
	end := len(s) - 1
	for start < end {
		//is negative
		if s[start] == "-" {
			//skip it...
			start++
		}
		s[start], s[end] = s[end], s[start]
		start++
		end--
	}
	v, _ := strconv.Atoi(strings.Join(s, ""))
	if v > math.MaxInt32 || v < math.MinInt32 {
		return 0
	}
	return v
}
