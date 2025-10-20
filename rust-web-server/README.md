
>> Step 1 : " Hello Socket " - single Threaded
1. bind a TCP socket on 127.0.0.1:7878
2. accept one connection at a time
3. print the request’s first line
4. send a valid minimal HTTP/1.1 response

>> cargo run 
>>  curl 127.0.0.1:7878  

>> Step 2 : Request Routing and Response Logic

>> Step 3 : Concurrent Request 
    Handles multiple request 

    ThreadPool: Manage a fixed number of thread threads
    Arc (Atomic Reference Counting) : shared ownership
    Mutex<T> for mutual exclusion
    move : closures to guarantee ownership transfer of the connection stream to the execution thread.

>> Step 4 : Shutdown 
    1. Accepts just 2 connections 
    2. Shut down after 2 connections 
    JoinHandle::join() : wait for all worker threads to finish their current task before the main process exits cleanly.

    cargo run 
    curl http://127.0.0.1:7878/ ( 2 separate terminal )
    Shutdown after 2 request 


