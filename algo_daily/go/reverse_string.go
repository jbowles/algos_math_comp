package main

import (
	"fmt"
)

func main() {

	result := reverse("hello")
	fmt.Println(result)
	/*
		result := naive("hello")
		fmt.Println(result)

		resultSB := naiveWithSB("hello")
		fmt.Println(resultSB)
	*/
}

func reverse(s string) string {
	return s
}

/*
	naive creates a new buffer,
	iterates in reverse the size of string,
	indexes each code point
	casts code point to rune/string
	writes to buffer
*/
/*
func naive(s string) string {
	var b bytes.Buffer
	//reverse....
	for i := len(s) - 1; i >= 0; i-- {
		// fmt.Printf("%v \n", s[i])
		b.WriteString(string(s[i]))
	}

	return b.String()
}
func naiveWithSB(s string) string {
	var b strings.Builder
	//reverse....
	for i := len(s) - 1; i >= 0; i-- {
		// fmt.Printf("%v \n", s[i])
		b.WriteString(string(s[i]))
	}

	return b.String()
}
*/
