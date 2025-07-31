function main()
    num_prompt = "Enter a number:"
    number = validate_input(Int, num_prompt)
    println("You entered $number")
    fizzbuzz(number)
end

function validate_input(input_type, prompt)
    res = nothing
    while res == nothing
        print("$prompt ")
        input = readline()
        try
            res = parse(input_type, input)
        catch err
            println("$err")
        end
    end
    return res
end

function fizzbuzz(upper_limit)
    subs = Dict(3 => "fizz", 5 => "buzz")
    for i in 1:upper_limit
        output = ""
        for key in keys(subs)
            if i % key == 0
                output *= subs[key]
            end
        end
        println(isempty(output) ? i : output)
    end
end

if abspath(PROGRAM_FILE) == @__FILE__
    main()
end