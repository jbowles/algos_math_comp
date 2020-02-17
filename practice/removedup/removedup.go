package main

import "fmt"

func main() {
	t := []int{1, 4, 2, 6, 3, 5, 3, 2, 5} //9
	result := dedup(t)
	good := []int{1, 4, 2, 6, 3, 5} //6
	fmt.Printf("CORRECT?[%t]\n", len(result) == len(good))
	fmt.Printf("%v ==\n%v\n", result, good)
}

func dedup(n []int) []int {
	out := []int{}
	seen := make(map[int]bool)
	for _, v := range n {
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
func dedup(in []int) []int {
	out := []int{}
	var seen = make(map[int]bool)
	for i, v := range in {
		fmt.Printf("%d, %d\n", i, v)
		if seen[v] {
			continue
		} else {
			out = append(out, v)
			seen[v] = true
		}
	}
	return out
}
*/
