package main

import "fmt"

func main() {
	t := []int{4, 3, 7, 6, 5, 1, 2, 10, 9, 8}
	fmt.Println(sort(t))
}

// Runs MergeSort algorithm on a slice single
func sort(slice []int) []int {

	if len(slice) < 2 {
		return slice
	}
	mid := (len(slice)) / 2
	return merge(sort(slice[:mid]), sort(slice[mid:]))
}

// Merges left and right slice into newly created slice
func merge(left, right []int) []int {

	size, i, j := len(left)+len(right), 0, 0
	slice := make([]int, size, size+1)

	for k := 0; k < size; k++ {
		if i > len(left)-1 && j <= len(right)-1 {
			slice[k] = right[j]
			j++
		} else if j > len(right)-1 && i <= len(left)-1 {
			slice[k] = left[i]
			i++
		} else if left[i] < right[j] {
			slice[k] = left[i]
			i++
		} else {
			slice[k] = right[j]
			j++
		}
	}
	return slice
}

/*
func sort(a []int, p, q, r int) []int {
	n1 := q - p + 1
	n2 := r - q
	ls := make([]int, n1+1, n1+2)
	rs := make([]int, n2+1, n2+2)

	return a
}
*/
