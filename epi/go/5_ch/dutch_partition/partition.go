package main

import "fmt"

func main() {

	t := []int{0, 1, 2, 0, 2, 1, 1}
	part(len(t)/2, t)
	fmt.Println(t)

	tt := []int{0, 2, 1, 1, 0, 2, 0, 1, 0, 2, 2, 0, 1, 2, 0, 2}
	part(len(tt)/2, tt)
	part(len(tt)/2, tt)
	fmt.Println(tt)

	ttt := []int{2, 0, 0, 2, 2, 1, 0}
	part(len(ttt)/2, ttt)
	part(len(ttt)/2, ttt)
	part(len(ttt)/2, ttt)
	fmt.Println(t)
}

// wrong... works for ind=2 but not ind=3
func part(ind int, A []int) []int {
	var pivot = A[ind]
	var smaller, equal, larger = 0, 0, len(A)
	fmt.Printf("pivot index=%d value=%d\n", ind, pivot)
	for equal < larger {
		if A[equal] < pivot {
			A[smaller], A[equal] = A[equal], A[smaller]
			smaller++
			equal++
		} else if A[equal] == pivot {
			equal++
		} else {
			larger--
			A[equal], A[larger] = A[larger], A[equal]
		}
	}
	return A
}

/*
func partNaive(ind int, A []int) []int {
	var pivot = A[ind]
	fmt.Println("pivot=", pivot)
	for i := 0; i < len(A)-1; i++ {
		for j := i; j < len(A); j++ {
			if A[j] < pivot {
				A[i], A[j] = A[j], A[i]
				break
			}
		}
		for i := len(A) - 2; i >= 0; i-- {
			for j := i; j >= 0; j-- {
				if A[j] > pivot {
					A[i], A[j] = A[j], A[i]
					break
				}
			}
		}
	}
	return A
}
*/

/*
const (
	kRed = iota
	kWhite
	kBlue
)
*/
