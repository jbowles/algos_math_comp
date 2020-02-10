using HTTP, JSON

file_loc() = ENV["HOME"]* "/x/training_data/shakespeare/shakes_total_clean.txt"
ok = isfile(file_loc())
if ok 
    shakes = String(read(file_loc()))
else
    # gutenberg shakespeare
    shakes = String(HTTP.request("GET", "https://ocw.mit.edu/ans7870/6/6.006/s08/lecturenotes/files/t8.shakespeare.txt").body)
end;



shakesWords = split(shakes);

println("typeof shakespeare words", typeof(shakesWords))
println("size of shakespeare words", size(shakesWords))
#retrive these words
"""
    {
    "words": [ "heaven","hell","man","woman","boy","girl","king","queen",
        "prince","sir","love","hate","knife","english","england","god"],
    "numToShow": 5
    }
"""
jsonWords = HTTP.request("GET", "https://raw.githubusercontent.com/" * "h-Klok/StatsWithJuliaBook/master/1_chapter/jsonCode.json");


parsedJsonDict = JSON.parse(String(jsonWords.body))
println("typeof parsedJsonDict", typeof(parsedJsonDict))

keywords = Array{String}(parsedJsonDict["words"])
println("size of keywords", size(keywords))
numberToShow = parsedJsonDict["numToShow"]
wordCount = Dict([(x, count(w->lowercase(w) == lowercase(x), shakesWords)) for x in keywords])
sortedWordCount = sort(collect(wordCount), by = last, rev = true)
sortedWordCount[1:numberToShow]
