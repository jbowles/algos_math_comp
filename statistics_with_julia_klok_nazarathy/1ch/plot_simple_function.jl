using Plots, LaTeXStrings, Measures
pyplot()

f(x,y) = x^2 + y^2
f0(x) = f(x,0)
f2(x) = f(x,2)

xVals,yVals = -5:0.1:5, -5:0.1:5
Plots.plot(xVals, [f0.(xVals), f2.(xVals)], c=[:blue :red], xlims=(-5,5), legend=:top, ylims=(-5,25), ylabel=L"f(x,\cdot)", label=[L"f(x,0)" L"f(x,2)"])
p1 = Plots.annotate!(0, -0.2, text("(0,0) The minimum\n of f(x,0)", :left, :top, 10))

z = [f(x,y) for y in yVals, x in xVals]
p2 = Plots.surface(xVals, yVals, z, c=cgrad([:blue, :red]), legend=:none, ylabel="y", xlabel="x")

M = z[1:10,1:10]
p3 = heatmap(M, c=cgrad([:blue, :red]), yflip=true, ylabel="y", xlabel="x", xticks=([1:10;],xVals), yticks=([1:10;],yVals))

Plots.plot(p1,p2,p3, layout=(1,3), size=(1200,1400), xlabel="x", margin=5mm)
