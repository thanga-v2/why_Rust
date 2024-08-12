use std::time::Duration;
use anyhow::{anyhow, Result};
use tokio::io::{self,AsyncWriteExt};
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::timeout;
// we can't write an async function directly.

// we need a wrapper which can wrap our asynch main function

#[tokio::main]

// the '#' is a macro that can generate code for a wrapper.

async fn main() -> Result<()> {

    // Rust is heavily based on macros.

    println!("Hello, world!");

    let listener: TcpListener = TcpListener::bind("127.0.0.1:8083").await?; // '?' is an error handling operator. If the left side is negative, throw error else cont the execution
    println!("Server Running on 8083");

    let testlistenre = TcpListener::bind("127.0.0.1:8090").await?;
    println!("This for another test port 8090");

    // endless loop

    loop {
        let (mut socket, _) = listener.accept().await?; // PORT - 8083
        // when someone is connecting we need a dedicated thread -> tokio spawn.

        let (mut testsocket, _) = testlistenre.accept().await?; // PORT - 8090

        // Rust doesn't have a grabage collector
        // the variable memory gets free as soon as its move out of scope.

        // *move is using *socket**
        tokio::spawn( async move {
            println!("A new user connected");
            _ = socket.write_all(b"Hello thanga im awaiting as bytes").await;
            match user_loop(&mut socket).await {
                Ok(_) => println!("User Disconnected"),
                Err(e) => eprintln!("Error {}",e),
            }


        } // socket is moving out of scope *here*
                      // this makes sure tolerant for memory leaks.
        );


        // this is for test purpose

        tokio::spawn(async move{
            println!("a test user");
            _ = testsocket.write_all(b"hello test user").await;
        });

    }

    enum Input{
        Empty,
        Question(String)
    }
    async fn get_user_input(socket: &mut TcpStream) -> Result<Input> {
        let mut buffer = [0;1024]; // initialize an array of size 1024, starting with 0

        //Ok(Input::Empty)
        // lets do a method call
        let n = match timeout(Duration::from_secs(30), socket.read(&mut buffer)).await? {
            // match timeout(Duration::from_secs(30), socket.read(&mut buffer)).await
            // if user doesn't type anything in 30 sec, we want to have an error.
            Ok(0) => return Err(anyhow!("Connection closed")), // 0 bytes
            Ok(1) if buffer[0] == b'\n' => return Ok(Input::Empty), // 1 bytes
            Ok(2) if buffer[0] == b'\r' && buffer[1] == b'\n' => return Ok(Input::Empty), // 2 bytes
            Ok(n) => n,
            Err(e) => return Err(anyhow!("Failed to read from socket : {}", e)),
        };

        let input = String::from_utf8_lossy(&buffer[..n]);
        // Ok(Input::Question(input.to_string()))

        // or

        Ok(Input::Question(input.into_owned()))
    }

    async fn user_loop(socket: &mut TcpStream) -> Result<()>{

        // this will be an interaction with user

        // input something, output something
        // input something, output something

        loop {
            let input = get_user_input(socket).await?;
            match input{
                Input::Empty => continue,
                Input::Question(question) => {
                    let response = format!("Hey You asked: {}", question);
                    // the above question will get send it back to the user using below line.

                    // hence user will get an echo back on his question
                    socket.write_all(response.as_bytes()).await?;
                }
            }
        }

        Ok(())
    }

   Ok(())   // if its success return nothing else return the error
}
