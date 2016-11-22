class Test
    attr_writer :write_me
    attr_accessor :access_me
    attr_reader :read_me
    def setting
        @read_me = 10
    end

    def print
        puts "#{@write_me},#{@access_me},#{@read_me}"
    end
end
str = Test.new
str.setting
str.access_me = 20
str.write_me = 30
str.print
