function main()
    a, b, c = validate_input(Int, "Please enter three numbers separated by spaces:")
    triangle_type(a, b, c)
end

function validate_input(input_type, prompt)
    res = nothing
    while res == nothing
        print("$prompt ")
        input = readline()
        nums = split(input)

        if length(nums) != 3
            continue
        end

        try
            parsed = [parse(input_type, num) for num in nums]
            res = parsed
        catch
            println("All values must be integers.")
        end
        res = nums
    end
    return res
end

function triangle_type(side_a, side_b, side_c)
    if side_a == side_b && side_b == side_c
        println("Equilateral")
    elseif side_a == side_b || side_b == side_c || side_c == side_a
        println("Scalene")
    else
        println("Isoceles")
    end
end

if abspath(PROGRAM_FILE) == @__FILE__
    main()
end