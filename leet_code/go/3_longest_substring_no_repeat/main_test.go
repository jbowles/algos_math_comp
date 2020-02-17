package main

import "testing"

var (
	//count is 3 == wke
	shortStr = "pwwkew"
	// count is 3 == abc; with multiple 'abc' substrings
	longStr = "ahfdkayfuioeuiojkdhfuiehwhuiaowiqeioyuhabcabcabcdiuwhhfuihewhufejwkjfhueiwhfabcabcabchdkfheuwhfindjsh89iabcjfkdjoeiwhuibcjdgufiewjhuihdiwoufkjgheihncfiuewhfioewhfuewfheowiukjfheiwlhfiewfkhewbcabcbb"
	// count is 10 == abcdefghij; with multiple 'abcdefghij' substrings
	longLongStr = "ahfabcdefghijabcdefghijdkayfuioeuiojkdhfuiehwhuiaowabcdefghijabcdefghijiqeioyuhabcabcabcdiuwhhfuihewhufejwkjfhueiwhfabcabcabchdkfheuwhfindjsh89iabcjfkdjoeiwhuibcjdgufiewjhuihdiwoufkjgheihncfiuewhfioewhfueabcdefghijabcdefghijwfheowiukjfheiwlhfiewfkhewbcabcbb"
)

func BenchmarkLosLongLong(b *testing.B) {
	for i := 0; i < b.N; i++ {
		los(longLongStr)
	}
}

func BenchmarkLos(b *testing.B) {
	for i := 0; i < b.N; i++ {
		los(longStr)
	}
}

func BenchmarkLosShort(b *testing.B) {
	for i := 0; i < b.N; i++ {
		los(shortStr)
	}
}

/*
func BenchmarkLosTwoLongLong(b *testing.B) {
	for i := 0; i < b.N; i++ {
		losTwo(longLongStr)
	}
}

func BenchmarkLosThreeLongLong(b *testing.B) {
	for i := 0; i < b.N; i++ {
		losThree(longLongStr)
	}
}
*/

/*
func BenchmarkLosTwo(b *testing.B) {
	for i := 0; i < b.N; i++ {
		losTwo(longStr)
	}
}

func BenchmarkLosThree(b *testing.B) {
	for i := 0; i < b.N; i++ {
		losThree(longStr)
	}
}
*/

/*
func BenchmarkLosTwoShort(b *testing.B) {
	for i := 0; i < b.N; i++ {
		losTwo(shortStr)
	}
}

func BenchmarkLosThreeShort(b *testing.B) {
	for i := 0; i < b.N; i++ {
		losThree(shortStr)
	}
}
*/

/*

los function edges out the other methods, specifically when string is long

goos: darwin
goarch: amd64
BenchmarkLosLongLong-8        	   80343	     13611 ns/op
BenchmarkLosTwoLongLong-8     	   82828	     13494 ns/op
BenchmarkLosThreeLongLong-8   	   57453	     19627 ns/op
BenchmarkLos-8                	  110139	     11176 ns/op
BenchmarkLosTwo-8             	   96043	     10619 ns/op
BenchmarkLosThree-8           	   97298	     12214 ns/op
BenchmarkLosShort-8           	 7382847	       161 ns/op
BenchmarkLosTwoShort-8        	 6971406	       188 ns/op
BenchmarkLosThreeShort-8      	 7696102	       157 ns/op




goos: darwin
goarch: amd64
BenchmarkLosLongLong-8        	   81136	     13816 ns/op
BenchmarkLosTwoLongLong-8     	   88028	     14739 ns/op
BenchmarkLosThreeLongLong-8   	   69499	     16928 ns/op
BenchmarkLos-8                	  108090	     11032 ns/op
BenchmarkLosTwo-8             	  110650	     10803 ns/op
BenchmarkLosThree-8           	   97076	     12490 ns/op
BenchmarkLosShort-8           	 7160439	       167 ns/op
BenchmarkLosTwoShort-8        	 7110103	       169 ns/op
BenchmarkLosThreeShort-8      	 7669390	       156 ns/op




goos: darwin
goarch: amd64
BenchmarkLosLongLong-8        	   84204	     13598 ns/op
BenchmarkLosTwoLongLong-8     	   88573	     13892 ns/op
BenchmarkLosThreeLongLong-8   	   77606	     15556 ns/op
BenchmarkLos-8                	  110515	     10770 ns/op
BenchmarkLosTwo-8             	  110488	     11064 ns/op
BenchmarkLosThree-8           	   98227	     12190 ns/op
BenchmarkLosShort-8           	 7470633	       164 ns/op
BenchmarkLosTwoShort-8        	 6989107	       168 ns/op
BenchmarkLosThreeShort-8      	 7737360	       156 ns/op

*/
