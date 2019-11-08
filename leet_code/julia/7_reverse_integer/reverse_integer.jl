function reverse(x)
    rev = 0
    while x != 0
        pop = x % 10
        # x = trunc(Int32, x/10)
        x = trunc(Int32, x/10)
        rev = rev*10+pop
    end
    if rev > typemax(Int32) || rev < typemin(Int32)
        return 0
    end
    return rev
end

println(reverse(123)
println(reverse(-123))
println(reverse(120))
println(reverse(1534236469))
println(reverse(-2147483648))