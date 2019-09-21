#include <iostream>
#include <vector>

int coll(int x)
{
    if (x % 2 == 0)
    {
        return x / 2;
    }
    else
    {
        return 3 * x + 1;
    }
}

std::vector<int> collatz_coll(int x)
{
    std::vector<int> l;
    l.insert(l.begin(), x);
    while (x != 1)
    {
        x = coll(x);
        l.insert(l.end(), x);
    }
    return l;
}

int collatz(int x)
{
    int step = 0;
    while (x != 1)
    {
        x = coll(x);
        step += 1;
    }
    return step;
}

void one_collatz_step()
{
    int x;
    std::cout << "Number: " << std::endl;
    std::cin >> x;
    std::cout << collatz(x) << std::endl;
}

void steps()
{
    for (int i = 1; i <= 100; i++)
    {
        std::cout << collatz(i) << std::endl;
    }
}
void collatz_values()
{
    int x;
    std::cout << "Number: " << std::endl;
    std::cin >> x;
    std::vector<int> input = collatz_coll(x);
    std::cout << '[';
    for (std::vector<int>::size_type i = 0; i < input.size(); i++)
    {
        if (i == input.size() - 1)
        {
            std::cout << input.at(i);
            std::cout << ']' << std::endl;
        }
        else
        {
            std::cout << input.at(i) << ',';
            std::cout << ' ';
        }
    }
}

int main()
{
    // steps();
    // one_collatz_step();
    collatz_values();
    return 0;
}
