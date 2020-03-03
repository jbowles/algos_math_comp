package main

import (
	"fmt"
)

func main() {
	t1 := []int{3, 3, 1, 0, 2, 0, 1} // true
	t2 := []int{3, 2, 0, 0, 2, 0, 1} // false
	t1a, t2a := canReachEnd(t1), canReachEnd(t2)
	fmt.Printf("true==t1[%t]\n", t1a)
	fmt.Printf("false==t2[%t]\n", t2a)
}

func max(i, j int) int {
	if i > j {
		return i
	}
	return j
}

func canReachEnd(ns []int) bool {
	var f int
	last := len(ns) - 2
	for i := 0; i <= f && f < last; i++ {
		f = max(f, ns[i]+i)
	}
	return f >= last
}

/*
//O(n) time, O(1) space
func canReachEnd(ns []int) bool {
	var f int
	last := len(ns) - 2
	for i := 0; i <= f && f < last; i++ {
		f = max(f, ns[i]+i)
	}
	return f >= last
}
*/
