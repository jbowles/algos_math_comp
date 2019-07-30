/*
 * =====================================================================================
 *
 *       Filename:  sum_fib_v3.cpp
 *
 *    Description:
 *
 *        Version:  1.0
 *        Created:  07/20/2019 15:31:45
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
#include <vector>
#include <cstdlib>
using namespace std;

int fibonacci_sum_fast(long long n)
{
    if (n <= 1)
        return n;
    else
    {
        n = n % 60;
        long long sum = 1;
        vector<int> a(n + 2);
        a[0] = 0;
        a[1] = 1;
        for (long long i = 2; i <= n + 2; i++)
        {
            a[i] = (a[i - 1] + a[i - 2]) % 10;
            cout<<a[i]<<endl;
        }
        sum = a[n + 2] - 1;
        return sum % 10;
    }
}
int main()
{
    unsigned long long n = 0;
    /*while (true)
{ n=rand()%100;
int res1=fibonacci_sum_naive(n);
int res2=fibonacci_sum_fast(n);
if (res1!=res2)
{std::cout<<res1<<" "<<res2<<"Error"<<std::endl;
break;}
else
{std::cout<<"\nok\n";
continue;}
}*/
    std::cin >> n;
    std::cout << fibonacci_sum_fast(n);
    return 0;
}
