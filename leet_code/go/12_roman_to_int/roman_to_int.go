package main

import (
	"fmt"
)

func main() {
	tester()
}

func tester() {
	for key, val := range map[string]int{"I": 1, "II": 2, "III": 3, "IV": 4, "VI": 6, "IX": 9, "XI": 11, "CD": 400, "CM": 900, "XL": 40, "XC": 90, "MCMXCIV": 1994} {
		symbol := intToRoman(val)
		symbolTwo := intToRomanTwo(val)
		fmt.Printf("Test: %d \n%s [%s] %t\n", val, symbol, key, key == symbol)
		fmt.Printf("TestTwo: %d \n%s [%s] %t\n\n", val, symbolTwo, key, key == symbolTwo)
	}
}

func intToRoman(num int) string {
	res := ""
	table := [][]string{
		{"", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"},
		{"", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"},
		{"", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"},
		{"", "M", "MM", "MMM"},
	}
	counter := 0
	for num > 0 {
		t := num % 10
		res = table[counter][t] + res
		num /= 10
		counter++
	}

	return res
}
