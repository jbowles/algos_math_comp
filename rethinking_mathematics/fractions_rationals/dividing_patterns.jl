#==
julia> div_increments(4,5)
20-element Array{Tuple{String,Float64},1}:
 ("1/1", 1.0)               
 ("2/1", 2.0)               
 ("3/1", 3.0)               
 ("4/1", 4.0)               
 ("5/1", 5.0)               
 ("1/2", 0.5)               
 ⋮       
==#
function print_div_increments(num_max, denom_max)
    return [("$i/$j", i / j) for j in 1:num_max for i in 1:denom_max]
end

function div_increments(num_max, denom_max)
    return [i / j for j in 1:num_max for i in 1:denom_max]
end

#==
julia> div_fraction_by_integer(3,10,3)
3 / 10 ÷ 3 == 0.09999999999999999
but also (3/..) 10 * 3 == 3 /30 == 0.09999999999999999
==#
function div_fraction_by_integer(num, denom, divisor)
    # 1/7 ÷ 4 == 1/28; same as mulitply denominator with integer 7*4 and just keeping the numerator
    # more precise: same as 1/7 ÷ 4/1; multiply first denominator with second numerator, and keep the first numerator
    ans = num / denom / divisor
    println("$num / $denom ÷ $divisor == ", ans)
    println("but also ($num/..) $denom * $divisor == $num /$(denom * divisor) == $ans")
end