package main

import "fmt"

func main() {
	a := []int{2, 7, 11, 15}
	fmt.Printf("twoSumNaive(9,a) %v\n", twoSumNaive(9, a))
	fmt.Printf("twoSum(9,a) %v\n", twoSum(9, a))

	b := []int{3, 2, 4}
	fmt.Printf("twoSumNaive(6,b) %v\n", twoSumNaive(6, b))
	fmt.Printf("twoSum(6,b) %v\n", twoSum(6, b))
}

/*
time:
space:
*/
func twoSum(t int, a []int) []int {
	m := make(map[int]int, len(a))
	for i := 0; i < len(a); i++ {
		if _, ok := m[t-a[i]]; ok {
			return []int{m[t-a[i]], i}
		}
		m[a[i]] = i
	}
	return a
}

// time: O(n^2); space: O(1)
func twoSumNaive(t int, a []int) []int {
	l := len(a)
	for i := 0; i < l; i++ {
		for j := 1; i < l; i++ {
			// fmt.Printf("i=%d, ii=%d; (%d + %d)\n", i, j, a[i], a[j])
			if a[j] == t-a[i] {
				return []int{i, j}
			}

		}
	}
	return []int{}
}
