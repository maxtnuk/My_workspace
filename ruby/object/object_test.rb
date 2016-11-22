str = 'hello'
puts str
dude = str
bot = Object.new
def bot.test(x)
    puts x
end

def is_it_same(object1, object2)
    if object1.object_id == object2.object_id
        puts 'same object'
    else
        puts 'another object'
    end
end

dude.replace('fuck')
is_it_same(str, dude)
dude = 'hello'
is_it_same(str, dude)
bot.send('test', 10) if bot.respond_to?('test')
