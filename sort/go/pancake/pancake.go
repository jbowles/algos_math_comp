package main

import "fmt"

/*
Pancake sorting is the colloquial term for the mathematical problem of sorting a disordered stack of pancakes in order of size when a spatula can be inserted at any point in the stack and used to flip all pancakes above it. A pancake number is the maximum number of flips required for a given number of pancakes. Here is source code of the Go Program to implement Pancake Sorting Algorithm.
*/
func main() {
	in := []int32{28, 11, 59, -26, 503, 158, 997, 193, -23, 44}
	fmt.Printf("in =%v\n", sort(in))

}

/*
swapTo swaps indices up to position value for a slice int32
	for l,r set to 0 and number of positions/cakes/stack-depth to flip;
	l less than r;
	l,r iterate +/- 1;
	then swap left,right with right,lift
*/
func swapUpTo(position int, a []int32) []int32 {
	fmt.Printf("swapUpTo position=%d\n", position)
	for l, r := 0, position; l < r; l, r = l+1, r-1 {
		fmt.Printf("\tswap l=%d,r=%d\n", l, r)
		a[l], a[r] = a[r], a[l]
	}
	return a
}

/*
sort is a pancake sort impl for the input slice up to the specified number of flips
	for i the length of slice input-1, i <0, work backwards;
	set and index and value agains input to find largest unsorted sequence;
	divide and conquer: backward from i--; forwards from j++;
	swap for value of i,idx;
	return sorted []int32

we have to go through all elements twice so O(n^2) time
*/
func sort(input []int32) []int32 {
	for i := len(input) - 1; i > 0; i-- {
		//set index,value; try to find longest seq
		idx, val := 0, input[0]
		for j := 1; j <= i; j++ {
			fmt.Printf("i=%d, j=%d\n", i, j)
			if input[j] > val {
				idx = 1
				val = input[j]
			}
		}
		//reassign input to swap final positions
		// this order is important! MUST swap largest seq first,
		// then for i
		input = swapUpTo(idx, input)
		input = swapUpTo(i, input)
	}
	return input
}
