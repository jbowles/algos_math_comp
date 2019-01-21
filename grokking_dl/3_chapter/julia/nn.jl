using LinearAlgebra

function neural_network(input, weight)
    prediction = input * weight
    prediction
end
function version1()
    weight = 0.1
    number_of_toes = [8.5 9.5 10 9]
    input = number_of_toes[1]
    pred = neural_network(input, weight)
    println(pred)
end

w_sum(a, b) = sum(a .* b)
function neural_network_n(input, weights)
    w_sum(input, weights)
end
function version2()
    toes = [8.5 9.5 9.9 9.0]
    wlrec = [0.65 0.8 0.8 0.9]
    nfans = [1.2 1.3 0.5 1.0]
    weights = [0.1 0.2 0.0]
    input = [toes[1] wlrec[1] nfans[1]]
    pred = neural_network_n(input, weights)
    println(pred)
end

function neural_network_dot(input, weights)
    dot(input, weights)
end
function version3()
    toes = [8.5 9.5 9.9 9.0]
    winlossrec = [0.65 0.8 0.8 0.9]
    nfans = [1.2 1.3 0.5 1.0]
    weights = [0.1 0.2 0.0]
    input = [toes[1] winlossrec[1] nfans[1]]
    pred = neural_network_dot(input, weights)
    println(pred)
end

function neural_network_ele_for(number::Float64, vector::Array{Float64,2})
    output = [0. 0. 0.]
    for i = 1:length(vector)
        output[i] = number * vector[i]
    end
    output
end
function version0_4_1() 
    pred = neural_network_ele_for(0.65, [0.3 0.2 0.9])
    println(pred)
end

function neural_network_ele_mul(number::Float64, vector::Array{Float64,2})
    ele_mul(number, vector)
end
ele_mul(a::Float64, b::Array{Float64,2}) = (a * b)
function version4() 
    pred = neural_network_ele_mul(0.65, [0.3 0.2 0.9])
    println(pred)
end

function vect_mat_mul(vect::Array{Float64,2}, matrix::Array{Array{Float64,2},1})
    @assert(length(vect) == length(matrix))
    output = [0. 0. 0.]
    for i in 1:length(vect)
        output[i] = w_sum(vect, matrix[i])
    end
    output
end
function neural_network_vect_mat_mul(input, weights)
    vect_mat_mul(input, weights)
end
function version5()
    toes = [8.5 9.5 9.9 9.]
    winlossrec = [0.65 .8 .8 .9]
    nfans = [1.2 1.3 .5 1.]
    input = [toes[1] winlossrec[1] nfans[1]]
    weights = [
        [.1 .1 -.3], # hurt?
        [.1 .2 .0],  # win?
        [.0 1.3 .1], # sad?
    ]
    pred = neural_network_vect_mat_mul(input, weights)
    print(pred)
end


version1()
version2()
version3()
version0_4_1()
version4()
version5()


##################################################
############## playing with some inbounds ########
##################################################
function axpy!(c::Array, a::Array, b::Array)
    @assert length(a) == length(b) == length(c)
    @inbounds for i in 1:length(a)
        c[i] = a[i] * b[i]
    end
end
function version8()
    wlrec = [0. 0. 0. 0.]
    nfans = [1.2 1.3 0.5 1.0]
    weights = [0.1 0.2 0.0 0.4]
    axpy!(wlrec, nfans, weights)
    println(wlrec)
end
version8()

#V64 = Vector{Float64}
#print LLVM IR
#code_llvm(axpy!, (V64, V64, V64))
#print assembly instructions
#code_native(axpy!, (V64, V64, V64))