#abstract exponentiation function, subtract max for numerical stability
_exp(x::AbstractVecOrMat) = exp.(x .- maximum(x))
#abstract exponentiation function, subtract max for numerical stability and scale by theta
_exp(x::AbstractVecOrMat, θ::AbstractFloat) =   exp.((x .- maximum(x)) * θ)
#softmax algorithm expects stablized eponentiated e
_sftmax(e::AbstractVecOrMat, d::Integer) = (e ./ sum(e, dims = d))

# top level softmax function
function softmax(X::AbstractVecOrMat{T}, dim::Integer)::AbstractVecOrMat where T <: AbstractFloat
    _sftmax(_exp(X), dim)
end

function softmax(X::AbstractVecOrMat{T}, dim::Integer, θ::Float64)::AbstractVecOrMat where T <: AbstractFloat
    _sftmax(_exp(X, θ), dim)
end



scores4 = [1.1, 5.0, 2.8, 7.3] # won't work: 4-element Array{Float64,1}
scores1x4 = [1.1 5.0 2.8 7.3] # 1×4 Array{Float64,2} ... need to transpose
scores4x4 = [
    1.0 2.0 3.0 6.0;
    2.0 4.0 5.0 6.0;
    3.0 8.0 7.0 6.0;
    3.0 8.0 7.0 6.0;
]
scores3x4 = [
    1.1 5.0 2.2 7.3;
    6.5 3.2 8.8 5.3;
    2.7 5.1 9.6 7.4;
]
X =[
    1.1 5.0 2.2 7.3;
    6.5 3.2 8.8 5.3;
    2.7 5.1 9.6 7.4;
]

Z =[
    [1.1 5.0 2.2 7.3];
    [6.5 3.2 8.8 5.3];
    [2.7 5.1 9.6 7.4];
]

Y = cat(
    [0.844  0.237; 0.364  0.768; 0.811  0.959;],
    [0.511  0.06; 0.594  0.029; 0.963  0.292;],
    [0.463  0.869; 0.704  0.786; 0.173  0.89;], dims=3)


# softmax over rows
softmax(X, 1, 0.5)
#3×4 Array{Float64,2}
# 0.0552325  0.406861  0.0145858  0.413368
# 0.821845   0.165417  0.395459   0.15207
# 0.122922   0.427721  0.589955   0.434562

# softmax over columns
softmax(X, 2, 0.5)
#3×4 Array{Float64,2}:
# 0.0312892  0.219922   0.0542321  0.694557
# 0.204121   0.0392014  0.644654   0.112024
# 0.0215954  0.0716994  0.680265   0.22644

# softmax over columns, minimze!
softmax(X, 2, 50.0)
#3×4 Array{Float64,2}:
# 2.33728e-135  1.13798e-50   1.79849e-111  1.0
# 1.13798e-50   2.49773e-122  1.0           9.96473e-77
# 1.47368e-150  1.92195e-98   1.0           1.68891e-48



#chkdim(x::AbstractVecOrMat) = size(x, 1) == 1 ? cat(dims = 1, x, ones(size(x))) : x
smax_naive(x, dims=1, θ=1.0) = exp.(x*θ) ./ sum(exp.(x*θ), dims = d)
f(x) = exp.(x) ./ sum(exp.(x))

#1xY matrix (unit vector) needs to sum across dimension 2
function softmax_naive(x::Matrix)::Matrix
    r, _ = size(x)
    if r == 1
        return exp.(x) ./ sum(exp.(x), dims = 2)
    end
    exp.(x) ./ sum(exp.(x), dims = 1)
end

#1xY matrix (unit vector) needs to sum across dimension 2
function softmax_naive(x::Matrix, θ::Float64)::Matrix
    r, _ = size(x)
    if r == 1 
        return exp.(x * θ) ./ sum(exp.(x * θ), dims = 2)
    end
    exp.(x * θ) ./ sum(exp.(x * θ), dims = 1)
end

#== fix types to work with n-dims....
_aexp(x, θ::AbstractFloat) =   exp.((x .- maximum(x)) * θ)
#softmax algorithm expects eponentiated e
_sftmax(e, d::Integer) = (e ./ sum(e, dims = d))
function softmax(X, dim::Integer, θ::Float64)
    _sftmax(_aexp(X, θ), dim)
end
==#