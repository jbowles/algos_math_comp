package main

import "fmt"

func main() {
	in := []int{121, 11, 636, 99, 1, 666, 123321, -121, -1, -10, 10, -99, 82937483}
	for _, i := range in {
		fmt.Printf("%d is %v\n", i, isPalindrome(i))
	}

}

/*
	Time: 	O(log_10(n))
	Space: 	O(1)

	-- Since we've divided input by 10 then multiplying the reversed number by 10 extends
		the value back to the digit place of the original number
	-- Adding the last digit of original number to multiplied reversed number gives it
		the new tens place
	-- Once the original number is less than the reversedNumber we've gotten to the
		"halfway point" of the sequence... so we can use this half-point to reduce
		the sequence processing
*/
func isPalindrome(x int) bool {
	// x < 0 is not a palindrome
	if x < 0 {
		return false
	}
	// Also if the last digit of the number is 0, in order to be a palindrome,
	// the first digit of the number also needs to be 0.
	// Only 0 satisfy this property.
	if (x%10 == 0) && (x != 0) {
		return false
	}

	var reversedNum int
	for x > reversedNum {
		reversedNum = (reversedNum * 10) + (x % 10)
		x /= 10
	}
	// When the length is an odd number, we can get rid of the middle digit by reversedNum/10
	// For example when the input is 12321, at the end of the while loop we get x = 12, revertedNumber = 123,
	// since the middle digit doesn't matter in palidrome(it will always equal to itself), we can simply get rid of it.
	return (x == reversedNum) || (x == reversedNum/10)
}
