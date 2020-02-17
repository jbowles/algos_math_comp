package main

import "testing"

//go test -bench=.

func BenchmarkRomanToInt(b *testing.B) {
	// run the Fib function b.N times
	for n := 0; n < b.N; n++ {
		_ = romanToInt("MCMXCIV")
	}
}
func BenchmarkRomanToIntTwo(b *testing.B) {
	// run the Fib function b.N times
	for n := 0; n < b.N; n++ {
		_ = romanToIntTwo("MCMXCIV")
	}
}
func BenchmarkRomanToIntThree(b *testing.B) {
	// run the Fib function b.N times
	for n := 0; n < b.N; n++ {
		_ = romanToIntThree("MCMXCIV")
	}
}
