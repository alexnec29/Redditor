# Redditor

A command-line tool built in **Rust** that fetches and displays posts from any subreddit in real-time

Live updates straight from Reddit — filtered by subreddit, sorting method, and interval

Through this project, I demonstrated my ability to build reliable, real-time systems using Rust and public APIs. It showcases my attention to detail, strong error handling, and a clear understanding of Rust’s ecosystem — from data handling to API integration and CLI design.

## Features

- Fetches Reddit posts via public API (JSON format)
-  Periodically checks for **new posts**
-  Supports sorting by: `hot`, `new`, `top`, `rising`, `controversial`
-  Customizable update interval (default: 60 seconds)
-  Shows local creation date of each post
-  Handles invalid subreddit or malformed JSON gracefully

 ## 🚀 How to Run

### 1. Clone the repository

```bash
git clone https://github.com/YOUR_USERNAME/redditor.git
cd redditor
```
### 2. Build and run with Cargo
```bash
cargo run -- [subreddit] [sort] [interval]
```
