package main

import "fmt"

func main() {
	/*
		t := []int{1, 4, 2, 6, 3, 5, 3, 2, 5} //9, valid=6
		result := dedup(t)
		good := []int{1, 4, 2, 6, 3, 5} //6
		fmt.Printf("CORRECT?[%t]\n", len(result) == len(good))
		fmt.Printf("%v ==\n%v\n", result, good)
	*/
	t := []int{1, 4, 2, 6, 3, 5, 3, 2, 5} //9, valid=6
	good := []int{1, 4, 2, 6, 3, 5}       //6
	resu := dedup(t)
	fmt.Printf("CORRECT?[%t]\n", check(resu, good))
	fmt.Printf("resu:%v ==\ngood:%v\n", resu, good)
}

func dedup(a []int) []int {
	out := []int{}
	var m = make(map[int]bool)
	for _, v := range a {
		if m[v] {
			continue
		}
		m[v] = true
		out = append(out, v)
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
