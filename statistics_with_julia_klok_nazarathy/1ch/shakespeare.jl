using HTTP, JSON

data = HTTP.request("GET", "https://ocw.mit.edu/ans7870/6/6.006/s08/lecturenotes/files/t8.shakespeare.txt")

shakespeare = String(data.body);
shakespeareWords = split(shakespeare);

println("typeof shakespeare words", typeof(shakespeareWords))
println("size of shakespeare words", size(shakespeareWords))

jsonWords = HTTP.request("GET", "https://raw.githubusercontent.com/" *
"h-Klok/StatsWithJuliaBook/master/1_chapter/jsonCode.json");


parsedJsonDict = JSON.parse(String(jsonWords.body))
println("typeof parsedJsonDict", typeof(parsedJsonDict))

keywords = Array{String}(parsedJsonDict["words"])
println("size of keywords", size(keywords))
numberToShow = parsedJsonDict["numToShow"]
wordCount = Dict([(x, count(w->lowercase(w) == lowercase(x), shakespeareWords)) for x in keywords])
sortedWordCount = sort(collect(wordCount), by = last, rev = true)
sortedWordCount[1:numberToShow]
