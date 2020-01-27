package main

import "fmt"

/*
Given an array of integers, return indices of the two numbers such that they add up to a specific target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

Example:

Given nums = [2, 7, 11, 15], target = 9,

Because nums[0] + nums[1] = 2 + 7 = 9,
return [0, 1].

*/

func main() {
	a := []int{2, 7, 11, 15}
	fmt.Printf("twoSumNaive(9,a) %v\n", twoSumNaive(9, a))
	fmt.Printf("twoSum(9,a) %v\n", twoSum(a, 9))

	b := []int{3, 2, 4}
	fmt.Printf("twoSumNaive(6,b) %v\n", twoSumNaive(6, b))
	fmt.Printf("twoSum(6,b) %v\n", twoSum(b, 6))
}

/*
time: O(n)
space: O(n)
*/
func twoSum(nums []int, target int) []int {
	m := make(map[int]int, len(nums))
	for i := 0; i < len(nums); i++ {
		if _, ok := m[target-nums[i]]; ok {
			return []int{m[target-nums[i]], i}
		}
		m[nums[i]] = i
	}
	return nums
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
