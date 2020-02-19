package main

import "fmt"

func main() {
	//items := []int{3, 2, 3} // panic: runtime error: index out of range
	items := []int{4, 2, 4, 5, 2, 3, 1, 5, 3}
	res := findRepeat(items)
	fmt.Printf("%v\n", res)

	items2 := []int{1, 2, 3, 1, 3, 6, 6}
	res2 := findRepeat(items2)
	fmt.Printf("%v\n", res2)
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func findRepeat(ns []int) []int {
	out := []int{}
	for i := 0; i < len(ns); i++ {
		if ns[abs(ns[i])] >= 0 {
			ns[abs(ns[i])] = -ns[abs(ns[i])]
		} else {
			out = append(out, abs(ns[i]))
		}
	}
	return out
}

/*
func findRepeat(ns []int) []int {
	out := []int{}
	for i := 0; i < len(ns); i++ {
		if ns[abs(ns[i])] >= 0 {
			ns[abs(ns[i])] = -ns[abs(ns[i])]
		} else {
			out = append(out, abs(ns[i]))
		}
	}
	return out
}
*/

/*
//Find duplicates in O(n) time and O(1) extra space | Set 1
// go vervsion of this c implementation found here: https://www.geeksforgeeks.org/find-duplicates-in-on-time-and-constant-extra-space/
// size of array cannot be smaller or equal to largest integer in array
func findRepeating(arr []int) {
	//need to find largest value, comapre to len(arr) and bail if not good.
	// fmt.Println("arrary == ", arr)
	// fmt.Println("len(array) == ", len(arr))
	// fmt.Printf("repeated items: ")
	for i := 0; i < (len(arr) - 1); i++ {
		//fmt.Println("i: ", i)
		//fmt.Println("arr[i]: ", arr[i])
		//fmt.Println("arr[arr[i]]: ", arr[arr[i]])
		//fmt.Println("-arr[abs(arr[i])]: ", -arr[abs(arr[i])])
		if arr[abs(arr[i])] >= 0 {
			arr[abs(arr[i])] = -arr[abs(arr[i])]
		} else {
			fmt.Printf("%d  ", abs(arr[i]))
			//return
		}
	}
	fmt.Println()
}
*/
