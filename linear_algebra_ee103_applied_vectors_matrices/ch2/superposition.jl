"""
The function f(x) = aTx is linear, which means that for any n-vectors x and y, and any scalars α and β, the superposition equality

    f(αx + βy) = αf(x) + βf(y)


  Let’s define the inner product function f for a specific value of a, and then verify superposition in Julia for specific values of x, y, α, and β. (This check does not show that the function is linear. It simply checks that superposition holds for these specific values.)
"""

a = [-2, 0, 1, -3]
f(x) = a'*x #inner product function
x = [2,2,-1,1]; y = [0,1,-1,0];

α = 1.5; β = -3.7;
lhs = f(α*x + β*y)
println(lhs)
rhs = α*f(x) + β*f(y)
println(rhs)

#For the function f(x) = aTx, we have f(e_3) = a_3. Let’s check that this holds in our example.
e3 = [0,0,1,0]
println(f(e3))




