
use std::net::{ TcpListener , TcpStream }; // Networking module 
use std::io::prelude::* ;


fn main() {
   
   // 1. Bind to a port 
   // TcpListener::bind returns a Result 
   // The server will listen on 127.0.0.1 (localhost) on port 7878.

   let addr: String  = String::from( "127.0.0.1:7878" );
   let listener = TcpListener::bind(addr).unwrap();

   println!("Server listening on http://127.0.0.1:7878");

   // 2. Loop over incoming connections
   // .incomimg() gives an iterator that yeilds a Result<TcpStram , E> for each connection attemp.

   for stream in listener.incoming() {
    let stream = stream.unwrap(); // unwrap the result 
    
    // 3. Handling the stream 
    handle_connection(stream);

   }


}


// Function to hab=ndle s single connection 

fn handle_connection(mut stream: TcpStream){
    print!("connection established!");

    let mut buffer: [u8; 1024] = [0;1024];

    // .read returns Result<usize.E> => usize number of bytes read 
    // &mut - muttable reference -> allows modify buffer witout taking ownership.
    // unwrap => returns results (success) , error -> panic 
    stream.read(&mut buffer).unwrap();

    // Print the raw request data 
    println!("Request:\n{}" , String::from_utf8_lossy(&buffer[..]) );

}
