use serde::{Deserialize, Serialize};
use std::error::Error;

// Define your data structures based on the API response
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    username: String,
    email: String,
    password: String,
    avatar: String,
    address: Address,
    phone: String,
    website: String,
    company: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    country: String,
    city: String,
    street: String,
    alley: String,
    number: i32,
    geo: Geo,
}

#[derive(Debug, Serialize, Deserialize)]
struct Geo {
    lat: String,
    lng: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Company {
    company_name: String,
    #[serde(rename = "catchPhrase")]
    catch_phrase: String,
    bs: String,
}

// Implement a client for the API
struct ApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl ApiClient {
    fn new(base_url: &str) -> Self {
        ApiClient {
            client: reqwest::Client::new(),
            base_url: base_url.to_string(),
        }
    }

    async fn get_users(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let response = self.client
            .get(&format!("{}/users", self.base_url))
            .send()
            .await?
            .json::<Vec<User>>()
            .await?;
        
        Ok(response)
    }

    async fn get_user(&self, id: i32) -> Result<User, Box<dyn Error>> {
        let response = self.client
            .get(&format!("{}/users/{}", self.base_url, id))
            .send()
            .await?
            .json::<User>()
            .await?;
        
        Ok(response)
    }

    // Add more methods for different API endpoints as needed
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a client instance with the API base URL
    let client = ApiClient::new("https://jsonplaceholder.ir");
    
    // Get all users
    println!("Fetching all users...");
    let users = client.get_users().await?;
    println!("Found {} users", users.len());
    
    // Get a specific user
    println!("\nFetching user with ID 1...");
    let user = client.get_user(1).await?;
    println!("User details: {:#?}", user);
    
    Ok(())
}