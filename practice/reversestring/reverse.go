package main

import "fmt"

func main() {
	fmt.Println(reverse("hhelllo078"))
}

func reverse(s string) string {
	rs := []rune(s)
	for l, r := 0, len(rs)-1; l < r; l, r = l+1, r-1 {
		rs[l], rs[r] = rs[r], rs[l]
	}
	return string(rs)
}

/*























func reverse(s string) string {
	rs := []rune(s)
	for l, r := 0, len(rs)-1; l < r; l, r = l+1, r-1 {
		rs[l], rs[r] = rs[r], rs[l]
	}
	return string(rs)
}

// O(n/2)
func reverse(s string) string {
	fmt.Println("size: ", len(s))
	rs := []rune(s)
	c := 0
	for l, r := 0, len(rs)-1; l < r; l, r = l+1, r-1 {
		fmt.Printf("l=%d, r=%d\n", l, r)
		rs[l], rs[r] = rs[r], rs[l]
		c++
	}
	fmt.Println("ops: ", c)
	return string(rs)
}
*/
