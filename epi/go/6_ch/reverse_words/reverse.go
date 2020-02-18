package main

import (
	"fmt"
	"strings"
	"unicode"
)

func main() {
	testR()
}

func testR() {
	var tester = []struct {
		input  string
		expect string
	}{
		{"ram is costly", "costly is ram"},
		{"ram is costly and expensive", "expensive and costly is ram"},
		{"For example ram is costly reversed yeilds yltsoc si mar", "mar si yltsoc yeilds reversed costly is ram example For"},
	}

	for _, t := range tester {
		rF := reverseSentenceFunk(t.input)
		fmt.Printf("F: input ='%s'\nresult='%s'\nexpect='%s'\nCORRECT[%t] \n\n", t.input, rF, t.expect, rF == t.expect)
		rS := reverseSentenceS(t.input)
		fmt.Printf("S: input ='%s'\nresult='%s'\nexpect='%s'\nCORRECT[%t] \n\n", t.input, rS, t.expect, rS == t.expect)
		rA := reverseSentenceA(t.input)
		fmt.Printf("A: input ='%s'\nresult='%s'\nexpect='%s'\nCORRECT[%t] \n\n", t.input, rA, t.expect, rA == t.expect)
	}
}

//fastest O(n)
func reverseSentenceA(s string) string {
	w := strings.Fields(s)
	for l, r := 0, len(w)-1; l < r; l, r = l+1, r-1 {
		w[l], w[r] = w[r], w[l]
	}
	return strings.Join(w, " ")
}

// second fastest
func reverseSentenceS(s string) string {
	out := []rune{}
	sr := []rune(s)
	start, end := 0, len(sr)
	for i := len(sr) - 1; i > 0; i-- {
		if unicode.IsSpace(sr[i]) {
			start = i
			out = append(out, sr[start+1:end]...)
			out = append(out, ' ')
			end = start
		}
	}
	return string(append(out, sr[0:end]...))
}

func reverse(s string) string {
	rs := []rune(s)
	for l, r := 0, len(rs)-1; l < r; l, r = l+1, r-1 {
		rs[l], rs[r] = rs[r], rs[l]
	}
	return string(rs)
}

//third fastest
func reverseSentenceFunk(s string) string {
	var out string
	sr := strings.Split(reverse(s), " ")
	for _, v := range sr {
		out += " " + reverse(v)
	}
	return strings.Trim(out, " ")
}
