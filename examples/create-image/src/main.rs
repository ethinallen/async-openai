use async_openai::{
    types::{CreateImageRequestArgs, ImageSize, ResponseFormat},
    Client,
};
use std::error::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    for (n,v) in env::vars() {
        println!("{}: {}", n,v);
    }
    // create client, reads OPENAI_API_KEY environment variable for API key.
    let client = Client::new();

    let request = CreateImageRequestArgs::default()
        .prompt("four lads sitting in the virtual ether. it's a water color of their souls floating in a bath of light. The abyss surrounds them and yet in each other's company the four of them sit vibrantly across from one another.")
        .n(2)
        .response_format(ResponseFormat::Url)
        .size(ImageSize::S512x512)
        .user("async-openai")
        .build()?;

    let response = client.images().create(request).await?;

    // Download and save images to ./data directory.
    // Each url is downloaded and saved in dedicated Tokio task.
    // Directory is created if it doesn't exist.
    let paths = response.save("./data").await?;

    paths
        .iter()
        .for_each(|path| println!("Image file path: {}", path.display()));

    Ok(())
}
