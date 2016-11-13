use std::io::prelude::*;
use std::net::{TcpStream,TcpListener,Ipv4Addr};

pub fn chat_server(){
    let ip=Ipv4Addr::new(127,0,0,1);
    let port = 3090;
    let tcp_s=TcpListener::bind((ip,port)).unwrap();
    for stream in tcp_s.incoming(){
            let mut stream= stream.unwrap();
            let mut buffer=[0;10];
            stream.read(&mut buffer).expect("read fail");
            println!("you just put this {}",
            buffer.iter().map(|x|x.to_string()).collect::<String>());
            stream.write(b"hello").unwrap();
    }
}
pub fn chat_client(){
    let ip=Ipv4Addr::new(127,0,0,1);
    let port = 3090;
    let input=&[1,2,3,4];
    let mut tcp_c=TcpStream::connect((ip,port)).unwrap();
    tcp_c.write(input).unwrap();
    let mut result =[0;4];
    tcp_c.read(&mut result).unwrap();
    println!("oh here is result{}",
    result.iter().map(|x|x.to_string()).collect::<String>());
}
