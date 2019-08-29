# julia --optimize=3 --check-bounds=no distance_among_pairs.jl
function distances_among_pairs(v)
    nin = length(v)
    nout = div(nin*nin - nin, 2)
    dists = Vector{eltype(v)}(undef, nout)
    k = 1 #position of output vector
    @inbounds for i in 1:(nin-1)
        a = v[i]
        @inbounds for j in (i+1):nin
            b = v[j]
            dists[k] = abs(a-b)
            k+=1
        end
    end
    dists
end

function run()
    v = rand(10_000)
    distances_among_pairs(v)
end

#warmup
t = rand(3)
distances_among_pairs(t)

using BenchmarkTools
using CPUTime
@benchmark run()
@time @CPUtime run()

const vals = rand(10_000)
@benchmark distances_among_pairs(vals)
@time @CPUtime distances_among_pairs(vals)


#without inline
#--------------
#minimum time:     261.228 ms (0.18% GC)
#median time:      304.784 ms (8.59% GC)
#mean time:        315.466 ms (12.33% GC)
#maximum time:     378.255 ms (28.41% GC)
#--------------
#samples:          16
#evals/sample:     1
