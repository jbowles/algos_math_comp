#=
We now ask what is the probability of logging an event with a random password? 
Denote the event of logging a password A. In this case, it turns out to be much 
more convenient to consider the complement, A^c := Ω \ A, which is the event of 
having 0 character matches. We have that |Ac| = 61^8 because given any (arbitrary) 
correct password, there are 61 = 62 − 1 character options for each character, 
in order ensure A^c holds. Hence,

    P(A^c) = 61^8/62^8 ≈ 0.87802

We then have that the probability of logging an event is 

    P(A) = 1 − P(A^c) ≈ 0.12198. So if, 

for example, 107 login attempts are made, we can expect that 
about 1.2 million login attempts would be written to the security log. 
We now simulate such a scenario

=#

using Random 
Random.seed!()
N = 10^7

passLength, numMatchesForLog = 8,1
possibleChars = ['a':'z' ; 'A':'Z' ; '0':'9']

correctPassword = "3xyZu4vN"

numMatch(loginPassword) = sum([loginPassword[i] == correctPassword[i] for i in 1:passLength])

passwords = [String(rand(possibleChars, passLength)) for _ in 1:N]
numLogs = sum([numMatch(p) >= numMatchesForLog for p in passwords])


println("Number of login attempts logged: ", numLogs)
println("Proportion of login attempts logged: ", numLogs/N)
