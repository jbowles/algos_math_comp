/*
 * =====================================================================================
 *
 *       Filename:  sum_of_squares_fib.cpp
 *
 *    Description:
 *
 *        Version:  1.0
 *        Created:  07/15/2019 14:22:23
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (),
 *   Organization:
 *
 * =====================================================================================
 */
#include <stdlib.h>
#include <iostream>
#include <math.h>

// C++ Program to find sum of Fibonacci numbers in O(Log n) time.
using namespace std;
const int MAX = 1000;

// Create an array for memoization
long double f[MAX] = { 0 };

// Returns n'th Fibonacci number using table f[]
long double fib(long double n)
{
	// Base cases
	if (n == 0)
		return 0;
	if (n == 1 || n == 2)
		return (f[(long)n] = 1);

	// If fib(n) is already computed
	if (f[(long)n])
		return f[(long)n];

	long double k = ((long)n & 1) ? (n + 1) / 2 : n / 2;

	// Applying above formula [Note value n&1 is 1
	// if n is odd, else 0].
	f[(long)n] = ((long)n & 1) ? (fib(k) * fib(k) + fib(k - 1) * fib(k - 1))
				: (2 * fib(k - 1) + fib(k)) * fib(k);

	return round(f[(long)n]);
}

// Function to calculate sum of
// squares of Fibonacci numbers
long double calculateSumOfSquares(long double n)
{
	return fib(n + 2)-1;
}

// g++ -pipe -O2 -std=c++14 sum_fib.cpp -lm
int main()
{
  long double n = 0;
  std::cin >> n;
  printf("%6.1Lf",calculateSumOfSquares(n));
  //std::cout << calculateSumOfSquares(n)
  //<< endl;
}
