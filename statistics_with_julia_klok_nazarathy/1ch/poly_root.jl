using Roots

#==
f(x)=−10x2 +3x+1
-----------------
x= −3 ± sqrt(2^2−4(−10)) /2(-10)= 3±7 /20   ⇒ x1 =0.5, x2 =−0.2.
==#
function polyGen(a...)
    n = length(a) - 1
    poly = function(x)
        return sum([a[i + 1] * x^i for i in 0:n])
    end
    return poly
end

# polynom = polyGen(1, 3, -10)
# zeroVals = Roots.find_zeros(polynom, -10, 10)


zeroVals = Roots.find_zeros(polyGen(1, 3, -10), -10, 10)
println("zeros of function f(x): ", zeroVals)


# function sumc(a, n, x)
#     return sum([a[i + 1] * x^i for i in 0:n])
# end

# a = [1 3 10]
# sumc(a, length(a)-1, rand(10,10))
