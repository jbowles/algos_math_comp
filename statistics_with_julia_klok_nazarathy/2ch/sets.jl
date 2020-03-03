A = Set([2,7,2,3])
B = Set(1:6)
omega = Set(1:10)

MinA = minimum(A)
MaxB = maximum(B)

AunionB = union(A, B)
AintersectionB = intersect(A, B)
BdifferenceA = setdiff(B,A)
AdifferenceB = setdiff(A,B)
Bcomplement = setdiff(omega,B)
Acomplement = setdiff(omega,A)
AsymDifferenceB = union(setdiff(A,B),setdiff(B,A))


println("A = $A, B = $B")
println("MinimumA = $MinA, MaximumB = $MaxB")
println("A union B = $AunionB")
println("A intersection B = $AintersectionB")
println("B diff A = $BdifferenceA")
println("A diff B = $AdifferenceB")
println("B complement = $Bcomplement")
println("A complement = $Acomplement")
println("A symDifference B = $AsymDifferenceB")
println("The element ’6’ is an element of A: $(in(6,A))")
println("Symmetric difference and intersection are subsets of the union: ",
issubset(AsymDifferenceB,AunionB),", ", issubset(AintersectionB,AunionB))