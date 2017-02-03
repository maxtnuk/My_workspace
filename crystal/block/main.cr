def print_string(&block : -> Int32)
  val = block.call
  puts val
end

print_string { 1 }

struct Int
  def times
    i = 0
    while i < self
      yield i
      i += 1
    end
  end
end

def print_yield
  10.times { |count|
    puts "here is number " + count.to_s
  }
end

print_yield

def twice
  yield
  yield
end

twice do
  print_string { 1 }
end
