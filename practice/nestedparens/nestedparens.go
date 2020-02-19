package main

import (
	"fmt"
	"strings"
)

func main() {
	fmt.Println("1.TRUE", isNested("(fdkls(fkdl))"))
	fmt.Println("2.TRUE", isNested("(fdji)890s(fkdl)"))
	fmt.Println("3.FALSE", isNested("(fd(ji)890s(fk(dl)"))
	fmt.Println("4.FALSE", isNested(")fd("))
	fmt.Println("5.FALSE", isNested("((fd)"))
	fmt.Println("6.FALSE", isNested(")(fd"))
}

func isNested(s string) bool {
	c := 0
	for _, val := range strings.Split(s, "") {
		if val == ")" {
			if c--; c < 0 {
				return false
			}
		} else if val == "(" {
			c++
		}
	}
	return c == 0
}

/*
func isNested(s string) bool {
	c := 0
	for _, val := range strings.Split(s, "") {
		if val == "(" {
			c++
		} else if val == ")" {
			c--
			if c < 0 {
				return false
			}
		}
	}
	return c == 0
}
*/
