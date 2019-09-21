#==
We generate a sequence of numbers as follows: given a positive integer x, if it is even, then the next number in the sequence is x/2, otherwise it is 3x + 1. That is, we start with some x0 and then iterate xn+1 = f(xn)
==#

function collatz(x::Int)
    step = 0
    while x != 1
        if x % 2 == 0
            x = Int(x / 2)
        else
            x = 3x + 1
        end
        step += 1
    end
    step
end

#julia> ans1 = collatz_val_with_step(3)
#([3, 10, 5, 16, 8, 4, 2, 1], 7)
function collatz_val_with_step(x::Int)
    step = 0
    l = Int64[]
    push!(l, x)
    while x != 1
        if x % 2 == 0
            x = Int(x / 2)
            push!(l, x)
        else
            x = 3x + 1
            push!(l, x)
        end
        step += 1
    end
    return (l, step)
end

function collatz_limit_step(x::Int, lim::Int)
    step = 0
    l = Int64[]
    push!(l, x)
    while step != lim
        if x % 2 == 0
            x = Int(x / 2)
            push!(l, x)
        else
            x = 3x + 1
            push!(l, x)
        end
        step += 1
    end
    return (l, step)
end

function coll(x)
    if x % 2 == 0
        return Int(x / 2)
    end
    return 3x + 1
end

#julia> ans2 = collatz_coll(3)
#([3, 10, 5, 16, 8, 4, 2, 1], 7)
function collatz_coll(x::Int)
    step = 0
    l = Int64[]
    push!(l, x)
    while x != 1
        x = coll(x)
        push!(l, x)
        step += 1
    end
    return (l, step)
end
