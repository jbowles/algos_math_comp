#=
Consider the random experiment where two independent, fair, six sided dice are rolled, and we wish to find the probability that the sum of the outcomes of the dice is even. Here the sample space can be represented as Ω = {1, . . . , 6}2, i.e. the Cartesian product of the set of single roll outcomes with itself.



    elements of the sample space are tuples 
    of the form (i, j) with i, j ∈ {1, . . . , 6}. 
A = {(i,j) | i+j == even}

    random experiment, 
    dice have no inherent bias, 
    assume a symmetric probability function for any B ⊂ Ω
P(B) = |B|/|Ω|

P(A) = 18/36 = 0.5
=#

N, faces = 10^6, 1:6

numSol = sum([iseven(i+j) for i in faces, j in faces]) / length(faces)^2
mcEst = sum([iseven(rand(faces) + rand(faces)) for _ in 1:N]) / N

println("Numerical solution = $numSol \nMonte Carlo estimate = $mcEst")