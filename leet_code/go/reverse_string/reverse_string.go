package main

import (
	"fmt"
	"strings"
)

func main() {
	s := "hello"
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
