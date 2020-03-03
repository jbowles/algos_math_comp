package main

import "fmt"

func main() {
	fmt.Println("1==", countFlip("-----")) //1
	// fmt.Println("2==", countFlip("+-"))    //2
	// fmt.Println("1==", countFlip("-+"))    //1
	// fmt.Println("3==", countFlip("--+-"))  //3
	// fmt.Println("3==", countFlip("-+-"))   //3
}

func countFlip(s string) int {
	c := 0
	rs := []rune(s)
	for i := 1; i < len(rs); i++ {
		if rs[i] != rs[i-1] {
			c++
		}
	}
	if rs[len(rs)-1] == '-' {
		c++
	}
	return c
}

/*
func countFlip(s string) int {
	c := 0
	rs := []rune(s)
	for i := 1; i < len(rs); i++ {
		if rs[i] != rs[i-1] {
			c++
		}
	}
	if rs[len(rs)-1] == '-' {
		c++
	}
	return c
}

func countFlip(s string) int {
	var c int
	rs := []rune(s)
	for i := 1; i < len(rs); i++ {
		if rs[i] != rs[i-1] {
			c++
		}
	}
	//condition where -----
	if rs[len(s)-1] == '-' {
		c++
	}

	return c
}
*/
