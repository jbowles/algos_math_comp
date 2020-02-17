package main

import (
	"fmt"
)

func main() {
	t1 := []int{2, 3, 5, 5, 7, 11, 11, 11, 13}
	fmt.Printf("valid element count=6[%d]\n", dedupSorted(t1))
}

//O(n) time
//O(1) space
func dedupSorted(n []int) int {
	ll := len(n)
	if ll == 0 {
		return 0
	}
	w := 1
	for i := 1; i < ll; i++ {
		if n[w-1] != n[i] {
			n[w] = n[i]
			w++
		}
	}
	fmt.Printf("n  =%v\n", n)
	return w
}

/*
func del(a []int, i int) []int {
	return append(a[:i], a[i+1:]...)
}
*/
