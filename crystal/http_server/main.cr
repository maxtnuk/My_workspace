require "http/server"

server = HTTP::Server.new(8000) do |context|
  context.response.content_type = "text/plain"
  context.response.print "hello i am in crystal server"
end

puts "Listening on http://127.0.0.1:8000"
server.listen
