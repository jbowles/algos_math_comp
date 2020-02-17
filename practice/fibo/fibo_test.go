package main

import "testing"

func BenchmarkFibNaive(b *testing.B) {
	for i := 0; i < b.N; i++ {
		fibNaive(33)
	}
}
func BenchmarkFibL(b *testing.B) {
	for i := 0; i < b.N; i++ {
		fibL(33)
	}
}
