/*
 * =====================================================================================
 *
 *       Filename:  sum_fib_v2.cpp
 *
 *    Description:
 *
 *        Version:  1.0
 *        Created:  07/17/2019 22:02:56
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
#include <vector>

using namespace std;

long long fib(long long n) {
    if (n <= 1) return 1;                           // base case
    static std::vector<long> computed(n + 1, 0);         // this vector holds computed vals
    if (computed[n]) return computed[n];            // if we have already computed it
    return computed[n] = fib(n - 1) + fib(n - 2); // if we have not computed it yet
}


long long sum(long long n) {
  return fib(n + 2) - 1;
}

int main() {
  long long n = 0;
  std::cin >> n;
  //std::cout << fib(n)
  std::cout << sum(n)
  << std::endl;
}
