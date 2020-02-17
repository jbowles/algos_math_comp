package main

import "fmt"

func main() {
	tester := []struct {
		in  string
		out int
		sub string
	}{
		{" ", 1, " "},
		{"abc", 3, "abc"},
		{"abba", 2, "ab"},
		{"abcabcbb", 3, "abc"},
		{"abcdefghijabcdefghijabcde", 10, "abcdefghij"},
		{"fheowiukjjaiouyeiwop;", 9, "fheowiukj"},
		{"bbbbb", 1, "b"},
		{"pwwkew", 3, "wke"},
	}
	for _, t := range tester {
		res := los(t.in)
		fmt.Printf("input:'%s'==%d\nresult=%d. \tCORRECT[%t]\n\n", t.in, t.out, res, res == t.out)
	}
}

func max(i, j int) int {
	if i > j {
		return i
	}
	return j
}
func los(s string) int {
	var maxLen, start = 0, -1
	rs := []rune(s)
	var m = make(map[rune]int)
	for i := 0; i < len(s); i++ {
		if _, ok := m[rs[i]]; ok {
			start = max(start, m[rs[i]])
		}
		m[rs[i]] = i
		//abba
		//a:i=0:1,b:i=1:2||a:i=0:Y,b:i=X:Y
		maxLen = max(maxLen, i-start)
	}
	// fmt.Printf("%v %d \n", m, start)
	return maxLen
}

/*
func los(s string) int {
	var maxLen, start = 0, -1
	rs := []rune(s)
	var m = make(map[rune]int)
	for i := 0; i < len(rs); i++ {
		if _, ok := m[rs[i]]; ok {
			start = max(start, m[rs[i]])
		}
		m[rs[i]] = i
		maxLen = max(maxLen, i-start)
	}
	return maxLen
}


func losThree(str string) int {
	m, max, left := make(map[rune]int), 0, 0
	for idx, c := range str {
		if _, ok := m[c]; ok && m[c] >= left {
			if idx-left > max {
				max = idx - left
			}
			left = m[c] + 1
		}
		m[c] = idx
	}
	if len(str)-left > max {
		max = len(str) - left
	}
	return max
}
*/
