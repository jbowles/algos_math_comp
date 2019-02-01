flow(rate) = ([1. 0 0], [1. -(1. + rate) 0], [0. 1 -(1. + rate)])

# r == positive per-period interest rate
r = 0.05
e1, l1, l2 = flow(r)
#e1 == single payment of $1 in period time t =1.
#l1 == load on $1 in period t = 1, paid back in period t = 2 with interest r.
#l2 loan on $1 in period t = 2, paid back in period t = 3 with interest rate r.

#==
CASH FLOW
c = α1 * e1 + α2 * l1 + α3 * l2
:

From the third component we have 
    c3 = α3(−(1 + r)), so α3 = −c3/(1 + r)

From the second component we have
    c2 =α2(−(1+r))+α3  =  α2(−(1+r))−c3/(1+r)

so  
    α2 = −c2/(1 + r) − c3/(1 + r)^2

Finally from c1 = α1 + α2, we have 
    α1 =c1 +c2/(1+r)+c3/(1+r)^2,
which is the net present value (NPV) of the cash flow c.
==#

c = [1, 2, -3]
#coefficients of expansion
α3 = -c[3] / (1 + r)
α2 = -c[2] / (1 + r) - (c[3] / (1 + r)^2)
α1 = c[1] + (c[2] / (1 + r)) + (c[3] / (1 + r)^2) #NPV of cash flow

α1 * e1 + α2 * l1 + α3 * l2