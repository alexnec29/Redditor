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
             println!("Extracted JSON: {}", json);
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