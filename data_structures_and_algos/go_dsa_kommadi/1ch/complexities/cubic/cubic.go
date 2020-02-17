package main

import "fmt"

/*
processing time proportional to the cube of inputs.
*/

// O(n^3)
func main() {
	var k, l, m int
	var arr [10][10][10]int
	for k = 0; k < 10; k++ {

		for l = 0; l < 10; l++ {

			for m = 0; m < 10; m++ {
				arr[k][l][m] = 1
				// fmt.Printf("elem k[%d],l[%d],m[%d]\n", arr[k], arr[l], arr[m])
				fmt.Println("elem k,l,m", k, l, m, " is", arr[k][l][m])
			}
		}
	}
}
