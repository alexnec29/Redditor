use chrono::{DateTime, Local};
use serde::Deserialize;
use std::env;
use std::{thread, time};
use ureq::Error;

#[derive(Debug, Deserialize)]
struct RedditResponse {
    data: RedditData,
}

#[derive(Debug, Deserialize)]
struct RedditData {
    children: Vec<RedditPost>,
}

#[derive(Debug, Deserialize)]
struct RedditPost {
    data: PostData,
}

#[derive(Debug, Deserialize)]
struct PostData {
    id: String,
    title: String,
    created_utc: f64,
    permalink: String,
}

impl PostData {
    fn created_datetime(&self) -> Result<DateTime<Local>, String> {
        if self.created_utc < 0.0 {
            return Err("Invalid timestamp: cannot be negative.".to_string());
        }

        match DateTime::<chrono::Utc>::from_timestamp(self.created_utc as i64, 0) {
            Some(date_time) => Ok(date_time.with_timezone(&Local)),
            None => Err("Invalid timestamp: could not create a valid DateTime.".to_string()),
        }
    }
}

fn main() {
    let valid_sorts = ["hot", "new", "top", "rising", "controversial"];

    let args: Vec<String> = env::args().collect();
    let subreddit = args.get(1).unwrap_or(&"rust".to_string()).clone();
    let mut sort = args.get(2).unwrap_or(&"hot".to_string()).clone();
    let seconds = args
        .get(3)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(60);
    let interval = time::Duration::from_secs(seconds);

    let mut printed_posts: Vec<String> = Vec::new();

    if !valid_sorts.contains(&sort.as_str()) {
        eprintln!(
            "Invalid sort method: '{}'. Switching to default: 'hot'.",
            sort
        );
        sort = "hot".to_string();
    }

    let link = format!("https://www.reddit.com/r/{}/{}/.json", subreddit, sort);

    loop {
        println!("Loading new posts...");

        match fetch_reddit_posts(&link) {
            Ok(reddit_response) => {
                if reddit_response.data.children.is_empty() {
                    eprintln!("No posts found. The subreddit might be invalid or empty.");
                    break;
                }

                for post in reddit_response.data.children {
                    let post_data = post.data;

                    if !printed_posts.contains(&post_data.id) {
                        printed_posts.push(post_data.id.clone());
                        println!("Title: {}", post_data.title);
                        println!("Link to post:https://www.reddit.com{}", post_data.permalink);
                        match post_data.created_datetime() {
                            Ok(local_date_time) => println!("Creation date: {}", local_date_time),
                            Err(e) => println!("Failed to parse creation date: {}", e),
                        }
                        println!();
                    }
                }
                println!(
                    "Printed the posts! Waiting the {} seconds interval...",
                    seconds
                );
            }
            Err(e) => {
                eprintln!("Error fetching posts: {}", e);
                break;
            }
        }

        thread::sleep(interval);
    }
}

fn fetch_reddit_posts(link: &str) -> Result<RedditResponse, String> {
    match ureq::get(link).call() {
        Ok(res) => serde_json::from_reader(res.into_reader())
            .map_err(|e| format!("Failed to parse JSON response: {}", e)),
        Err(Error::Status(code, res)) => {
            let response_string = res.into_string().unwrap_or_default();
            let reason =
                extract_reason(&response_string).map_or("Unknown reason".to_string(), |r| r);
            Err(format!("HTTP Error {} - {}", code, reason))
        }
        Err(Error::Transport(transport)) => {
            if let Some(message) = transport.message() {
                Err(format!("Network error: {}", message.to_string()))
            } else {
                Err("Unknown network error occurred.".to_string())
            }
        }
    }
}

fn extract_reason(response: &str) -> Option<String> {
    serde_json::from_str::<serde_json::Value>(response)
        .ok()
        .and_then(|json| {
            json.get("reason")
                .and_then(|r| r.as_str().map(|s| s.to_string()))
        })
}