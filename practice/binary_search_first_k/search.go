package main

import (
	"fmt"
)

func main() {
	// fmt.Printf("9/2 == %d\n", 9/2)
	t1 := []int{-14, -10, 2, 108, 108, 243, 285, 285, 285, 401}
	idx1 := searchFirstOfK(t1, 108)
	fmt.Printf("idx1=%d, value1=%d\n", idx1, t1[idx1])
}

func searchFirstOfK(a []int, k int) int {
	var left, right, result = 0, len(a) - 1, -1
	for left <= right {
		//left=0; right=9
		// mid = 0 + (9-0)/2 ==
		if mid := left + (right-left)/2; a[mid] > k {
			right = mid - 1
		} else if a[mid] == k {
			result = mid
			right = mid - 1
		} else {
			left = mid + 1
		}
	}
	return result
}
