package gdlgo3

import "testing"

func TestNN1(t *testing.T) {
	var weight float32 = 0.1
	number_of_toes := []float32{8.5, 9.5, 10, 9}
	input := number_of_toes[0]

	pred := NN1(input, weight)
	//go test -v if you wann see logs
	t.Logf("NN1 prediction == %f", pred)
	if pred < 0.85000001 {
		t.Error("NN1 prediction wrong", pred)
	}
}
