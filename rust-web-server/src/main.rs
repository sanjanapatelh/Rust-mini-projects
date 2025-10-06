
use std::net::{ TcpListener , TcpStream }; // Networking module 
use std::io::{ self , prelude :: *}  ;
use std::fs; 


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

    if let Err(e) = try_handle_connection(&mut stream, &mut buffer ) {
        eprintln!("Error handling connection: {}",e);
    }

    let response = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n<h1>500 Internal Server Error</h1>";

    stream.write_all(response.as_bytes()).unwrap_or_else(|write_err|{
        eprintln!("Failed to write 500 response:{}",write_err);
    });

}

fn try_handle_connection( stream : &mut TcpStream , buffer: &mut[u8]  ) -> io::Result<()> {
    eprintln!("Connection established");

    // Read the request 
    stream.read(buffer)?;

    // Error with the borrowing 
    let request_str = String::from_utf8_lossy(buffer);

    let request_line = request_str.lines().next().unwrap_or("");

    // Basic routing 

    let ( status_line , filename) = if request_line.starts_with("GET / HTTP/1.1"){
         ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(filename)?;

    // Formating http response 

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );

    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    eprintln!("Request handled . Repsonse sent.");

    Ok(())


}
