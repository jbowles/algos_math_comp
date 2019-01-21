using Printf
const BASE_RES = 10

mutable struct Result
    xs::Array{Float64}
    ys::Array{Float64}
end

function interp(y0, y1, x0, x1, x)
    y0 + (x - x0) * (y1 - y0) / (x1 - x0)
end

function interpolate(xFit, yFit, n)
    scap = 10 * (n - 1) #90
    xs = Array{Float64}(undef, scap)
    ys = Array{Float64}(undef, scap)
    for i = 1:(n - 1)
        icount = 0
        for j = 1:10
            delta = xFit[i + 1] - xFit[i]
            idx = (10 * icount) + j
            xs[idx] = xFit[i] + 0.1 * float(j) * delta
            ys[idx] = interp(yFit[i], yFit[i + 1], xFit[i], xFit[i + 1], xs[idx])
        end
        icount += 1
    end
    Result(xs, ys)
end

function run()
    xsi = Array{Float64}(undef, BASE_RES)
    ysi = Array{Float64}(undef, BASE_RES)

    for i = 1:BASE_RES
        xsi[i] = 4.0 * π * float(i) / float(BASE_RES)
        ysi[i] = sin(xsi[i])
    end

    res = interpolate(xsi, ysi, BASE_RES)

    
    for i = 1:(10 * (BASE_RES - 1))
        @printf("%e %e\n", res.xs[i], res.ys[i])
    end

end

#not correct
#julia linear_interpolation.jl > linplot_out
# ./plot_interp.py julia/linplot_out
run()