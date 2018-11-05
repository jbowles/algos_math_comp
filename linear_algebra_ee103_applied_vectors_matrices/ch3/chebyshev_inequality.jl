using LinearAlgebra, VMLS

"""
The Chebyshev inequality states that the number of entries of an n-vector x that have absolute value at least a is no more than ∥x∥^2/a^2 = nrms(x)^2/a^2.

If this number is, say, 12.15, we can conclude that no more that 12 entries have absolute value at least a, since the number of entries is an integer.

So the Chebyshev bound can be improved to be floor(∥x∥^2/a), where floor(u) is the integer part of a positive number. Let’s define a function with the Chebyshev bound, including the floor function improvement, and apply the bound to the signal found above, for a specific value of a.
"""

t = 0:0.01:1; #some sequence of time series data
#typeof(t) 
# => StepRangeLen{Float64,Base.TwicePrecision{Float64},Base.TwicePrecision{Float64}}
x = cos.(8*t) - 2*sin.(11*t)

# define chebyshev bound function
cheb_bound(x,a) = floor(norm(x)^2/a)
a = 1.5

#no more than this number of entries can absolute value of at least 'a'
bound = cheb_bound(x,a)
println("a=", a)
println("no more than this number of entries can have absolute value of at least 'a': ", bound)

#number of entries of x with |x_i| >= a
sum_x_a = sum(abs.(x) .>= a)
println("number of entries >= a: ", sum_x_a)





