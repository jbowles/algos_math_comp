package main

import "fmt"

//gets power series of integare a an dreturn tuple of square and cube
func powerSeries(a int) (int, int) {
	return a * a, a * a * a
}

func main() {
	p2, p3 := powerSeries(3)
	fmt.Println("powerSeries(3)", p2, p3)
}
