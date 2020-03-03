package main

import "fmt"

func main() {
	fmt.Println(reverse(321))
	fmt.Println(reverse(3872))
}

func reverse(n int) int {
	var out int
	for n > 0 {
		out = (out * 10) + (n % 10)
		n /= 10
	}
	return out
}

/*
func reverse(n int) int {
	var rev int
	for n != 0 {
		rev = (rev * 10) + (n % 10)
		n /= 10
	}
	return rev
}

func reverse(x int) int {
	var rev int
	for x != 0 {
		rev = (rev * 10) + (x % 10)
		x /= 10
	}
	return rev
}

func reverse(x int) int {
	var rev int
	for x != 0 {
		rev = (rev * 10) + (x % 10)
		x /= 10
	}

	return rev
}
*/
