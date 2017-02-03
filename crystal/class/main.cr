class Foo
  def finalize
    # Invoked when Foo is garbage-collected
    puts "Bye bye from #{self}!"
  end
end

# Prints "Bye bye ...!" for ever
Foo.new
