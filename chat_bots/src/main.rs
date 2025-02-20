use anyhow::{anyhow, Error, Result};
use async_openai::{
    types::{
        ChatCompletionRequestMessage , ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestAssistantMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs, Role
    },
    Client,
};

use tokio::{
    io::{ AsyncWriteExt,AsyncReadExt },
    net::{TcpListener, TcpStream},
    time::{timeout,Duration}
};

use std::fmt::Display;
use async_openai::config::OpenAIConfig;
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

        match updated_user_loop_open_ai(&mut socket).await {
            Ok(client) => println!("Client {:?}",client),
            Err(e) => eprintln!("Err"),
        }

        match openai_user_loop(&mut socket).await{
            Ok(msg) => println!("The prompt rightly called"),
            Err(e) => eprintln!("There seem to be an error"),
        };

        // *move is using *socket**
        tokio::spawn( async move {
            println!("A new user connected");
            _ = socket.write_all(b"Hello thanga im awaiting as bytes").await;
            match user_loop(&mut socket).await {
                Ok(_) => println!("User Disconnected"),
                Err(e) => eprintln!("Error {}",e),
            }

            match updated_user_loop_open_ai(&mut socket).await {
                Ok(_) => println!("Client got connected inside the spawn"),
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

    async fn updated_user_loop_open_ai(socket : &mut TcpStream) -> Result<Client<OpenAIConfig>, Error>{

        let client = Client::new();

        println!("Show what's inside client object {:?}",&client);

        Ok(client)
    }

    async fn openai_user_loop(socket : &mut TcpStream) -> Result<()>{

        // INITIALIZE THE OPEN AI CLIENT
        let client = Client::new();

        let prompt = "You are a helpfull assistant for thangaraj";

        // let message = ChatCompletionRequestSystemMessageArgs::default()
        //     .content(prompt)
        //     .build()?;
        //
        // let mut messages = vec![message];

        // let mut messages: Vec<ChatCompletionRequestSystemMessageArgs> = vec![
        //     ChatCompletionRequestSystemMessageArgs::default()
        //         .content(prompt.to_string())
        //         .build()?
        // ];

        // Create the system message directly as a ChatCompletionRequestMessage
        let system_message = ChatCompletionRequestSystemMessageArgs::default()
            .role(Role::System)
            .content(prompt.to_string())
            .name: None;


        // Create a vector to hold all the messages
        let mut messages: Vec<ChatCompletionRequestMessage> = vec![system_message];


        println!("Here is the message {:?}",messages);

        // messages.push(message);

        loop {
            // first get the user input
            let input = match get_user_input(socket).await? {
                Input::Question(input) => input,
                Input::Empty => return Ok(())
            };

            let user_message = ChatCompletionRequestSystemMessageArgs::default()
                .role: Role::User
                .content(input.to_string())
                .name: None;


            // putting together the payload
            // creating a request
            let request = CreateChatCompletionRequestArgs::default()
                .model("gpt-4o")
                .messages(messages.clone())
                .build()?;


        }



        Ok(())
    }

   Ok(())   // if its success return nothing else return the error
}
