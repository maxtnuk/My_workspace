require './stacklike.rb'
class Stack
    include Stacklike
end

test = Stack.new
test.add_to_stack(10)
puts test.stack
