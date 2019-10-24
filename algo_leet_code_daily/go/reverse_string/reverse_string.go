package main

import (
	"fmt"
	"strings"
)

func main() {
	s := "hello"
	fmt.Println("reverseOn: ", reverseOn(s))
	fmt.Println("reverse: ", reverse(s))
}

//shoudl be O(n/2)
func reverse(s string) string {
	a := strings.Split(s, "")
	start := 0
	end := len(a) - 1
	// fmt.Printf("%v \n", a)
	for start < end {
		a[start], a[end] = a[end], a[start]
		start++
		end--
	}
	return strings.Join(a, "")
}

// this still has to go through all n elements
func reverseOn(s string) string {
	var a []byte
	l := len(s)
	for i := range s {
		a = append(a, s[l-(i+1)])
	}
	return string(a)
}
