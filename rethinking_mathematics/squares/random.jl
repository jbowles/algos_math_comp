function for_square(upper)
    r = rand(1:upper)
    println("what is √", r, "?")
    return r
end

function answer_square(root)
    println("√", root, " = ", sqrt(root))
end

function timed_quiz(;number_of_questions = 5, upper_limit = 100, timer_seconds = 5)
    for i in 1:number_of_questions
        print("Q$i:  ")
        r = for_square(upper_limit)
        sleep(timer_seconds)
        answer_square(r)
        println(" ")
        sleep(5)
    end
end