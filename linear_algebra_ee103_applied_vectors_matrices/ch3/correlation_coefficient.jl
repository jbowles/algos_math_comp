using VMLS, LinearAlgebra
"""
ρ = (a ̃T ̃b) / (∥a ̃∥∥ ̃b∥)

rho =   de-meaned a transpose times de-meaned b
        divided by norm of de-meaned a times norm of de-meaned b
"""
#ρ

function correl_coef(a,b)
    a_tilde = a .- avg(a)
    b_tilde = b .- avg(b)
    return (a_tilde'*b_tilde) / (norm(a_tilde)*norm(b_tilde))
end

a = [4.4, 9.4, 15.4, 12.4, 10.4, 1.4, -4.6, -5.6, -0.6, 7.4];
b = [6.2, 11.2, 14.2, 14.2, 8.2, 2.2, -3.8, -4.8, -1.8, 4.2];
println(correl_coef(a,b))

a = [4.1, 10.1, 15.1, 13.1, 7.1, 2.1, -2.9, -5.9, 0.1, 7.1];
b = [5.5, -0.5, -4.5, -3.5, 1.5, 7.5, 13.5, 14.5, 11.5, 4.5];
println(correl_coef(a,b))

a = [-5.0, 0.0, 5.0, 8.0, 13.0, 11.0, 1.0, 6.0, 4.0, 7.0];
b = [5.8, 0.8, 7.8, 9.8, 0.8, 11.8, 10.8, 5.8, -0.2, -3.2];
println(correl_coef(a,b))

x = randn(10^6); y = randn(10^6);
@time correl_coef(x,y)

x = randn(10^7); y = randn(10^7);
@time correl_coef(x,y)

x = randn(10^8); y = randn(10^8);
@time correl_coef(x,y)