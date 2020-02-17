package main

import "fmt"

func main() {
	fmt.Println(reverse(321))
}

func reverse(x int) int {
	var rev int
	for x != 0 {
		rev = (rev * 10) + (x % 10)
		x /= 10
	}
	return rev
}

/*
func reverse(x int) int {
	var rev int
	for x != 0 {
		rev = (rev * 10) + (x % 10)
		x /= 10
	}

	return rev
}
*/
