package main

import "fmt"

func main() {
	t := []int{1, 4, 2, 6, 3, 5, 3, 2, 5} //9, valid=6
	good := []int{1, 4, 2, 6, 3, 5}       //6
	resu := dedup(t)
	fmt.Printf("CORRECT?[%t]\n", check(resu, good))
	fmt.Printf("resu:%v ==\ngood:%v\n", resu, good)
}

func dedup(ns []int) []int {
	out := []int{}
	seen := make(map[int]bool)
	for _, v := range ns {
		if seen[v] {
			continue
		} else {
			out = append(out, v)
			seen[v] = true
		}
	}
	return out
}

/*
func dedup(n []int) []int {
	out := []int{}
	seen := make(map[int]bool)
	// c := 0
	for _, v := range n {
		if seen[v] {
			continue
		} else {
			out = append(out, v)
			seen[v] = true
			// c++
		}
	}
	// fmt.Printf("valid elements count=%d\n", c)//6
	return out
}

*/
func check(a, b []int) bool {
	if len(a) != len(b) {
		return false
	}
	for i := 0; i < len(a); i++ {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}
