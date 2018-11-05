a = rand(3);
println(a)

b = rand(3);
println(b)

beta = randn();
println(beta)

lhs = beta * (a + b);
println(lhs)

rhs = beta * a + beta * b;
println(rhs)
