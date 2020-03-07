package main

import "testing"

var (
	ttt = []int{4, 3, 7, 6, 55, 4, 3, 7, 60, 4, 3, 7, 6, 5, 1, 2, 10, 96, 5, 410, 96, 5, 4, 3, 7, 3, 7, 8}
)

func BenchmarkSort(b *testing.B) {
	for i := 0; i < b.N; i++ {
		sort(ttt)
	}
}
func BenchmarkSortTee(b *testing.B) {
	for i := 0; i < b.N; i++ {
		sortTee(ttt)
	}
}

func BenchmarkSortTwo(b *testing.B) {
	for i := 0; i < b.N; i++ {
		sortTwo(ttt)
	}
}

/*
goos: darwin
goarch: amd64
BenchmarkSort-8      	 4088696	       278 ns/op
BenchmarkSortTee-8   	51658411	        21.8 ns/op
BenchmarkSortTwo-8   	26037170	        45.1 ns/op

goos: darwin
goarch: amd64
BenchmarkSort-8      	 4183171	       284 ns/op
BenchmarkSortTee-8   	51951517	        21.6 ns/op
BenchmarkSortTwo-8   	25520971	        44.1 ns/op
*/
