package main

import (
	"fmt"
)

func main() {
	t := []int{1, 3, 6, 8, 10, 13, 16, 18, 14, 23, 26, 27, 29, 30, 33, 36, 38, 45, 48, 50}
	idx := bsearch(33, t)
	fmt.Printf("idx=%d, value=%d\n", idx, t[idx])

	t1 := []int{-14, -10, 2, 108, 108, 243, 285, 285, 401}
	idx1 := bsearch(108, t1)
	fmt.Printf("idx1=%d, value1=%d\n", idx1, t1[idx1])
}

// O(log n)
func bsearch(t int, a []int) int {
	var l, u = 0, len(a) - 1
	for l <= u {
		m := l + (u-l)/2
		if a[m] < t {
			l = m + 1
		} else if a[m] == t {
			return m
		} else {
			u = m - 1
		}
	}
	return -1
}
