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

// C++ Program to find sum of squares of
// Fibonacci numbers in O(Log n) time.
using namespace std;
const int MAX = 1000;

// Create an array for memoization
long long f[MAX] = { 0 };

// Returns n'th Fibonacci number using table f[]
long long fib(long long n)
{
	// Base cases
	if (n == 0)
		return 0;
	if (n == 1 || n == 2)
		return (f[n] = 1);

	// If fib(n) is already computed
	if (f[n])
		return f[n];

	long long k = (n & 1) ? (n + 1) / 2 : n / 2;

	// Applying above formula [Note value n&1 is 1
	// if n is odd, else 0]r
	f[n] = (n & 1) ? (fib(k) * fib(k) + fib(k - 1) * fib(k - 1))
				: (2 * fib(k - 1) + fib(k)) * fib(k);

	return f[n];
}

// Function to calculate sum of
// squares of Fibonacci numbers
long long calculateSumOfSquares(long long n)
{
	return std::abs(fib(n) * fib(n + 1)) % 10;
}

// Driver Code
int main()
{
  /*
	long long n = 6;

  cout << "Sum of Squares of Fibonacci numbers is : "
		<< calculateSumOfSquares(n) << endl;
	return 0;
  */

  long long n = 0;
  std::cin >> n;
  std::cout << calculateSumOfSquares(n)
  << endl;
}
