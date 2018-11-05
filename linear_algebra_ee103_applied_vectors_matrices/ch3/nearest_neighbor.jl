using LinearAlgebra

# nearest neighbor in z for x; solve for x
nearest_neighbor(x,z) = z[argmin([norm(x-y) for y in z])]
z = ( [2,1], [7,2], [5.5,4], [4,8], [1,5], [9,6] )

one = nearest_neighbor([5,6], z)
println(one) #-> [5.5, 4.0]
two = nearest_neighbor([3,3], z)
println(two) #-> [2,1]