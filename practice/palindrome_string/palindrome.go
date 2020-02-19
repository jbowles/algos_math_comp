package main

import (
	"fmt"
)

func main() {
	var tester = []struct {
		input  string
		expect bool
	}{
		{"helleh", true},
		{"heleh", true},
		{"ueihwi", false},
	}
	for _, t := range tester {
		r := isPalindrome(t.input)
		fmt.Printf("for: '%s', isPalindrome=%t, CORRECT[%t]\n", t.input, r, r == t.expect)
	}
}

func isPalindrome(s string) bool {
	for l, r := 0, len(s)-1; l < r; l, r = l+1, r-1 {
		if s[l] != s[r] {
			return false
		}
	}
	return true
}

/*
// O(n) time, O(1) space
func isPalindrome(s string) bool {
	for i, j := 0, len(s)-1; i < j; i, j = i+1, j-1 {
		fmt.Printf("i=%d j=%d\n", i, j)
		fmt.Printf("s[i]=%s s[j]=%s\n", string(s[i]), string(s[j]))
		if s[i] != s[j] {
			return false
		}
	}
	return true
}
*/
