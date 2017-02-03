condition = false
answer = condition ? 1 : "hello"
puts typeof(answer)
if answer.is_a?(Number)
  puts "this is the number " + answer.to_s
else
  puts "this is the string " + answer
end

def showit(*args : Int32 | String | Bool)
  puts typeof(args)
  args.each { |value|
    case value
    when .is_a?(Number)
      puts "this is a number " + value.to_s
    when .is_a?(String)
      puts "this is a string " + value
    else
      puts "wrong insert "
    end
  }
end

showit 1, 2, 4, true, "hello", 2, "fuck"
