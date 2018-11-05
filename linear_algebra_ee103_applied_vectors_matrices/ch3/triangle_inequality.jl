using LinearAlgebra
# triangle inequality, ∥x + y∥ ≤ ∥x∥ + ∥y∥, for some specific values of x and y.
#
x = randn(10); y = randn(10);
lhs = norm(x+y)
println(lhs)
rhs = norm(x) + norm(y)
println(rhs)
