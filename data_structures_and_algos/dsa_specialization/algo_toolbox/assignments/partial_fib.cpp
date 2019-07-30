#include <stdlib.h>
#include <iostream>
#include <math.h>

long long fib(int n)
{
    double phi = (1 + sqrt(5)) / 2;
    return round(pow(phi, n) / sqrt(5));
}

// Function to return the required sum
long long calculateSum(int l, int r)
{

    // Using our deduced result
    long long sum = fib(r + 2) - fib(l + 1);
    return sum % (long)10;
}

int main() {
  int a = 0;
  int b = 0;
  std::cin >> a;
  std::cin >> b;
  std::cout << calculateSum(a, b)
  << std::endl;

}

