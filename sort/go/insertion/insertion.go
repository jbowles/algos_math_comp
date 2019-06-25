package main

import "fmt"

func main() {
	abar := []int{5, 2, 4, 6, 1, 3}
	fmt.Println("NOSORT:", abar)
	a := isort(abar)
	fmt.Println("isort SORTED:", a)
	bbar := []int{5, 2, 4, 6, 1, 3}
	b := insert_sort(bbar)
	fmt.Println("insert_sort SORTED:", b)

}

//isort SORTED: [5 1 2 3 4 6]
func isort(a []int) []int {
	for j := 1; j < len(a); j++ {
		key := a[j]
		i := j - 1
		for i >= 0 && a[i] > key {
			a[i+1] = a[i]
			i -= 1
		}
		a[i+1] = key
	}
	return a
}

//insert_sort SORTED: [1 2 3 4 5 6]
func insert_sort(a []int) []int {
	for j := 1; j < len(a); j++ {
		i := j
		for i > 0 {
			if a[i-1] > a[i] {
				a[i-1], a[i] = a[i], a[i-1]
			}
			i -= 1
		}
	}
	return a
}
