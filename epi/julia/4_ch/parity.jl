
# O(k) where k = number of bits that are 1
function parity(x)
    println("bitstring == ", bitstring(x))
    result = 0
    while x > 0
        println("x =", x)
        result ⊻= 1 #xor result
        x &=(x-1) #drop lowest bit set
    end
    return result
end

function printShift(x)
    c = 0
    while x > 0
        println("x=", x)
        x >>= 1
        c+=1
    end
    println("x=", x)
    println("moves = ", c)
end

function printBase2BitString()
    println("Kilobyte: ", bitstring(1024))
    println("Megabyte: ", bitstring(1048576))
    println("Gigabyte: ", bitstring(1073741824))
    println("Terabyte: ", bitstring(1099511627776))
    println("Petabyte: ", bitstring(1125899906842624))
    println(" Exabyte: ", bitstring(1152921504606846976))
end
