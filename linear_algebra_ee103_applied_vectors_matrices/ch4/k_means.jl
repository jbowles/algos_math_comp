using VMLS, LinearAlgebra

"""
where x is an array of N vectors and k is the number of groups.

The first output argument is an array of N integers, containing the computed group assignments (integers from 1 to k).

The second output argument is an array of k vectors, with the k group representatives.
We also include two optional keyword arguments, with a limit on the number of iterations and a tolerance used in the stopping condition.

1. Initialization.
-----
 As discussed in VMLS (page 76), the k-means algorithm can start from a random initial choice of representatives, or from a random assignment of the points in k groups. In this implementation, we use the second option. The Julia function rand(1:k) picks a random number from the set 1:k, i.e., the integers 1,...,k. On line 11 we create an array assignment of N elements, with each element chosen by calling rand(1:k).

        assignment = [rand(1:k) for i in 1:N] #N-element Array{Int64,1}
-----

2. Updating group representatives.
------
Update the k group representa- tives. We find the indexes of the points in cluster j and collect them in an array group. The expression x[group] on line 19 constructs an array from the subset of elements of x indexed by group. The function sum computes the sum of the elements of the array x[group]. Dividing by the number of elements length(x[group]) gives the average of the vectors in the group. The result is jth the group representative. This vector is stored as the jth element in an array reps of length N.

        for j = 1:k
            group = [i for i=1:N if assignment[i] == j]
            reps[j] = sum(x[group]) / length(group)
        end
-------

3. Updating group assignments.
-------
Update the group assignments. The Julia function findmin computes both the minimum of a sequence of numbers and the position of the minimum in the sequence. The result is returned as a 2- tuple. We apply findmin to the array of k distances of point x[i] to the k representatives. We store the distance to the nearest representative in distances[i], and the index of the nearest representative (i.e., the new assignment of point i) in assignment[i].

        (distances[i], assignment[i]) = findmin([norm(x[i] - reps[j] for j = 1:k)])
-------

4. Clustering objective.
-------
Compute the clustering objective Jclust (equation (4.1) in VMLS) as the square of the RMS value of the vector of distances.

        J = norm(distances)^2 / N
-------

5. Convergence.
-------
We terminate the algorithm when the improvement in the clustering objective becomes very small

        if iter > 1 && abs(J - Jprevious) < tol * J
            return assignment, reps
        end
-------
"""

function mykmeans(x, k; maxiters = 100, tol = 1e-5)
    N = length(x)
    n = length(x[1])
    distances = zeros(N) #store distance of each point to nearest representative
    reps = [zeros(n) for j=1:k] #store representatives: n-element Array{Array{Float64,1},1}
    #assignment is N-vector value of integer in 1:k
    #initial assignment is chosen randomly
    assignment = [rand(1:k) for i in 1:N] #N-element Array{Int64,1}
    #stopping condition
    Jprevious = Inf
    for iter = 1:maxiters
        #cluster j representative is average of points in cluster j
        for j = 1:k
            #find indexes of points in cluster for j, collect values in 'group' array
            group = [i for i=1:N if assignment[i] == j]
            #set reps[j] to the constructed array from subset of elements of x
            #indexed by 'group', summed and divided by 'group' length
            reps[j] = sum(x[group]) / length(group)
        end
        #each x[i] find distance to nearest representative and its group index
        for i = 1:N
          (distances[i], assignment[i]) = findmin([norm(x[i] - reps[j]) for j=1:k])
        end

        #compute clustering objective
        J = norm(distances)^2 / N

        #show progress and terminate if J stops descreasing
        #println("Iteration ", iter, ": Jclust = ", J, ".", ", reps typeof = ", typeof(reps), ", reps = ", reps)
        println("Iteration ", iter, ": Jclust = ", J, ".")
        println("")
        if iter > 1 && abs(J - Jprevious) < tol * J
            return assignment, reps
        end
        Jprevious = J
    end
end

k = 3
X = vcat([0.3*randn(2) for i = 1:100], [[1,1] + 0.3*randn(2) for i = 1:100], [[1,-1] + 0.3*randn(2) for i = 1:100])


ment, rep = mykmeans(X, k)


#==
using Plots
scatter([x[1] for x in X], [x[2] for x in X])
plot!(legend = false, grid = false, size = (500,500),xlims = (-1.5,2.5), ylims = (-2,2))
grps  = [[X[i] for i=1:length(X) if ment[i] == j] for j=1:k]
scatter([c[1] for c in grps[1]], [c[2] for c in grps[1]])
scatter!([c[1] for c in grps[2]], [c[2] for c in grps[2]])
scatter!([c[1] for c in grps[3]], [c[2] for c in grps[3]])
plot!(legend = false, grid = false, size = (500,500),xlims = (-1.5,2.5), ylims = (-2,2))
==#

#==
function mykmeans_debug(x, k; maxiters = 100, tol = 1e-5)
    N = length(x)
    println("k = ", k, ", N = ", N)
    println("")
    assignment = [rand(1:k) for i in 1:N]
    println("assignment = ", assignment)
    println("")
    println("")
    for iter = 1:maxiters
        for j = 1:k
            for i in 1:N
                println("at i = ", i, ", j = ", j, ", assignment[i] = ", assignment[i])
                if assignment[i] == j
                    println("push i: ", i)
                end
            end
        end
    end
end
==#
