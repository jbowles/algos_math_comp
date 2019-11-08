package main

/*
Note:

    Each element in the result should appear as many times as it shows in both arrays.
    The result can be in any order.

*/

import "fmt"

func main() {
	a := []int{1, 2, 2, 1}
	b := []int{2, 2}
	fmt.Printf("intersect(a,b) %v\n", intersect(a, b))

	c := []int{4, 9, 5, 4, 9}
	d := []int{9, 4, 9, 8, 4}
	fmt.Printf("intersect(c,d) %v\n", intersect(c, d))

	var e = []int{1, 2, 3, 4, 5}
	var f = []int{2, 3, 5, 7, 11}
	fmt.Printf("intersect(e,f) %v\n", intersect(e, f))
}

/*
collect smaller slice into hash map with counts of values, then
set_intersect over smaller slice, check if those values in hashmap,
if so append and decremen hashmap value

time: O(n+m)
space:  O(min⁡(n,m))
*/
func intersect(a, b []int) []int {
	m := make(map[int]int)
	if len(a) < len(b) {
		for _, v := range a {
			m[v]++
		}
		return set_intersect(m, b)
	}

	for _, v := range b {
		m[v]++
	}
	return set_intersect(m, a)
}

// O(n)
func set_intersect(m map[int]int, s []int) []int {
	c := []int{}
	for _, v := range s {
		if i, ok := m[v]; ok && i > 0 {
			c = append(c, v)
			m[v]--
			fmt.Printf("%v\n", m)
		}
	}
	return c
}
