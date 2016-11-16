puts "test file function"
c= gets
puts "you just put this:" + c
file_put =File.new("test.out","w")
file_put.puts c
file_put.close
