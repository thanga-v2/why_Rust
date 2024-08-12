use anyhow::Result;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

// we can't write an asynch function directly.

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
        let (mut socket, _) = listener.accept().await?;
        // when someone is connecting we need a dedicated thread -> tokio spawn.

        let (mut testsocket, _) = testlistenre.accept().await?;

        // Rust doesn't have a grabage collector
        // the variable memory gets free as soon as its move out of scope.

        // *move is using *socket**
        tokio::spawn( async move {
            println!("A new user connected");
            _ = socket.write_all(b"Hello thanga as bytes").await;
        } // socket is moving out of scope *here*
        );


        // this is for test purpose

        tokio::spawn(async move{
            println!("a test user");
            _ = testsocket.write_all(b"hello test user").await;
        });

    }

   Ok(())   // if its success return nothing else return the error
}
