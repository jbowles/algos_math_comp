package main

import (
	"fmt"
	"math"
)

func main() {
	fmt.Printf("1221 PALINDROME[%t]\n", isPalindrome(1221))
	fmt.Printf("-1221 PALINDROME[%t]\n", isPalindrome(-1221))
	fmt.Printf("566666 PALINDROME[%t]\n", isPalindrome(566666))
	fmt.Printf("2147447412 PALINDROME[%t]\n", isPalindrome(2147447412))
}

func isPalindrome(x int) bool {
	if x <= 0 {
		return false
	}
	numDigits := math.Floor(math.Log10(float64(x))) + 1.0
	//most significant digit (10^n-1) VERSUS least significant digit (n%10)
	msdMask := int(math.Pow(10.0, numDigits-1))

	for i := 0; i < (int(numDigits) / 2); i++ {
		if x/msdMask != x%10 {
			return false
		}
		x %= msdMask //remove most significant digit
		x /= 10      //remove least significant digit
		msdMask /= 100
	}
	return true
}
