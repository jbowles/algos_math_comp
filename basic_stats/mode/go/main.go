package main

import (
	"fmt"
)

func main() {
	testFindMode()
}
func testFindMode() {
	m := map[int]int{1: 8, 2: 4, 6: 1, 8: 3, 9: 4, 19: 44}
	fmt.Println("for", m)
	fmt.Printf("mode[19] == %d\n", find_mode(m))
}

func find_mode(hist map[int]int) int {
	var res, tmp int
	for k, v := range hist {
		if v > tmp {
			res = k
			tmp = v
		}
	}
	return res
}
