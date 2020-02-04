package main

import (
	"bytes"
	"fmt"
	"strings"
)

func main() {
	tester()
}
func tester() {
	for key, val := range map[string]int{"I": 1, "II": 2, "III": 3, "IV": 4, "VI": 6, "IX": 9, "XI": 11, "CD": 400, "CM": 900, "XL": 40, "XC": 90, "MCMXCIV": 1994} {
		symbol := intToRoman(val)
		symbolTwo := intToRomanTwo(val)
		symbolThree := intToRomanThree(val)
		symbolFour := intToRomanFour(val)
		fmt.Printf("Test: %d \n%s [%s] %t\n", val, symbol, key, key == symbol)
		fmt.Printf("TestTwo: %d \n%s [%s] %t\n\n", val, symbolTwo, key, key == symbolTwo)
		fmt.Printf("TestThree: %d \n%s [%s] %t\n\n", val, symbolThree, key, key == symbolThree)
		fmt.Printf("TestFour: %d \n%s [%s] %t\n\n", val, symbolFour, key, key == symbolFour)
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

func intToRomanTwo(num int) string {
	table := [][]string{
		{"", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"},
		{"", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"},
		{"", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"},
		{"", "M", "MM", "MMM"},
	}
	return (table[3][num/1000] + table[2][(num%1000)/100] + table[1][(num%100)/10] + table[0][num%10])
}

func intToRomanThree(num int) string {
	buf := bytes.Buffer{}
	numerals := []int{1, 4, 5, 9, 10, 40, 50, 90, 100, 400, 500, 900, 1000}
	symbols := []string{"I", "IV", "V", "IX", "X", "XL", "L", "XC", "C", "CD", "D", "CM", "M"}

	i := len(numerals) - 1
	// fmt.Printf("START: %d\n", num)
	for num > 0 {
		// fmt.Printf("i==%d compare 'num(%d) >= numerals[i](%d)'\n", i, num, numerals[i])
		if num >= numerals[i] {
			repeat := num / numerals[i]
			// fmt.Printf("'r := num / numerals[i]' == %d\n", repeat)
			num %= numerals[i]
			// fmt.Printf("'num %%= numerals[i]' == %d\n", num)
			buf.WriteString(strings.Repeat(symbols[i], repeat))
			// fmt.Printf("'buf.WriteString(strings.Repeat(symbols[i], r))' == (%s, %d)\n\n", symbols[i], repeat)
		}
		i--
	}
	return buf.String()
}

func intToRomanFour(num int) string {
	var res string
	symbols := []string{"M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"}
	numerals := []int{1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1}

	for i := 0; num != 0; i++ {
		for num >= numerals[i] {
			num -= numerals[i]
			res += symbols[i]
		}
	}
	return res
}

/*
func intRoman(n int) []byte {
	switch n {
	case 0:
		return []byte{}
	//I
	case 1:
		return []byte{73}
	//IV
	case 4:
		return []byte{73, 86}
	//V
	case 5:
		return []byte{86}
	//IX
	case 9:
		return []byte{73, 88}
	//X
	case 10:
		return []byte{88}
	//XL
	case 40:
		return []byte{88, 76}
	//L
	case 50:
		return []byte{76}
	//LC
	case 90:
		return []byte{76, 67}
	//C
	case 100:
		return []byte{67}
	//CD
	case 400:
		return []byte{67, 68}
	//D
	case 500:
		return []byte{68}
	//CM
	case 900:
		return []byte{67, 77}
	//M //1000
	case 1000:
		return []byte{77}
	default:
		return []byte{}
	}
}

const (
	One = iota
	Ten
	Hundred
	Thousand
)

func intToRoman(n int) string {
	b := []byte{}
	revCut := cutReverse(n)
	for i, v := range revCut {
		fmt.Printf("i=%d, v=%d\n", i, v)
		b = append(b, intRoman(v)...)
	}

	return string(b)
}

//working backwards with count you'll know the place numbers
func cutReverse(n int) []int {
	places := []int{}
	for n > 0 {
		places = append(places, n%10)
		n /= 10
	}
	return places
}

func printCut() {
	fmt.Printf("%s \n", intRoman(4))
	// fmt.Printf("+ %s \n", intToRomanWrong(4))
	fmt.Printf("IV== %s \n", intToRoman(4))
	fmt.Printf("%s \n", intRoman(40))
	// fmt.Printf("+ %s \n", intToRomanWrong(40))
	fmt.Printf("XL== %s \n", intToRoman(40))
	fmt.Printf("%s \n", intRoman(400))
	// fmt.Printf("+ %s \n", intToRomanWrong(400))
	fmt.Printf("CD== %s \n", intToRoman(400))
	// fmt.Printf("%s \n", intRoman(9))
	// fmt.Printf("%s \n", intRoman(90))
	// fmt.Printf("%s \n", intRoman(900))
	// fmt.Printf("%s \n", intRoman(1000))
}

		{"", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"},
		{"", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"},
		{"", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"},
		{"", "M", "MM", "MMM"}


*/
