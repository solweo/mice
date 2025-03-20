use clap::{Parser, Subcommand};
use dotenv::dotenv;
use std::path::PathBuf;
use reqwest::Client;
use serde_json::Value;
use chrono::{Datelike, NaiveDateTime};
use arboard::Clipboard;
use clap::builder::Str;
use std::env;
use std::env::args;
use colored::Colorize;
use itertools::Itertools;
use std::convert::TryFrom;
use anyhow::Error;
use std::io::Read;
use std::net::Ipv4Addr;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Cli {
    #[arg(long, default_value = ".")]
    root: PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    pub fn from_env_and_args() -> Self {
        dotenv().ok();
        Self::parse()
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    CiteYT {
        #[arg(short, long, env("YOUTUBE_API_KEY"))]
        yt_api_key: String,
        
        #[arg(short, long)]
        video_url: String,
    },
    RefWiki {
        #[arg(short, long)]
        article_url: String,
    },
}

async fn fetch_video_info(api_key: &str, video_id: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let client = Client::new(); // Create a new HTTP client

    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?key={}&id={}&fields=items(id,snippet(channelTitle, title, publishedAt))&part=snippet",
        api_key, video_id
    );

    let response = client.get(&url).send().await?; // Send the HTTP GET request

    // Check if the response was successful
    if !response.status().is_success() {
        println!("API request failed with status: {}", response.status());
        println!("Response body: {}", response.text().await?);
        return Err("API request failed".into());
    }

    let json: Value = response.json().await?; // Parse the response body as JSON

    // Check for API errors
    if let Some(error) = json.get("error") {
        print!("API returned an error: {:?}", error);
        return Err("API returned an error".into());
    }

    Ok(json) // Return the list of videos
}

fn format_to_cite(video: Value) -> String {
    let video = &video["items"][0];
    let snippet = &video["snippet"];

    format!("\
---
aliases: 
  - «{}» — {}; ({})
---

[`=this.aliases[0]`][link]^preview

[link]: https://youtu.be/{}

%%

Related:: 
Tags:: #status/glazed, #cite/video
Index:

Preceded:: 
Followed::",
        snippet["title"].as_str().unwrap_or(""),
        snippet["channelTitle"].as_str().unwrap_or(""),
        {
            let published_at = snippet["publishedAt"].as_str().unwrap_or("");
            NaiveDateTime::parse_from_str(&published_at[..19], "%Y-%m-%dT%H:%M:%S").unwrap().year()
        },
        video["id"].as_str().unwrap_or(""),
    )
}

pub fn parse_yt_link(url: &str) -> &str {
    url.split("v=").nth(1).unwrap().split('&').next().unwrap_or("")
}

pub async fn cite_yt(api_key: &str, video_url: &str) {
    let video_id = parse_yt_link(video_url);
    match fetch_video_info(api_key, video_id).await {
        Ok(video) => {
            println!("{video}");
            let mut clipboard = Clipboard::new().unwrap();
            let res = format_to_cite(video);
            clipboard.set_text(res.clone()).unwrap();
            println!("{}", res);
        }
        Err(e) => {
            println!("Error fetching video: {}", e);
        }
    }
}

async fn fetch_wiki_info(wiki_id: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let client = Client::new(); // Create a new HTTP client

    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=parse&format=json&formatversion=2&page={}&prop=",
        wiki_id
    );

    let response = client.get(&url).send().await?; // Send the HTTP GET request

    // Check if the response was successful
    if !response.status().is_success() {
        println!("API request failed with status: {}", response.status());
        println!("Response body: {}", response.text().await?);
        return Err("API request failed".into());
    }

    let json: Value = response.json().await?; // Parse the response body as JSON
    
    // Check for API errors
    if let Some(error) = json.get("error") {
        print!("API returned an error: {:?}", error);
        return Err("API returned an error".into());
    }

    Ok(json) 
}

pub fn parse_wiki_link(url: &str) -> &str {
    url.split("/").last().unwrap_or("")
}

pub async fn ref_wiki(wiki_url: &str) {
    let wiki_id = parse_wiki_link(wiki_url);
    
    match fetch_wiki_info(wiki_id).await {
        Ok(article) => {
            let mut clipboard = Clipboard::new().unwrap();
            let res = format_to_ref(article, wiki_url);
            clipboard.set_text(res.clone()).unwrap();
            println!("{}", res);
        }
        Err(e) => {
            println!("Error fetching video: {}", e);
        }
    }
}

fn format_to_ref(article: Value, article_url: &str) -> String {
    format!(" - [«{}» — wikipedia.org]({})",
        article["parse"]["title"].as_str().unwrap_or(""),
        article_url,
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Cli::from_env_and_args();

    match &cfg.command {
        Some(Commands::CiteYT { yt_api_key, video_url }) => { cite_yt(yt_api_key, video_url).await; },
        Some(Commands::RefWiki { article_url }) => { ref_wiki(article_url).await; }
        None => {
            println!("There was no subcommand given");
        }
    }

    Ok(())
}