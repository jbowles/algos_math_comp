package main

import (
	"fmt"
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
	for key, val := range map[string]int{"I": 1, "II": 2, "III": 3, "IV": 4, "VI": 6, "IX": 9, "XI": 11, "CD": 400, "CM": 900, "XL": 40, "XC": 90} {
		score := Score(key)
		fmt.Printf("Test: %s == %d [%d] %t\n\n", key, score, val, val == score)
	}
}

func Score(s string) int {
	result := 0
	for i := 0; i < len(s); i++ {
		//fmt.Printf("S=%s i=%d s[i]=%d, string(s[i])=%s, romanInt=%d\n", s, i, s[i], string(s[i]), romanInt(s[i]))
		result += romanInt(s[i])
	}

	for i := 0; i < len(s)-1; i++ {
		if romanInt(s[i]) < romanInt(s[i+1]) {
			// fmt.Printf("SHIFTBACK(%s) val[i]==%d  val[i+1]==%d\n", val, romanInt(string(val[i])), romanInt(string(val[i+1])))
			result -= 2 * romanInt(s[i])
			//result = subtractFront(string(s[i]), result)
		}
	}

	return result
}

/*
I,73             1     -> * 5
V,86             5     -> * 2
X,88             10    -> * 5
L,76            50    -> * 2
C,67             100   -> * 5
D,68             500   -> * 2
M,77            1000
*/
func romanInt(in byte) int {
	switch in {
	//I
	case 73:
		return 1
	//V
	case 86:
		return 5
	//X
	case 88:
		return 10
	//L
	case 76:
		return 50
	//C
	case 67:
		return 100
	//D
	case 68:
		return 500
	//M
	default:
		return 1000
	}
}

/*
func rInt(input string) int {
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

func subtractFront(val string, res int) int {
	switch val {
	case "I":
		return res - 1*2
	case "X":
		return res - 10*2
	case "C":
		return res - 100*2
	default:
		return res - 0
	}
}
*/
