package main

import (
	"fmt"
	"strings"
)

/*
Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

Symbol       Value
I             1
V             5
X             10
L             50
C             100
D             500
M             1000

For example, two is written as II in Roman numeral, just two one's added together.
Twelve is written as, XII, which is simply X + II. The number twenty seven is written as XXVII, which is XX + V + II.

Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII.
Instead, the number four is written as IV. Because the one is before the five we subtract it making four.
The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:

    I can be placed before V (5) and X (10) to make 4 and 9.
    X can be placed before L (50) and C (100) to make 40 and 90.
    C can be placed before D (500) and M (1000) to make 400 and 900.

Given a roman numeral, convert it to an integer. Input is guaranteed to be within the range from 1 to 3999.
*/
func main() {
	for key, val := range map[string]int{"I": 1, "II": 2, "III": 3, "IV": 4, "VI": 6, "IX": 9, "XI": 11} {
		score := naiveScore(key)
		fmt.Printf("Test: %s == %d [%d] %t\n\n", key, score, val, val == score)
	}
}

func naiveScore(val string) int {
	result := 0
	for _, v := range strings.Split(val, "") {
		// fmt.Printf("\t  index: %d value: %s result == %d\n", i, v, romanInt(v))
		result += romanInt(v)
	}
	return result
}

/*
I             1     -> * 5
V             5     -> * 2
X             10    -> * 5
L             50    -> * 2
C             100   -> * 5
D             500   -> * 2
M             1000
*/
func romanInt(input string) int {
	switch input {
	case "I":
		return 1
	case "V":
		return 5
	case "X":
		return 10
	case "L":
		return 50
	case "C":
		return 100
	case "D":
		return 500
	default:
		return 1000
	}
}
