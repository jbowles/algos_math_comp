
# O(k) where k = number of bits that are 1
function parity(x)
    println("bitstring == ", bitstring(x))
    result = 0
    while x > 0
        println("x =", x)
        result ⊻= 1
        x &=(x-1) #drop lowest bit set
    end
    return result
end
