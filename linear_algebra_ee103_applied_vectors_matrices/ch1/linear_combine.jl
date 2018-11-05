function linear_combination(coeff, vectors)
    return sum(coeff[i] * vectors[i] for i = 1:length(vectors))
end

println(linear_combination((-0.5, 1.5), ([1, 2], [3, 4])))