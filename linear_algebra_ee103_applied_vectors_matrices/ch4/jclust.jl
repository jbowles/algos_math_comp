using LinearAlgebra, VMLS

"""
store the list of vectors in a Julia list or tuple of N vectors. 
If we call this list x, we can access the ith entry (which is a vector) using x[i]

specify the clusters or group membership, we can use a list of assignments 
called assignment, where assignment[i] is the number of the group that vector x[i]

store the k cluster representatives as a Julia list called reps
with reps[j] the jth cluster
"""

function jclust_debug(x,reps,assignment)
    for i in 1:length(x)
        println("i = ", i)
        println("x[i] = ", x[i])
        println("reps assg i = ", reps[assignment[i]])
        k = x[i]-reps[assignment[i]]
        println("k = ", k)
        println("norm(k) = ", norm(k))
        println("norm(k)^2 = ", norm(k)^2)
        println("")
    end
end

Jclust(x,reps,assignment) = avg([
    norm(
            x[i]-reps[assignment[i]]
        )^2 for i=1:length(x)
    ])
x = [[0,1],[1,0],[-1,1]]
reps = [[1,1],[0,0]]
assignment = [1,2,1]

jclust_debug(x,reps,assignment)
println(Jclust(x,reps,assignment))