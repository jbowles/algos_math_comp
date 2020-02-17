package main

import "fmt"

func main() {
	//ans2 := []int{0, 1, 0, 1, 1, 2, 2}
	//ans3 := []int{0, 0, 1, 2, 2, 1, 1}
	tN := []int{0, 1, 2, 0, 2, 1, 1}
	fmt.Println("partNaive3", partNaive(3, tN))
	t := []int{0, 1, 2, 0, 2, 1, 1}
	fmt.Println("part3     ", part(3, t))

	fmt.Println("")
	ttN := []int{0, 1, 2, 0, 2, 1, 1}
	fmt.Println("partNaive5", partNaive(5, ttN))
	tt := []int{0, 1, 2, 0, 2, 1, 1}
	fmt.Println("part5     ", part(5, tt))

	ttt := []int{-3, 0, -1, 1, 1, 1, 8, 8, 8, 4, 2}
	fmt.Println("part     ", part(1, ttt))
	tt4 := []int{0, 1, 1, 0, 1, 2, 1, 2, 0, 0, 0, 1}
	fmt.Println("part     ", part(0, tt4))
	tt5 := []int{0, 1, 1, 0, 1, 2, 1, 2, 0, 0, 0, 1}
	fmt.Println("part     ", partNaive(0, tt5))
}

// wrong... works for ind=2 but not ind=3
func part(ind int, A []int) []int {
	var pivot = A[ind]
	var smaller, equal, larger = 0, 0, (len(A) - 1)
	fmt.Println("pivot=", pivot)
	for equal < larger {
		if A[equal] < pivot {
			A[smaller], A[equal] = A[equal], A[smaller]
			// A[equal], A[smaller] = A[smaller], A[equal]
			smaller++
			equal++
		} else if A[equal] == pivot {
			equal++
		} else {
			A[equal], A[larger] = A[larger], A[equal]
			// A[larger], A[equal] = A[equal], A[larger]
			larger--
		}
	}
	return A
}

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

/*
const (
	kRed = iota
	kWhite
	kBlue
)
*/
