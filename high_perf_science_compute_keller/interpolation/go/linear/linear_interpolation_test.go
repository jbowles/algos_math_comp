package main

import "testing"

// walk through explcit example of first test case
var coordsTestTable = []struct {
	y1     float64
	y2     float64
	x1     float64
	x2     float64
	x      float64
	expect float64
}{
	//(2 + (5-1)) * (8-2)/(13-1)
	// == (2+4) * 6/12
	// == 6 * 1/5
	// == 6 * 0.5 == 3.0
	{2, 8, 1, 13, 5, 3},
	{14, 23, 75, 130, 7, -8.836363636363636},
	{456, 678, 350, 756, 789, 489.384236453202},
}

func TestInterpV2(t *testing.T) {
	for i, c := range coordsTestTable {
		val := interpV2(c.y1, c.y2, c.x1, c.x2, c.x)
		if val != c.expect {
			t.Errorf("for coords: %d, expect: %v, got %v\n", i, c.expect, val)
		}
	}
}
func TestInterp(t *testing.T) {
	for i, c := range coordsTestTable {
		val := interp(c.y1, c.y2, c.x1, c.x2, c.x)
		if val != c.expect {
			t.Errorf("for coords: %d, expect: %f, got %f\n", i, c.expect, val)
		}
	}
}

// from fib_test.go
func BenchmarkInterpV2(b *testing.B) {
	c := coordsTestTable[2]
	for n := 0; n < b.N; n++ {
		interpV2(c.y1, c.y2, c.x1, c.x2, c.x)
	}
}
func BenchmarkInterp(b *testing.B) {
	c := coordsTestTable[2]
	for n := 0; n < b.N; n++ {
		interp(c.y1, c.y2, c.x1, c.x2, c.x)
	}
}
