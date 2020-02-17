package main

import "fmt"

/*
go test -bench=
	BenchmarkRomanToIntTwo-8   	92835145	        11.1 ns/op
	BenchmarkRomanToInt-8      	 3802878	       319 ns/op
*/

func main() {
	for key, val := range map[string]int{"I": 1, "II": 2, "III": 3, "IV": 4, "VI": 6, "IX": 9, "XI": 11, "CD": 400, "CM": 900, "XL": 40, "XC": 90, "MCMXCIV": 1994} {
		score := romanToInt(key)
		scoreTwo := romanToIntTwo(key)
		scoreThree := romanToIntThree(key)
		fmt.Printf("Test:    	%s == %d [%d] %t\n", key, score, val, val == score)
		fmt.Printf("TestTwo: 	%s == %d [%d] %t\n", key, scoreTwo, val, val == scoreTwo)
		fmt.Printf("TestThree: 	%s == %d [%d] %t\n\n", key, scoreTwo, val, val == scoreThree)
	}
}
func romanToInt(s string) int {
	var m map[byte]int = map[byte]int{
		'I': 1,
		'V': 5,
		'X': 10,
		'L': 50,
		'C': 100,
		'D': 500,
		'M': 1000,
	}

	var total int
	for i := range s {
		if i == len(s)-1 {
			total += m[s[i]]
			break
		}
		// fmt.Printf("%d:%s, %d:%s\n", i, string(s[i]), i+1, string(s[i+1]))
		if m[s[i]] >= m[s[i+1]] {
			total += m[s[i]]
		} else {
			total -= m[s[i]]
			continue
		}
	}
	return total
}

func romanToIntTwo(s string) int {
	result := 0

	for i := 0; i < len(s); i++ {
		result += romanInt(s[i])
	}
	for i := 0; i < len(s)-1; i++ {
		if romanInt(s[i]) < romanInt(s[i+1]) {
			result -= 2 * romanInt(s[i])
		}
	}

	return result
}

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

func romanToIntThree(s string) int {
	var total int
	for i := range s {
		if i == len(s)-1 {
			total += romanMap()[s[i]]
			break
		}
		if romanMap()[s[i]] >= romanMap()[s[i+1]] {
			total += romanMap()[s[i]]
		} else {
			total -= romanMap()[s[i]]
			continue
		}
	}
	return total
}

func romanMap() map[byte]int {
	return map[byte]int{
		'I': 1,
		'V': 5,
		'X': 10,
		'L': 50,
		'C': 100,
		'D': 500,
		'M': 1000,
	}
}
