using SparseArrays

a = sparsevec([123456, 123457], [1.0, -1.0], 10^6);
println(a)
println(length(a))

b = randn(10^6); #ordinary non-sparse vec

for i in 1:2
  println("run", i)
  @time 2*a;
  @time 2*b;
  @time a'*b;
  @time b'*b;
  @time c = a+b;
  println()
end

