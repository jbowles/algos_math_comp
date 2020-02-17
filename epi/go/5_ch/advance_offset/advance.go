package main

import (
	"fmt"
)

func main() {
	t1 := []int{3, 3, 1, 0, 2, 0, 1} // true
	t2 := []int{3, 2, 0, 0, 2, 0, 1} // false
	t1a, t2a := canReachEnd(t1), canReachEnd(t2)
	fmt.Printf("t1[%t], CORRECT[%t]\n", t1a, t1a)
	fmt.Printf("t2[%t], CORRECT[%t]\n", t2a, !t2a)
}

func max(i, j int) int {
	if i > j {
		return i
	}
	return j
}

func canReachEnd(steps []int) bool {
	f := 0
	last := len(steps) - 2
	for i := 0; i <= f && f < last; i++ {
		f = max(f, steps[i]+i)
	}
	return f >= last
}
