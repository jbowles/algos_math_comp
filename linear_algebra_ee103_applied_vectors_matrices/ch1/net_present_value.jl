c = [0.1, 0.1, 0.1, 1.1]; # array cash flow vector
n = length(c);
r = 0.05; #5% per-period interest rate
d = (1+r) .^ -(0:n-1) #element-wise raise scalar powers to array from range (0:n-1)
println(d);

NPV = c' * d
println(NPV)
