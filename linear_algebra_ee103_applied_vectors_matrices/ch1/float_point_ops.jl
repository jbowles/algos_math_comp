
# For any two numbers a and b, we have (a+b)(a−b) = a^2 −b^2.
# floating point roundoff may return a very small difference

using Printf
for i in 1:4
  a = rand()
  b = rand()
  lhs = (a+b) * (a-b)
  rhs = a^2 - b^2
  @printf("a = %f, b = %f, lhs = %f, rhs = %f\n", a,b,lhs,rhs)
  println("for run ", i)
  println(abs(lhs - rhs))
  println()
end
