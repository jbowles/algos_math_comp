using VMLS

"""
refer to the vector x − avg(x)1 as the de-meaned version of x

elementwise subtract from average of the vector itself

returns vector with mean removed from itself

useful for understanding how vector entries deviate from vector mean
"""

de_mean(x) = x .- avg(x)
x = [1,-2.2,3]

println(x)
println(avg(x))

x_tilde = de_mean(x)

println(x_tilde)
println(avg(x_tilde)) #should be close to zero