using LinearAlgebra
scores = [
    1.0 2.0 3.0 6.0;
    2.0 4.0 5.0 6.0;
    3.0 8.0 7.0 6.0;
    3.0 8.0 7.0 6.0;
]

function softmax_naive(x)
    exp.(x) ./ sum(exp.(x), dims=1)
end

function softmax_wrong(x)
    exp.(x) / sum(exp.(x), dims=1)
end

function ∇softmax!(out::AbstractVecOrMat, Δ::AbstractVecOrMat, xs::AbstractVecOrMat)
  s = sum(exp, xs, dims=1)
  out .= exp.(xs)./s.*(Δ .- sum(Δ .* exp.(xs), dims=1)./s)
end
