a = randn(10^5); b = randn(10^5);
@time a'*b
@time a'*b
@time b'*b

c = randn(10^6); d = randn(10^6);
@time c'*d
@time c'*d
@time d'*c
@time c'*c
