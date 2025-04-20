mod error;

use error::ApiError;
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

// Define your data structures
#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: i32,
    name: String,
    username: String,
    email: String,
    address: Address,
    phone: String,
    website: String,
    company: Company,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Address {
    street: String,
    suite: String,
    city: String,
    zipcode: String,
    geo: Geo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Geo {
    lat: String,
    lng: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Company {
    name: String,
    catchPhrase: String,
    bs: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Post {
    userId: i32,
    id: i32,
    title: String,
    body: String,
}

// Synchronous API client
struct ApiClient {
    client: reqwest::blocking::Client,
    base_url: String,
}

impl ApiClient {
    fn new(base_url: &str) -> Self {
        ApiClient {
            client: reqwest::blocking::Client::new(),
            base_url: base_url.to_string(),
        }
    }
    
    fn get<T>(&self, endpoint: &str) -> Result<T, ApiError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.get(&url).send()?;
        
        if response.status().is_success() {
            let data = response.json::<T>()?;
            Ok(data)
        } else {
            Err(ApiError::Other(format!(
                "API request failed with status: {}",
                response.status()
            )))
        }
    }
    
    fn post<T, U>(&self, endpoint: &str, body: &T) -> Result<U, ApiError>
    where
        T: Serialize,
        U: for<'de> Deserialize<'de>,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.post(&url).json(body).send()?;
        
        if response.status().is_success() {
            let data = response.json::<U>()?;
            Ok(data)
        } else {
            Err(ApiError::Other(format!(
                "API request failed with status: {}",
                response.status()
            )))
        }
    }
}

// Function to poll users in a separate thread
fn poll_users(client: Arc<ApiClient>, shared_data: Arc<Mutex<Vec<User>>>, interval_secs: u64) {
    thread::spawn(move || {
        loop {
            match client.get::<Vec<User>>("/users") {
                Ok(users) => {
                    println!("Fetched {} users", users.len());
                    
                    // Update the shared data
                    {
                        let mut data = shared_data.lock().unwrap();
                        *data = users;
                    }
                },
                Err(e) => {
                    println!("Error fetching users: {}", e);
                }
            }
            
            thread::sleep(Duration::from_secs(interval_secs));
        }
    });
}

// Function to poll posts in a separate thread
fn poll_posts(client: Arc<ApiClient>, user_id: i32, tx: mpsc::Sender<Vec<Post>>, interval_secs: u64) {
    thread::spawn(move || {
        loop {
            match client.get::<Vec<Post>>(&format!("/users/{}/posts", user_id)) {
                Ok(posts) => {
                    println!("Fetched {} posts for user {}", posts.len(), user_id);
                    
                    // Send through channel
                    if let Err(e) = tx.send(posts) {
                        println!("Error sending posts: {}", e);
                        break;
                    }
                },
                Err(e) => {
                    println!("Error fetching posts: {}", e);
                }
            }
            
            thread::sleep(Duration::from_secs(interval_secs));
        }
    });
}

fn main() -> Result<(), ApiError> {
    println!("Starting API client application...");
    
    // Create the API client and wrap in Arc for thread sharing
    let client = Arc::new(ApiClient::new("https://jsonplaceholder.typicode.com"));
    
    // Create shared data structures
    let users_data = Arc::new(Mutex::new(Vec::<User>::new()));
    let (posts_tx, posts_rx) = mpsc::channel::<Vec<Post>>();
    
    // Start background polling threads
    poll_users(Arc::clone(&client), Arc::clone(&users_data), 5);
    poll_posts(Arc::clone(&client), 1, posts_tx, 8);
    
    println!("Main application running...");
    println!("Press Ctrl+C to stop");
    
    // Main application loop
    let mut counter = 0;
    loop {
        // Check for new posts (non-blocking)
        match posts_rx.try_recv() {
            Ok(posts) => {
                println!("Received updated posts data - {} posts available", posts.len());
                
                // Example of using the data
                if let Some(post) = posts.iter().max_by_key(|p| p.body.len()) {
                    println!("Longest post title: {}", post.title);
                }
            },
            Err(mpsc::TryRecvError::Empty) => {
                // No new data, continue
            },
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("Posts channel disconnected");
                break;
            }
        }
        
        // Access shared users data periodically
        if counter % 10 == 0 {
            let users = users_data.lock().unwrap();
            println!("Current user count: {}", users.len());
            
            if !users.is_empty() {
                println!("First user: {}", users[0].name);
            }
        }
        
        // Main application work here
        println!("Application is working...");
        
        counter += 1;
        thread::sleep(Duration::from_secs(1));
    }
    
    Ok(())
}