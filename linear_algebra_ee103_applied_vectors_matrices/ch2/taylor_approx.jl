"""
The (first-order) Taylor approximation of a function f :
  Rn → R,
at the point z, is the affine function of x given by
    f_hat(x) = f(z)+∇f(z)T(x−z)
For x near z, f_hat(x) is very close to f(x).
"""

# a function
f(x) = x[1] + exp(x[2]-x[1])
#and its gradient
grad_f(z) = [1-exp(z[2]-z[1]), exp(z[2]-z[1])]

z = [1,2]

#Taylor approx at z
f_hat(x) = f(z) + grad_f(z)'*(x-z)


"""
julia> z = [1,2]
2-element Array{Int64,1}:
 1
 2


julia> f(z)
3.718281828459045



julia> grad_f(z)
2-element Array{Float64,1}:
 -1.718281828459045
  2.718281828459045



julia> f_hat(z)
3.718281828459045


julia> f([1,2]), f_hat([1,2])
(3.718281828459045, 3.718281828459045)

julia> f([0.96,1.98]), f_hat([0.96, 1.98])
(3.733194763964298, 3.732647465028226)

julia> f([1.10,2.11]), f_hat([1.10,2.11])
(3.845601015016916, 3.845464646743635)
"""
