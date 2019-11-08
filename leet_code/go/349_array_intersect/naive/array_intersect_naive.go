package main

import "fmt"

func main() {
	a := []int{1, 2, 2, 1}
	b := []int{2, 2}
	fmt.Printf("intersectNaive(b,a) %v\n", intersectNaive(b, a))
	fmt.Printf("intersectNaive(a,b) %v\n", intersectNaive(a, b))
	fmt.Printf("intersect(b,a) %v\n", intersect(b, a))
	fmt.Printf("intersect(a,b) %v\n", intersect(a, b))

	c := []int{4, 9, 5}
	d := []int{9, 4, 9, 8, 4}
	fmt.Printf("intersectNaive(d,c) %v\n", intersectNaive(d, c))
	fmt.Printf("intersectNaive(c,d) %v\n", intersectNaive(c, d))
	fmt.Printf("intersect(d,c) %v\n", intersect(d, c))
	fmt.Printf("intersect(c,d) %v\n", intersect(c, d))

	var e = []int{1, 2, 3, 4, 5}
	var f = []int{2, 3, 5, 7, 11}
	fmt.Printf("intersectNaive(f,e) %v\n", intersectNaive(f, e))
	fmt.Printf("intersectNaive(e,f) %v\n", intersectNaive(e, f))
	fmt.Printf("intersect(f,e) %v\n", intersect(f, e))
	fmt.Printf("intersect(e,f) %v\n", intersect(e, f))
}

// O(n*m)
func intersectNaive(a, b []int) []int {
	var c []int
	for _, ae := range a {
		for _, be := range b {
			if (ae == be) && !contains(c, be) {
				c = append(c, be)
			}
		}
	}
	return c
}

//O(n)
func contains(c []int, in int) bool {
	for i := range c {
		if c[i] == in {
			return true
		}
	}
	return false
}

/*
better yet leverage a set intersect. create a Set from a,b, and in set_intersect
only loop over the larger array and check for items in smaller array

O(n+m)
*/
func intersect(a, b []int) []int {
	aa := toUnique(a)
	bb := toUnique(b)
	if len(aa) > len(bb) {
		return set_intersect(aa, bb)
	}
	return set_intersect(bb, aa)
}

//O(n)
func set_intersect(a, b []int) []int {
	// output := make([]int, len(a))
	output := []int{}
	for _, s := range a {
		if contains(b, s) {
			output = append(output, s)
		}
	}
	// return toUnique(output)
	return output
}

//O(n)
func toUnique(k []int) []int {
	l := []int{}
	for _, s := range k {
		if !contains(l, s) {
			l = append(l, s)
		}
	}
	return l
}
