use chrono::{DateTime, Local};
use serde_json::Value;
use std::env;
use ureq::Error;

fn main() -> Result<(), ureq::Error> {
    let valid_sorts = [
        String::from("hot"),
        String::from("new"),
        String::from("top"),
        String::from("rising"),
        String::from("controversial"),
    ];

    let args: Vec<String> = env::args().collect();
    let subreddit = args.get(1).unwrap_or(&"rust".to_string()).clone();
    let mut sort = args.get(2).unwrap_or(&"hot".to_string()).clone();

    if !valid_sorts.contains(&sort) {
        eprintln!(
            "Invalid sort method: '{}'. Switching to default: 'hot'.",
            sort
        );
        sort = "hot".to_string().clone();
    }

    let link = format!("https://www.reddit.com/r/{}/{}/.json", subreddit, sort);

    let response = ureq::get(&link).call();
    match response {
        Ok(res) => {
            let json: Value = serde_json::from_reader(res.into_reader()).unwrap();
            //println!("Extracted JSON: {}", json);

            let posts = match json["data"]["children"].as_array() {
                Some(posts) => posts,
                None => {
                    eprintln!("Invalid json!");
                    return Ok(());
                }
            };

            for post in posts {
                let post_data = &post["data"];
                let title = post_data["title"].as_str().unwrap_or("No title");
                let raw_permalink = post_data["permalink"].as_str().unwrap_or("");
                let permalink = format!("https://www.reddit.com{}", raw_permalink);
                let created_utc = post_data["created_utc"].as_f64().unwrap_or(0.0);

                let date_time =
                    DateTime::from_timestamp(created_utc as i64, 0).expect("Invalid timestamp");
                let local_date_time = date_time.with_timezone(&Local);
                {
                    println!("Title: {}", title);
                    println!("Link to post: {}", permalink);
                    println!("Creation date: {}", local_date_time);
                    println!("");
                }
            }
        }
        Err(Error::Status(code, res)) => {
            let response_string = res.into_string()?;
            eprintln!(
                "Response error of code {}. Reason: {}",
                code, response_string
            );
        }
        Err(_) => {
            eprintln!("Error at response!");
        }
    }
    Ok(())
}