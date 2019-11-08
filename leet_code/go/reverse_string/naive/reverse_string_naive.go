package main

import (
	"bytes"
	"fmt"
	"strings"
)

func main() {
	fmt.Println("reverseNaive: ", reverseNaive("hello"))
	fmt.Println("reverseNaiveWithSB: ", reverseNaiveWithSB("hello"))
	fmt.Println("reverseNaiveAppend: ", reverseNaiveAppend("hello"))
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

//loop backwards and write to buffer
func reverseNaiveWithSB(s string) string {
	var b strings.Builder
	//reverse....
	for i := len(s) - 1; i >= 0; i-- {
		// fmt.Printf("%v \n", s[i])
		b.WriteString(string(s[i]))
	}

	return b.String()
}

// this still has to go through all n elements
func reverseNaiveAppend(s string) string {
	var a []byte
	l := len(s)
	for i := range s {
		a = append(a, s[l-(i+1)])
	}
	return string(a)
}
