package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

func main() {
	fmt.Println(reverseNaive(123))         //321
	fmt.Println(reverseNaive(-123))        //-321
	fmt.Println(reverseNaive(120))         //21
	fmt.Println(reverseNaive(1534236469))  //should be 0 got: 9646324351
	fmt.Println(reverseNaive(-2147483648)) //should be 0 got: -8463847412
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
