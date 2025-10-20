
1. Project Setup and Basic Server: Create the project and handle single-threaded, synchronous TCP connections.

2. Request and Response: Implement basic HTTP request parsing and response generation.

3. Multithreading: Introduce a thread pool to handle multiple concurrent requests.

4. Graceful Shutdown (Advanced): Implement a way to shut down the server and thread pool cleanly. 


>> Step 1 : " Hello Socket " - single Threaded
1. bind a TCP socket on 127.0.0.1:7878
2. accept one connection at a time
3. print the request’s first line
4. send a valid minimal HTTP/1.1 response

>> cargo run 
>>  curl 127.0.0.1:7878  

>> Step 2 : Request Routing and Response Logic

>> Step 3 : Concurrent Request 

>> Step 4 : Shutdown 



