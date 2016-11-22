def yield_an_arg
    puts 'Yielding 10!'
    yield(10)
    puts 'finished'
end
yield_an_arg { |x| puts "here is the code #{x}" }
