package main

import (
	"fmt"

	"github.com/jbowles/mthutil"
)

// take unordered list, compare adjacent items each time
// putting in order of magnitude
// can only swap elements max 2 times

func bubbleSort(items []int) {
	//n := len(items)
	//sorted := false
	var (
		n      = len(items)
		sorted = false
	)

	for !sorted {
		swapped := false
		for i := 0; i < n-1; i++ {
			if items[i] > items[i+1] {
				items[i+1], items[i] = items[i], items[i+1]
				swapped = true
			}
		}
		if !swapped {
			sorted = true
		}
		n = n - 1
	}
}

func main() {
	s := mthutil.GenRandom(20)
	fmt.Printf("\n--UNSORTED--\n\n%v\n", s)
	bubbleSort(s)
	fmt.Printf("\n--Sorted--\n\n%v\n", s)
}
