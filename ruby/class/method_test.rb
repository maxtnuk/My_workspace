module Test
    def hello
        puts 'hello module test'
    end
end
class Base
    include Test
    def hello
        puts 'hello class base'
        super
    end
end
class Sub < Base
    puts self
    def hello
        puts 'hello class sub'
        super
    end
    include Test
    def print_self
        puts self
    end
end

halo = Sub.new
halo.hello
halo.print_self
