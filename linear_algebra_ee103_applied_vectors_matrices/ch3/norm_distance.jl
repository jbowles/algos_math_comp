using LinearAlgebra
""" 
distance between two vectors is dist(x,y) = ||x-y||
euclidean norm of difference between x,y
can be done with norm(x-y)
"""

u = [1.8, 2.0, -3.7, 4.7]
v = [0.6, 2.1, 1.9, -1.4]
w = [2.0, 1.9, -4.0, 4.6]

t_norm = norm(u-v), norm(u-w), norm(v-w)
println(t_norm)
#-> (8.36779540858881, 0.3872983346207417, 8.532877591996735)
# u,w are much closer than others