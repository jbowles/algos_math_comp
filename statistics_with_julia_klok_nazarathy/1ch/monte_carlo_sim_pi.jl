# estimate value of π with Monte Carlo.
# Area of one quarter section of the unit circle is π/4. 
# if we generate random points, (x,y), within a unit box, [0, 1] × [0, 1], 
# and calculate the proportion of total points that fall within the quarter circle, 
# we can approximate π via:
#   pi-hat = 4 * (Number of points with x2 + y2 ≤ 1) / (Total number of points)

using Random, LinearAlgebra, Plots; pyplot()
Random.seed!()

N = 10^5
data = [[rand(),rand()] for _ in 1:N]
indata = filter((x -> norm(x) ≤ 1), data) # (x^2 + y^2) <= 1
outdata = filter((x -> norm(x) > 1), data) # (x^2 + y^2) > 1
piApprox = 4*length(indata)/N
println("Pi Estimate: ", piApprox)

scatter(first.(indata),last.(indata), c=:blue, ms=1, msw=0)
scatter!(first.(outdata),last.(outdata), c=:red, ms=1, msw=0, xlims=(0,1), ylims=(0,1), legend=:none, ratio=:equal)