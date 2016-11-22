n = gets.to_i
begin
    result = 100 / n
rescue
    puts 'cause error'
    exit
end
puts result
require 'date'
d = Date.today
puts d
puts Date.parse('April 24 1705').england.strftime('%B %d %Y')
