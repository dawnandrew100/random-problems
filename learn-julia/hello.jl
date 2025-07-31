# My first Julia program
function main()
    println("Hello, world!")
end

if abspath(PROGRAM_FILE) == @__FILE__
    main()
end