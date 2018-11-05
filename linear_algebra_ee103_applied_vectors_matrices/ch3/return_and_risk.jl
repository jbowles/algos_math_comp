using VMLS

"""
evaluate the mean return and risk 
(measured by standard deviation) of the four time series
"""

a = ones(10)
t = avg(a), stdev(a)
println("t = ", t)
b = [ 5, 1, -2, 3, 6, 3, -1, 3, 4, 1 ]
t1 = avg(b), stdev(b)
println("t1 = ", t1)
c = [ 5, 7, -2, 2, -3, 1, -1, 2, 7, 8 ]
t2 = avg(c), stdev(c)
println("t2 = ", t2)
d = [ -1, -3, -4, -3, 7, -1, 0, 3, 9, 5 ]
t3 = avg(d), stdev(d)
println("t3 = ", t3)
