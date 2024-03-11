use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    id: i32,
    username: String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // URL of the Django API endpoint for fetching users
    let url = "http://localhost:8000/api/users/";

    // Send a GET request to the API endpoint
    let response = reqwest::get(url)
        .await?
        .json::<Vec<User>>()
        .await?;

    // Iterate through the fetched users and print them
    for user in response {
        println!("ID: {}, Username: {}, Password: {}", user.id, user.username, user.password);
    }

    Ok(())
}
