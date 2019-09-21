function isPrime(x::Int)
    prime = true
    for i in 2:ceil(x / 2)
        # print("inloop... ")
        factor = ceil(x / i)
        if factor * i == x
            println("found a factor", factor)
            prime = false
            break
        end
    end
    return prime
end
