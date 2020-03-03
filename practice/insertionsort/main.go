package main

import "fmt"

func main() {
	ttt := []int{4, 3, 7, 6, 5, 1, 2, 10, 9, 8}
	fmt.Println(sort(ttt))
	fmt.Printf("CORRECT[%t]\n", check(ttt))
	//fmt.Printf("CORRECT[%t]\n", checkSeq(ttt))
	// fmt.Println(check([]int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}))
}

/*
(0+1)=1 - 0 = 1
(1+2)=3 - 1 = 2
(2+3)=5 - 2 = 3
(3+4)=7 - 3 = 4
(4+5)=9 - 4 = 5
*/
func check(ns []int) bool {
	for i := 0; i < len(ns)-1; i++ {
		// fmt.Printf("%d %d %t\n", ns[i+1], ns[i]+1, ns[i+1] == ns[i]+1)
		if ns[i+1] != ns[i]+1 {
			return false
		}
	}
	return true
}

func sort(ns []int) []int {
	for i := 0; i < len(ns); i++ {
		for j := i; j > 0 && ns[j] < ns[j-1]; j-- {
			ns[j-1], ns[j] = ns[j], ns[j-1]
		}
	}
	return ns
}

/*
func sortTee(n []int) []int {
	for i := 1; i < len(n); i++ {
		for j := i; j > 0 && n[j] < n[j-1]; j-- {
			n[j-1], n[j] = n[j], n[j-1]
		}
	}
	return n
}
*/

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
