package main

import "fmt"

/*
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
*/

func multThreeFive(count int) int {
	sum := 0
	//continue out on for loop if we sum to prevent summing more than once
	for i := 1; i < count; i++ {
		//for multiples of 3 only
		if i%3 == 0 {
			sum += i
			continue
		}
		//for multiples of 5 only
		if i%5 == 0 {
			sum += i
			continue
		}
	}
	return sum
}

func multThreeFiveV2(count int) int {
	sum := 0
	for i := 1; i < count; i++ {
		switch {
		case (i%3 == 0), (i%5 == 0):
			sum += i
		}
	}
	return sum
}

func multThreeFiveV3(count int) int {
	sum := 0
	for i := 1; i < count; i++ {
		switch (i%3 == 0) || (i%5 == 0) {
		case true:
			sum += i
		}
	}
	return sum
}

func main() {
	//answer := multThreeFive(10) //==23
	answer := multThreeFive(1000)     //==233168
	answerv2 := multThreeFiveV2(1000) //==233168
	answerv3 := multThreeFiveV3(1000) //==233168
	fmt.Println(answer)
	fmt.Println(answerv2)
	fmt.Println(answerv3)
}
