class Hello
    def event
        'hey what are you doing i am a class'
    end
end
class Hello
    def fuck
        'and i am also class hello method'
    end
end
class Hello
    def inst_var_init(value)
        puts 'Setting an instance variable'
        @ivar = value
    end

    def inst_var_report
        puts 'inspecting the value of the instance variable'
        puts @ivar
    end
end
test = Hello.new
puts test.event
puts test.fuck
puts Time.new.strftime('%m-%d-%y')
request = gets.chomp
if test.respond_to?(request)
    test.send(request, 10)
else
    puts 'you just input the wrong method'
end
test.inst_var_report
