"""
  Suppose that the 100-vector x gives the age distribution of some population, with x_i the number of people of age i − 1, for i = 1, . . . , 100. The total number of people with age between 5 and 18 (inclusive) is given by
    x_6 +x_7 +···+x_18 +x_19.

express as s^T*x; where s vector of i = 6,...,19; zero otherwise.
"""

#sparse 100-vector with 6,...,19 filled
s = [zeros(5); ones(14); zeros(81)]
school_age_population = s'*0.5

number_of_students = sum(s[6:19])
