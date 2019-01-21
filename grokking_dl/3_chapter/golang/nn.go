package main

import "fmt"

func neural_network(input float32, weight float32) float32 {
	prediction := input * weight
	return prediction
}

func main() {
	var weight float32 = 0.1
	number_of_toes := []float32{8.5, 9.5, 10, 9}
	input := number_of_toes[0]

	pred := neural_network(input, weight)
	fmt.Println(pred)
}
