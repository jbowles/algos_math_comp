package main

import "testing"

var (
	s1 = "For example ram is costly reversed yeilds yltsoc si mar"
)

func BenchmarkReverseSentenceA(b *testing.B) {
	for i := 0; i < b.N; i++ {
		reverseSentenceA(s1)
	}
}
func BenchmarkReverseSentenceF(b *testing.B) {
	for i := 0; i < b.N; i++ {
		reverseSentenceFunk(s1)
	}
}
func BenchmarkReverseSentenceS(b *testing.B) {
	for i := 0; i < b.N; i++ {
		reverseSentenceS(s1)
	}
}
