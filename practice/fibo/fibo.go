package main

import "fmt"

func main() {
	testFibR()
	testFibL()
	testFibD()
}

// r := makeRange(0, (100%60)+2)
func makeRange(min, max int) []int {
	a := make([]int, max-min+1)
	for i := range a {
		a[i] = min + i
	}
	return a
}

func fibR(n int) int {
	if n <= 1 {
		return n
	}
	return fibR(n-1) + fibR(n-2)
}
func fibL(n int) int {
	if n <= 1 {
		return 1
	}
	f := make([]int, n+1, n+2)
	if n < 2 {
		f = f[0:2]
	}
	f[0] = 0
	f[1] = 1
	for i := 2; i <= n; i++ {
		f[i] = f[i-1] + f[i-2]
	}
	return f[n]
}

func fibD(n int) int {
	f0, f1 := 0, 1
	for range makeRange(1, ((n % 60) + 2)) {
		f0, f1 = f1, f0%10+f1%10
	}
	return (f0 - 1) % 10
}

/*
// recursive
func fibNaive(size int) int {
	if size <= 1 {
		return size
	}
	return fibNaive(size-1) + fibNaive(size-2)
}

func fibL(n int) int {
	f := make([]int, n+1, n+2)
	fmt.Println(f)
	if n < 2 {
		f = f[0:2]
	}
	f[0] = 0
	f[1] = 1
	for i := 2; i <= n; i++ {
		f[i] = f[i-1] + f[i-2]
	}
	return f[n]
}
func fibD(n int) int {
	f0, f1 := 0, 1
	for range makeRange(1, ((n + 2) % 60)) {
		f0, f1 = f1, f0%10+f1%10
		//f0 = std::mem::replace(&mut f1, f2)
	}
	return (f0 - 1) % 10
}

// r := makeRange(0, (100%60)+2)
func makeRange(min, max int) []int {
	a := make([]int, max-min+1)
	for i := range a {
		a[i] = min + i
	}
	return a
}

// NOTES ON fibD
// return last digit of large F number
// the nth Fibonacci number can be calculated as F(n+2)-1
// `(n%60 + 2)` is the `F(n+2)` and `f0-1` is the final subtraction `-1`
// Doing modulo 60 for our index helps us leapfrog indexes
// by exploiting periodicity patterns in the fibonacci sequence.
// Without the mod(60) the additions would take a very long time,
// noticeable mostly with largest nth numbers [614162383528, 99999999999999999


def last_digit(n):
    a, b = 0, 1
    for i in range((n + 2) % 60):
        a, b = b, (a + b) % 10
    return 9 if a == 0 else a - 1

print([last_digit(n) for n in range(1, 11)])
*/

func testFibR() {
	fmt.Println("Testing fibR")
	//calculate actual fibonacci value
	tester := [][]int{{0, 1}, {1, 1}, {33, 3524578}} // 83: 147844013817084101}
	for _, t := range tester {
		resNaive := fibR(t[0])
		fmt.Printf("fibNaive. size: %d, answer: %d, result: %d, CORRECT?[%t]\n", t[0], t[1], resNaive, resNaive == t[1])
	}
}

func testFibL() {
	fmt.Println("Testing fibL")
	//calculate actual fibonacci value
	tester := [][]int{{0, 1}, {1, 1}, {33, 3524578}} // 83: 147844013817084101}
	for _, t := range tester {
		resL := fibL(t[0])
		fmt.Printf("fibL. size: %d, answer: %d, result: %d, CORRECT?[%t]\n", t[0], t[1], resL, resL == t[1])
	}
}
func testFibD() {
	fmt.Println("Testing fibD")
	//calculate last digit of large fibonacci values
	//n=50 == 12586269025
	//n=99 == 218922995834555169026
	//n=178 == 7084593923980518516849609894969925639
	//n=256 == 10770594215935749279482183257489712959102052723690265265
	//n=300 == 222232244629420445529739893461909967206666939096499764990979600
	testerTwo := [][]int{
		{3, 4},
		{100, 5},
		{239, 0},
		{614162383528, 9},
	}
	for _, t := range testerTwo {
		res := fibD(t[0])
		fmt.Printf("fibD. size: %d, answer: %d, result: %d, CORRECT?[%t]\n", t[0], t[1], res, res == t[1])
	}
	/*
		for _, v := range []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10} {
			fmt.Println(fibD(v))
		}
	*/
}
