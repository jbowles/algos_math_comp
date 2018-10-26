package main

import (
	"fmt"

	"github.com/jbowles/mthutil"

	"golang.org/x/exp/rand"
)

// divide and conquer, unsorted array broken into sub-arrays
// that are partially sorted until all elements are sorted.
func qSort(a []int) []int {
	if len(a) < 2 {
		return a
	}
	//define left,right as a min,max/start,end
	start, end := 0, len(a)-1
	fmt.Printf("start: %d, end: %d\n", start, end)
	pivot := rand.Int() % len(a)
	fmt.Printf("pivot: %d\n", pivot)

	//swap pivot and end
	a[pivot], a[end] = a[end], a[pivot]
	fmt.Printf("swap pivot and end a: %v\n", a)
	for i := range a {
		//is index less than end
		fmt.Printf("a[i] %d < %d a[end]\n", a[i], a[end])
		if a[i] < a[end] {
			//swap start and index
			a[start], a[i] = a[i], a[start]
			fmt.Printf("swap start and index a: %v\n", a)
			//increment start
			start++
		}
	}
	//swap start and end
	a[start], a[end] = a[end], a[start]

	fmt.Printf("swap start and end a: %d\n", a)
	//run qsort on left side of slice
	//fmt.Printf("qSort(a[:start]): %d\n", a[:start])
	fmt.Printf("a[:start]; %d\n", a[:start])
	qSort(a[:start])

	//run qsort on right side of slice
	//fmt.Printf("qSort(a[start+1:]): %d\n", a[start+1:])
	fmt.Printf("a[start+1:]; %d\n", a[start+1:])
	qSort(a[start+1:])
	return a
}

func main() {
	s := mthutil.GenRandom(5)
	fmt.Println("\n--- Unsorted --- \n\n", s)
	qSort(s)
	fmt.Println("\n--- Sorted --- \n\n", s)
}
