#==
a1 =  0,0,1  , a2 = 1/√2  1, 1, 0  , a3 = 1/√2  1, −1, 0  ,

form an orthonormal basis, and check the expansion of x = (1, 2, 3) in this basis,

a transpose sub 1
x = ( aT_1 x ) a 1 + · · · + ( aT_n x ) a n .
==#

a1 = [0,0,-1]; a2 = [1,1,0]/sqrt(2); a3 = [1,-1,0]/sqrt(2);

# norms are all 1
norm(a1), norm(a2), norm(a3)

# no linear combinations, they are independent
a1'*a2, a1'*a3, a2'*a3

x = [1,2,3]

#get coefficients of x in orthonormal basis
beta1 = a1'*x; beta2 = a2'*x; beta3 = a3'*x;
#expansion of x in basis
xexp = beta1*a1 + beta2*a2 + beta3*a3

#==
The following is a Julia implementation of Algorithm 5.1
in VMLS (Gram–Schmidt algorithm). It takes as input an
array [ a[1], a[2], ..., a[k] ],
containing the k vectors a1, . . . , ak.
If the vectors are linearly independent,
it returns an array [ q[1], ..., q[k] ] with the orthonormal
set of vectors computed by the Gram– Schmidt algorithm.
If the vectors are linearly dependent and the Gram–Schmidt
algorithm terminates early in iteration i, it returns the array [ q[1], ..., q[i] ] of length i.
==#

function my_gram_schmidt(a; tol = 1e-10)
    #result
    q = []
    #loop through multi-dim vector
    for i = 1:length(a)
        #first vector is q
        qtilde = a[i]
        #loop through n-e elements in the first vector
        for j = 1:i-1
            #println("i==", i)
            #println("a[i]=>", a[i])
            #println("q=>", q)
            qtilde -= (q[j]' * a[i]) * q[j]
        end
        if norm(qtilde) < tol
            println("Vectors are linearly dependent")
            return q
        end
        push!(q, qtilde/norm(qtilde))
    end
    return q
end

a = [[-1, 1, -1, 1], [-1, 3, -1, 3], [1, 3, 4, 7]]
qres = my_gram_schmidt(a)
# test orthonormality
# norms should all be 1
norm(qres[1])
norm(qres[2])
norm(qres[3])
# linear combinations should be be 0
qres[1]'*qres[2]
qres[1]'*qres[3]

## early termination case can be done using linear combination of on the elements above
#b = [ a[1], a[2], a[1] .* a[2] ]
#b = [ a[1], a[2], a[1] + a[2] ]
b = [ a[1], a[2], 1.3*a[1] + 0.5*a[2] ]
early_terminate = my_gram_schmidt(b)


##INDEPENDENC DIMENSION INEQUALITY
## any 3 vector of 2-element vectors must be dependent. e.g. basis requires space of vecotors to be more the size of sub-vectors
three_two_vectors = [ [1,1], [1,2], [-1,1] ]
ttv = gram_schmidt(three_two_vectors)
