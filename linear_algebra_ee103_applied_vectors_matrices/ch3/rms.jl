using LinearAlgebra, VMLS, Plots
# The RMS value of a vector x is rms(x) = ∥x∥/√n. In Julia, this is expressed as norm(x)/sqrt(length(x))

myrms(x) = norm(x)/sqrt(length(x))
#println(myrms)

#define a vector (which represents a signal, i.e., the value of some quantity at uniformly space time instances), and find its RMS value
t = 0:0.01:1; #list of times
#typeof(t) 
# => StepRangeLen{Float64,Base.TwicePrecision{Float64},Base.TwicePrecision{Float64}}
x = cos.(8*t) - 2*sin.(11*t)
println(avg(x))
println(myrms(x))

plot(t,x)
#average
plot!(t, avg(x)*ones(length(x)))
#upper inequality
plot!(t, (avg(x)+myrms(x))*ones(length(x)), color = :green)
#lower inequality
plot!(t, (avg(x)-myrms(x))*ones(length(x)), color = :blue)
plot!(legend=false)