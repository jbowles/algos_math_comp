package main

import "fmt"

func main() {
	//t := []int{4, 3, 7, 6, 5, 1, 2, 10, 9, 8}
	//fmt.Println(sort(t))
	//tt := []int{4, 3, 7, 6, 5, 1, 2, 10, 9, 8}
	//fmt.Println(sortTwo(tt))
	ttt := []int{4, 3, 7, 6, 5, 1, 2, 10, 9, 8}
	fmt.Println(sortTee(ttt))
}

func sort(a []int) []int {
	for j := 1; j < len(a); j++ {
		i := j
		for i > 0 {
			if a[i-1] > a[i] {
				a[i-1], a[i] = a[i], a[i-1]
			}
			i--
		}
	}
	return a
}

func sortTee(n []int) []int {
	for i := 1; i < len(n); i++ {
		for j := i; j > 0 && n[j] < n[j-1]; j-- {
			n[j-1], n[j] = n[j], n[j-1]
		}
	}
	return n
}

/*
//fastest
//found this https://golang.org/src/sort/sort.go line:25
func sortTee(in []int) []int {
	for i := 0 + 1; i < len(in); i++ {
		for j := i; j > 0 && in[j] < in[j-1]; j-- {
			in[j-1], in[j] = in[j], in[j-1]
		}
	}
	return in
}
*/

//insertion sort
func sortTwo(a []int) []int {
	for j := 1; j < len(a); j++ {
		key := a[j]
		i := j - 1
		for i >= 0 && a[i] > key {
			a[i+1] = a[i]
			i--
		}
		a[i+1] = key
	}
	return a
}
