package main

import "fmt"

func main() {
	t1 := []string{}
	t2 := []string{}
	for i := 1; i <= 100; i++ {
		r := fizzbuzzOne(i)
		r2 := fizzbuzzTwo(i)
		if r != "" {
			s := fmt.Sprintf("%s %d", r, i)
			s2 := fmt.Sprintf("%s %d", r2, i)
			fmt.Println(s, s2)
			t1 = append(t1, s)
			t2 = append(t2, s2)
			continue
		}
	}
	for i := 0; i < len(t1)-1; i++ {
		if t1[i] != t2[i] {
			panic("Bad values")
		}
		// fmt.Println("check ", i)
	}
	fmt.Println("All good!")
}

func fizzbuzzOne(i int) string {
	var r string
	if i%3 == 0 {
		r += "Fizz"
	}
	if i%5 == 0 {
		r += "Buzz"
	}
	return r
}
func fizzbuzzTwo(i int) string {
	if i%15 == 0 {
		return "FizzBuzz"
	}
	if i%3 == 0 {
		return "Fizz"
	}
	if i%5 == 0 {
		return "Buzz"
	}
	return ""
}
