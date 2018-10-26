package main

import "fmt"

// find items in list by reducing data to search, increasing rate of hit.
// list must be sorted.
// O(Log n)
func binarySearch(needle int, haystack []int) bool {
	// len(hyastack) == 10
	low := 0
	//hiigh == 9
	high := len(haystack) - 1
	fmt.Printf("begin low %d \n", low)
	fmt.Printf("begin high %d \n", high)

	//low less than high
	for low <= high {
		// find middle total/2 == 9/2 == 4;  (then 7... 5... 6)
		median := ((low + high) / 2)
		fmt.Printf("top for loop median %d \n", median)
		fmt.Printf("top for loop low %d \n", low)
		fmt.Printf("top for loop high %d \n", high)

		//middle of list less than search term
		// split the list in half and serch halfsies
		// haystack value less than search term...
		if haystack[median] < needle {
			//move into end of list
			// low == 0
			low = median + 1
			// low == 5; (then 6)
			fmt.Printf("< needle low %d \n", low)
		} else {
			//move into start of list
			// high == 9
			high = median - 1
			// high == 6; (then 5)
			fmt.Printf("> needle high %d \n", high)
		}
	}
	fmt.Printf("out of for loop low %d \n", low)
	fmt.Printf("out of for loop high %d \n", high)
	if low == len(haystack) || haystack[low] != needle {
		return false
	}
	fmt.Printf("term== %d\n", haystack[low])
	return true
}

func main() {
	items := []int{1, 2, 9, 20, 31, 45, 63, 70, 100, 34}
	fmt.Println(binarySearch(63, items))
}

/*
	items := []int{1, 2, 9, 20, 31, 45, 63, 70, 100, 34}
	fmt.Println(binarySearch(63, items))

	begin low 0
	begin high 9

		median := ((low + high) / 2)
	top for loop median 4
	top for loop low 0
	top for loop high 9
	< needle low 5

		median := ((low + high) / 2)
	top for loop median 7
	top for loop low 5
	top for loop high 9
	> needle high 6

		median := ((low + high) / 2)
	top for loop median 5
	top for loop low 5
	top for loop high 6
	< needle low 6

		median := ((low + high) / 2)
	top for loop median 6
	top for loop low 6
	top for loop high 6

	> needle high 5
	out of for loop low 6
	out of for loop high 5
	true
*/
