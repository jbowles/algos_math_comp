package main

import "fmt"

func main() {
	fmt.Printf("1221    	true== PALINDROME[%t]\n", isPalindrome(1221))
	fmt.Printf("-1221 		false == PALINDROME[%t]\n", isPalindrome(-1221))
	fmt.Printf("566666 		false == PALINDROME[%t]\n", isPalindrome(566666))
	fmt.Printf("2147447412 	true== PALINDROME[%t]\n", isPalindrome(2147447412))
	fmt.Printf("0 		false == PALINDROME[%t]\n", isPalindrome(0))
	fmt.Printf("100 		false == PALINDROME[%t]\n", isPalindrome(100))
}

func isPalindrome(n int) bool {
	if n <= 0 {
		return false
	}
	if n%10 == 0 {
		return false
	}
	var r int
	for n > r {
		r = (r * 10) + (n % 10)
		n /= 10
	}
	return (n == r) || (n == r/10)
}

/*
func isPalindrome(x int) bool {
	// less than 0 is not a palindrome
	if x <= 0 {
		return false
	}

	// if last digit is zero then first digit must be zero.
	// zero itself is not palindrome either
	if (x%10 == 0) && (x == 0) {
		return false
	}

	//similar to reversing a number excpet instead of `x != 0`
	// we want `x > reversedNum` (i.e., halfway point of digit sequence)
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
*/

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
