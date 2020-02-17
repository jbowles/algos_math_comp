package main

import "fmt"

func main() {
	for i := 1; i <= 100; i++ {
		r := fizzbuzzNaive(i)
		if r != "" {
			fmt.Println(r, " ", i)
			continue
		}
	}
}

func fizzbuzzNaive(i int) string {
	var r string
	if i%3 == 0 {
		r += "Fizz"
	}
	if i%5 == 0 {
		r += "Buzz"
	}
	return r
	//fmt.Println(r)
}
