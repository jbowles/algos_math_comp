package main

import "fmt"

func main() {
	var tester = []struct {
		in     []string
		expect string
	}{
		// {[]string{"flower", "flow", "flight"}, "fl"},
		// {[]string{"dog", "racecar", "car"}, ""},
		{[]string{"dogoooder", "dog", "doglet"}, "dog"},
	}

	for _, t := range tester {
		res := lcp(t.in)
		fmt.Printf("in:'%s', expect:'%s'\ngot:'%s', CORRECT[%t]\n\n", t.in, t.expect, res, res == t.expect)
	}
}

func lcp(s []string) string {
	if len(s) == 0 {
		return ""
	}
	var pref []byte
	fmt.Printf("%d  %v\n", len(s[0]), s[0])
	for i := 0; i < len(s[0]); i++ {
		for j := 1; j < len(s); j++ {
			if len(s[j]) <= i || s[j][i] != s[0][i] {
				// fmt.Printf("W-%s, len(s[j])==%d, i==%d\n", s[j], len(s[j]), i)
				// fmt.Printf("W-%s, len(s[j])==%d, i==%d %s, %s\n", s[j], len(s[j]), i, string(s[j][i]), string(s[0][i]))
				return string(pref)
			}
		}
		pref = append(pref, s[0][i])
	}
	return string(pref)
}

/*
func lcp(strs []string) string {
	if len(strs) == 0 {
		return ""
	}
	var pref []byte
	for i := 0; i < len(strs[0]); i++ {
		for j := 1; j < len(strs); j++ {
			if len(strs[j]) <= i || strs[j][i] != strs[0][i] {
				return string(pref)
			}
		}
		pref = append(pref, strs[0][i])
	}

	return string(pref)
}
*/
