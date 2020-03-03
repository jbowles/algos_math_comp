#=
To carry out the analysis, we assume birthdays are uniformly distributed 
in the set {1, . . . , 365}. For n people in a room, we wish to evaluate 
the probability that at least two people share the same birthday. Set the 
sample space, Ω, to be composed of ordered tuples (x_1,...,x_n) with 
x_i ∈ {1,...,365}. Hence, |Ω| = 365^n. Now set the event A to be the set 
of all tuples (x_1,...,x_j) where x_i = x_j for some distinct i and j.


    A^c = 365 * 364 * ... * (365 - n+1) = 365!/(365-n)!

    P(A) = 1 - P(A^c) = 1 - |A^c|/|Ω| = 1 - (365 * 364 * ... * (365 - n+1)) / 365^n
=#

using StatsBase, Combinatorics, Plots; pyplot()
N = 10^3

matchExists1(n) = 1 - prod([k/365 for k in 365:-1:365-n+1])
matchExists2(n) = 1 - factorial(365, 365-big(n)) / 365^big(n)

function bdEvent(n)
    birthdays = rand(1:365,n)
    dayCounts = counts(birthdays, 1:365)
    return maximum(dayCounts) > 1
end

probEst(n) = sum([bdEvent(n) for _ in 1:N]) / N

xGrid = 1:50
analyticSolution1 = [matchExists1(n) for n in xGrid]
analyticSolution2 = [matchExists2(n) for n in xGrid]
println("Maximum error: $(maximum(abs.(analyticSolution1 - analyticSolution2)))")

mcEstimates = [probEst(n) for n in xGrid]

plot(xGrid, analyticSolution1, c=:blue, label="Analytic solution")
scatter!(xGrid, mcEstimates, c=:red, ms=6, msw=0, shape=:xcross,
    label="MC estimate", xlims=(0,50), ylims=(0, 1),
    xlabel="Number of people in room",
    ylabel="Probability of birthday match",
    legend=:topleft)