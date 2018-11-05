using LinearAlgebra

"""
Eucledian norm is 
    the sum of all elements squared
OR
    dot product of vector transpose and itself
"""

my_norm_sum(x) = sqrt(sum(x.^2))
my_norm_t(x) = sqrt(x'*x)

x = [ 2, -1, 2 ]

println("my_norm_sum(x) = ", my_norm_sum(x))
println("my_norm_t(x) = ", my_norm_t(x))
println("norm(x) = ", norm(x))