package main

import (
	"bytes"
	"fmt"
	"strings"
)

func main() {
	fmt.Println("reverseNaive: ", reverseNaive("hello"))

	fmt.Println("reverseNaiveWithSB: ", reverseNaiveWithSB("hello"))
}

/*
	naive creates a new buffer,
	iterates in reverse the size of string,
	indexes each code point
	casts code point to rune/string
	writes to buffer
*/
func reverseNaive(s string) string {
	var b bytes.Buffer
	//reverse....
	for i := len(s) - 1; i >= 0; i-- {
		// fmt.Printf("%v \n", s[i])
		b.WriteString(string(s[i]))
	}

	return b.String()
}
func reverseNaiveWithSB(s string) string {
	var b strings.Builder
	//reverse....
	for i := len(s) - 1; i >= 0; i-- {
		// fmt.Printf("%v \n", s[i])
		b.WriteString(string(s[i]))
	}

	return b.String()
}
