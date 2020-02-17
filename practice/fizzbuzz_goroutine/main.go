package main

import "fmt"

func main() {
	for out := range fizzbuzz(100) {
		fmt.Println(out)
	}
}

func fizzbuzz(n int) <-chan string {
	out := make(chan string, n)
	go func() {
		for i := 1; i <= n; i++ {
			r := ""
			if i%3 == 0 {
				r += "Fizz"
			}
			if i%5 == 0 {
				r += "Buzz"
			}
			if r == "" {
				r = fmt.Sprintf("%d", i)
			}
			out <- r
		}
		close(out)
	}()
	return out
}
