package main

import "fmt"

// Improves binary search where sorted values are uniformly distributed.
// Binary always goes to the middle first, but this can go to other locations.
// return the index
func interSearch(array []int, key int) int {
	//pull out first and last items of sorted list
	minValue, maxValue := array[0], array[len(array)-1]
	//index values
	lowIndex, highIndex := 0, len(array)-1

	fmt.Printf("BEGIN: minValue: %d, maxValue: %d, lowIndex: %d, highIndex: %d\n", minValue, maxValue, lowIndex, highIndex)

	for {
		//
		if key < minValue {
			return lowIndex
		}
		if key > maxValue {
			return highIndex + 1
		}
		var guess int
		if highIndex == lowIndex {
			guess = highIndex
			fmt.Printf("'if highIndex == lowIndex', guess==%d\n", guess)
		} else {
			size := highIndex - lowIndex
			//size==11
			fmt.Printf("'highIndex != lowIndex' size==%d\n", size)
			//                     (11-1) * (63-1)                  / (100-1)
			// (10*62)/99 == 5.2
			sizeSub1 := float64(size - 1)
			keySubMin := float64(key - minValue)
			maxSubMin := float64(maxValue - minValue)
			fmt.Printf("sizeOff: %f, keySubMin: %f, maxSubMin: %f .... '%f * (%f/%f)'\n", sizeSub1, keySubMin, maxSubMin, sizeSub1, keySubMin, maxSubMin)
			offset := int(sizeSub1 * (keySubMin / maxSubMin))
			fmt.Printf("'highIndex != lowIndex' offset==%d\n", offset)
			guess = lowIndex + offset
			fmt.Printf("'highIndex != lowIndex' guess==%d\n", guess)
		}

		//maybe we found it?
		if array[guess] == key {
			fmt.Printf("'if array[guess] == key' %d\n", array[guess])
			//scan backwards for start of value range for duplicates
			//this ensures we return the index of the first instance of the term
			for guess > 0 && array[guess-1] == key {
				fmt.Printf("'for guess > 0 && array[guess-1] == key' guess==%d\n", guess)
				guess--
			}
			fmt.Printf("found first index. guess==%d\n", guess)
			return guess
		}

		//if we guessed too highIndex, guess lowIndexer, or vice versa
		if array[guess] > key {
			fmt.Printf("'array[guess] > key' before decr: highIndex: %d, maxValue: %d\n", highIndex, maxValue)
			highIndex = guess - 1
			maxValue = array[highIndex]
			fmt.Printf("'array[guess] > key' before decr: highIndex: %d, maxValue: %d\n", highIndex, maxValue)
		} else {
			fmt.Printf("'array[guess] < key' before incr: lowIndex: %d, minValue: %d\n", lowIndex, minValue)
			lowIndex = guess + 1
			minValue = array[lowIndex]
			fmt.Printf("'array[guess] < key' after incr: lowIndex: %d, minValue: %d\n", lowIndex, minValue)
		}
	}
}

func main() {
	//items := []int{1, 2, 9, 20, 31, 45, 63, 63, 63, 70, 70, 100}
	items := []int{1, 2, 9, 20, 31, 45, 63, 70, 70, 100}
	//fmt.Println(interSearch(items, 70)) //9
	fmt.Println(interSearch(items, 63)) //6
}

/*

	items := []int{1, 2, 9, 20, 31, 45, 63, 63, 63, 70, 70, 100}
	fmt.Println(interSearch(items, 63)) //6

	BEGIN: minValue: 1, maxValue: 100, lowIndex: 0, highIndex: 11
	'highIndex != lowIndex' size==11
	'highIndex != lowIndex' offset==6
	'highIndex != lowIndex' guess==6
	'if array[guess] == key' 63
	found first index. guess==6
	6
*/

/*
	items := []int{1, 2, 9, 20, 31, 45, 63, 70, 70, 100}
	fmt.Println(interSearch(items, 63)) //6

	BEGIN: minValue: 1, maxValue: 100, lowIndex: 0, highIndex: 9
	'highIndex != lowIndex' size==9
	'highIndex != lowIndex' offset==5
	'highIndex != lowIndex' guess==5
	'array[guess] < key' before incr: lowIndex: 0, minValue: 1
	'array[guess] < key' after incr: lowIndex: 6, minValue: 63
	'highIndex != lowIndex' size==3
	'highIndex != lowIndex' offset==0
	'highIndex != lowIndex' guess==6
	'if array[guess] == key' 63
	found first index. guess==6
	6
*/
