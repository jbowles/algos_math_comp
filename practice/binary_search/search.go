package main

import (
	"fmt"
	"sort"
)

func main() {
	t := []int{1, 3, 6, 8, 10, 13, 16, 18, 14, 23, 26, 27, 29, 30, 33, 36, 38, 45, 48, 50}
	s := 33
	idx := bsearch(s, t)
	fmt.Printf("idx=%d, value=%d, CORRECT[%t]\n", idx, t[idx], t[idx] == s)

	t1 := []int{-14, -10, 2, 108, 108, 243, 285, 285, 401}
	s1 := 108
	idx1 := bsearch(s1, t1)
	fmt.Printf("idx1=%d, value1=%d, CORRECT[%t]\n", idx1, t1[idx1], t1[idx1] == s1)

	sort.Ints(t)
	idx2 := bisect(s, t)
	fmt.Printf("BISECT: idx2=%d, value=%d, CORRECT[%t]\n", idx2, t[idx], t[idx] == s)
}

// https://github.com/python/cpython/blob/3c88199e0be352c0813f145d7c4c83af044268aa/Lib/bisect.py
// bisect Return the index where to insert item x in list a, assuming a is sorte
func bisect(x int, xs []int) int {
	lo, hi := 0, len(xs)
	for lo < hi {
		mid := (lo + hi) / 2
		if x < xs[mid] {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
}

func bsearch(t int, a []int) int {
	lo, hi := 0, len(a)-1
	for lo <= hi {
		mid := lo + (hi-lo)/2
		if a[mid] < t {
			lo = mid + 1
		} else if a[mid] == t {
			return mid
		} else {
			hi = mid - 1
		}
	}
	return -1
}

/*
func bsearch(t int, a []int) int {
	var l, r = 0, len(a) - 1
	for l <= r {
		mid := l + (r-l)/2
		if a[mid] < t {
			l = mid + 1
		} else if a[mid] == t {
			return mid
		} else { //a[mid]>t
			r = mid - 1
		}
	}
	return -1
}
*/

/*
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
*/
