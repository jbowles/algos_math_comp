package main

import (
	"fmt"
	"strings"
)

func main() {
	fmt.Println("1.TRUE", proper("(fdkls(fkdl))"))
	fmt.Println("2.TRUE", proper("(fdji)890s(fkdl)"))
	fmt.Println("3.FALSE", proper("(fd(ji)890s(fk(dl)"))
	fmt.Println("4.FALSE", proper(")fd("))
	fmt.Println("5.FALSE", proper("((fd)"))
	fmt.Println("6.FALSE", proper(")(fd"))
}

func proper(s string) bool {
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

/*
func proper(s string) bool {
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
