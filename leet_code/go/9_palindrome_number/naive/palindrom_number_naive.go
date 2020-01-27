package main

import "fmt"

func main() {
	in := []int{121, 11, 636, 99, 1, 666, 123321, -121, -1, -10, 10, -99, 82937483}
	for _, i := range in {
		fmt.Printf("%d is %v\n", i, isPalindrome(i))
	}

}

func isPalindrome(i int) bool {
	if i < 0 {
		return false
	}
	if i < 10 {
		return true
	}
	if i == reverse(i) {
		return true
	}
	return false
}

/*
func reverse(x int) int {
	var tmp int
	for x != 0 {
		pop := x % 10
		x /= 10
		tmp = tmp*10 + pop
	}
	return tmp
}
*/
func reverse(x int) int {
	var rev int
	for x !=0 {
		rev = (rev*10) + (x%10)
		x /= 10
	}
	return rev
}