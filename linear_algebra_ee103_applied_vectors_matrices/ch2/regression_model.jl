"""
The regression model is the affine function of x given by f(x) = xTβ + v, where the n-vector β and the scalar v are the parameters in the model. The regression model is used to guess or approximate a real or observed value of the number y that is associated with x.
"""

#Paramaters
β = [148.73, -18.85]; v = 54.40;
y_hat(x) = x'*β + v

#Evaluate regression model
x = [0.846, 1]; y = 115
y_hat(x), y

x = [1.342, 2]; y = 234.5
y_hat(x), y

"""
julia> x = [0.846, 1]; y = 115;

julia> y_hat(x), y
(161.37557999999999, 115)

julia> x = [1.342, 2]; y = 234.5;

julia> y_hat(x), y
(216.29566000000003, 234.5)
"""

D = house_sales_data()
price = D["price"]
area = D["area"]
beds = D["beds"]

v = 54.4017;
β = [147.7521, -18.8534]
predicted = v .+ β[1] * area + β[2] * beds

using Plots
scatter(price, predicted, lims = (0,800));
plot!([0,800], [0,800], linestyle = :dash)
#make axes equal and add labels
plot!(xlims = (0, 800), ylims = (0, 800), size = (500, 500))
plot!(xlabel = "Actual price", ylabel = "Predicted Price")
