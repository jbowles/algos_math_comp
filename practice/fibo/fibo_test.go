package main

import "testing"

func BenchmarkFibNaive(b *testing.B) {
	for i := 0; i < b.N; i++ {
		fibR(33)
	}
}
func BenchmarkFibL(b *testing.B) {
	for i := 0; i < b.N; i++ {
		fibL(33)
	}
}
func BenchmarkFibM(b *testing.B) {
	for i := 0; i < b.N; i++ {
		fibM(33)
	}
}
