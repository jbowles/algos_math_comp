package main

import "fmt"

//pass over list using index to move from start to end.
// each item examined and if no match move to next.
// pass over sequentially.
// O(n)
func linearSearch(list []int, key int) int {
	for i, item := range list {
		if item == key {
			return i
		}
	}
	return -1
}

func main() {
	items := []int{95, 78, 46, 58, 45, 86, 99, 251, 320}
	fmt.Printf("expect: true, got: %v\n", linearSearch(items, 58))
	fmt.Printf("expect: false, got: %v\n", linearSearch(items, 5558))

	sterm := 86
	idx := linearSearch(items, sterm)
	fmt.Printf("search term: %d, value at[%d] == %d\n", sterm, idx, items[idx])
}
