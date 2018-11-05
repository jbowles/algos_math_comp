using VMLS

"""
If a vector x isn’t constant (i.e., at least two of its entries are different), we can standardize it:
    1. subtracting its mean 
    2. and dividing by its standard deviation
The resulting standardized vector has mean value zero and RMS value one. Its entries are called z-scores. We’ll define a standardize function, and then check it with a random vector
"""

function standardize(x)
    x_tilde = x .- avg(x) #de-mean vector
    return x_tilde/rms(x_tilde)
end

x = rand(100)
avgrmsx = avg(x), rms(x)
println("(avg,rms)(x) = ", avgrmsx)
z = standardize(x);
#println("standardized = ", z)
println("Standardized should be: (avg~0, rms=1)")
avgrmsz = avg(z), rms(z)
println("(avg,rms)(z) = ", avgrmsz)