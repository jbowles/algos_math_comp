using LinearAlgebra
"""
define function that computes angle between two vectors
call it ang, julia has funtion named angle for phase angle of complex 
"""

# ang returns radians, acos of x transpose times y, divided by the norm of x times the norm of y
ang(x,y) = acos( x'*y / (norm(x)*norm(y)) )
a = [1,2,-1]; b = [2,0,-3];
println("angle = ", ang(a,b))
println("angle in degrees ", ang(a,b)*(360/(2*pi)))