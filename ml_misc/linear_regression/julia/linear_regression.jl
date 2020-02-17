#using LinearAlgebra, GLM, DataFrames, BenchmarkTools
using LinearAlgebra, DataFrames, Plots

#=
julia
include("linear_regresion.jl")

minₓ ∥Aₓ - b∥₂
=#

N = 100
xp = rand(N)
Xp = [ones(N) xp]
yp = 10 .+ xp .* 0.3

# normal equations approach
#fastest but least accurate; doubles the number of digits you lose to rounding.
#only use when you know you have a well-conditioned matrices
function linreg1(y, X)
    #β_hat
    return (X' * X) \ (X' * y)
end
#QR factorization
#slower than linreg1 but uses pivoted QR factorization.
#more accurate for badly conditioned matrices than "normal equations" approach
#it does not square the condition number
function linreg2(y,X)
    #β_hat
    return X \ y
end
#SVD approach
#uses SVD to apply psuedo-inverse. the slowest of the linreg* approaches
#most accurate of linreg*
function linreg3(y, X)
    #β_hat
    return pinv(X) * y
end

df = DataFrame(X = xp, Y = yp)

#println("linreg1: ", linreg1(yp, Xp))
#println("linreg2: ", linreg2(yp, Xp))
#println("linreg3: ", linreg3(yp, Xp))
#println("GLM.fit: ", GLM.fit(LinearModel, Xp, yp, true))
#println("lm(): ", lm(@formula(Y ~ X), df))


#println("runnin benchmarks...")
#@benchmark linreg1(yp, Xp)
#@benchmark linreg2(yp, Xp)
#@benchmark linreg3(yp, Xp)
#@benchmark GLM.fit(LinearModel, Xp, yp, true)
#@benchmark lm(@formula(Y ~ X), df)