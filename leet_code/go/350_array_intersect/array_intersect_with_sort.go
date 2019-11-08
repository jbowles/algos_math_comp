package main

/*
Note:

    Each element in the result should appear as many times as it shows in both arrays.
    The result can be in any order.

*/

import (
	"fmt"
	"sort"
)

func main() {
	a := []int{1, 2, 2, 1}
	b := []int{2, 2}
	fmt.Printf("%v \n", intersect(a, b))

	c := []int{4, 9, 5}
	d := []int{9, 4, 9, 8, 4}
	fmt.Printf("%v \n", intersect(c, d))
}

//time: O(nlogn + mlogm); space: O(1)
func intersect(a, b []int) []int {
	sort.Ints(b)
	sort.Ints(a)
	i, j, k := 0, 0, 0
	for i < len(a) && j < len(b) {
		if a[i] < b[j] {
			i++
		} else if a[i] > b[j] {
			j++
		} else {
			a[k] = a[i]
			k++
			i++
			j++
		}
	}

	// fmt.Printf("%v\n", a)
	return append([]int{}, a[:k]...)
}

/*
  approach 1 is in naive/ directory
  approach 3 is a built-in (not sure go has that)

1. What if the given array is already sorted? How would you optimize your algorithm?
 	* We can use either Approach 2 or Approach 3, dropping the sort of course. It will give us linear time and constant memory complexity.

2. What if nums1's size is small compared to nums2's size? Which algorithm is better?
 	* Approach 1 is a good choice here as we use a hash map for the smaller array.

3. What if elements of nums2 are stored on disk, and the memory is limited such that you cannot load all elements into the memory at once?

 	* If nums1 fits into the memory, we can use Approach 1 to collect counts for nums1 into a hash map. Then, we can sequentially load and process nums2.

 	* If neither of the arrays fit into the memory, we can apply some partial processing strategies:

		- Split the numeric range into subranges that fits into the memory. Modify Approach 1 to collect counts only within a given subrange, and call the method multiple times (for each subrange).

		-Use an external sort for both arrays. Modify Approach 2 to load and process arrays sequentially.

*/
