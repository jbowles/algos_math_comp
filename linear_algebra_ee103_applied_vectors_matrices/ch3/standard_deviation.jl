using LinearAlgebra, Statistics, VMLS

"""
std vector, std(x) = ∥x − avg(x)1∥/√n, where n is the length of the vector

norm of the demeaned vector x divided by the square root of the length of vector x


(Julia’s Statistics package has a similar function, std(x), which computes 
the value ∥x − avg(x)1∥/√n − 1, where n is the length of x.)
"""
x = rand(100)
#My VMLS package definition of standard deviation; they use 'stdev'
my_stdev(x) = norm(x .- avg(x))/sqrt(length(x))
#My Statistics package definition of standard deviation; they use 'stdv'
my_stdv(x) = norm(x .- avg(x)*ones(length(x)))/sqrt(length(x)-1)

println("avg(x) = ", avg(x))
println("norm(x) = ", norm(x))
println("rms(x) = ", rms(x))
println("my_stdev(x) = ", my_stdev(x))
println("stdev(x) = ", stdev(x))
println("my_stdv(x) = ", my_stdv(x))
println("std(x) = ", std(x))
