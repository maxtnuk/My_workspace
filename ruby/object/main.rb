obj = Object.new
def obj.tell_temperature(temp)
    temp * temp
end
puts "here is the result #{obj.tell_temperature(9)}"
string_result = obj.tell_temperature(10).to_s
puts string_result
def obj.vailable?
    false
end
puts obj.vailable?
puts obj.object_id
puts 100.object_id
puts 'hello'.object_id
if obj.respond_to?('fuck')
    obj.fuck
else
    puts "you just put this method #{'fuclk'.object_id}"
end
print 'insert method what you want: '
request = gets.chomp
if obj.respond_to?(request)
    obj.send(request)
else
    puts "you just put the wrong method #{request}"
end
