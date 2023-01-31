use clap::{Args, Parser, Subcommand};
use serde::{Serialize, Deserialize};
use reqwest::blocking::Client;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(long, env = "PAVLOK_ACCESS_TOKEN")]
    access_token: String,
    #[command(subcommand)]
    command: Commands,
}


#[derive(Args)]
#[derive(Debug)]
struct Stimuli {
    #[arg(default_value_t = 85)]
    strength: u8,
    #[arg(default_value_t = String::from("Gotcha"))]
    reason: String,
}

#[derive(Args)]
#[derive(Debug)]
struct Stimuli4 {
    #[arg(
        default_value_t = 2,
        value_parser = clap::value_parser!(u8).range(1..=4),
    )]
    strength: u8,
    #[arg(default_value_t = String::from("Gotcha"))]
    reason: String,
}


#[derive(Debug, Serialize, Deserialize)]
struct StimuliResponse {
    success: bool,
    id: String,
}

impl Stimuli4 {
    fn url(&self, stimili: &str) -> String {
        format!("https://app.pavlok.com/api/v1/{}/{}", stimili, self.strength)
    }

    fn send(&self, token: &str, name: &str) -> Result<StimuliResponse, reqwest::Error> {
        Ok(Client::new()
           .post(self.url(name))
           .query(&[ ("access_token", token), ("reason", &self.reason)])
           .send()?
           .json::<StimuliResponse>()?)
    }
}

impl Stimuli {
    fn url(&self, stimili: &str) -> String {
        format!("https://app.pavlok.com/api/v1/{}/{}", stimili, self.strength)
    }

    fn send(&self, token: &str, name: &str) -> Result<StimuliResponse, reqwest::Error> {
        Ok(Client::new()
           .post(self.url(name))
           .query(&[ ("access_token", token), ("reason", &self.reason)])
           .send()?
           .json::<StimuliResponse>()?)
    }
}

#[derive(Debug)]
#[derive(Subcommand)]
enum Commands {
    /// Sends a zap to a pavlok
    Zap(Stimuli),
    /// Sends a beep to a pavlok
    Beep(Stimuli4),
    /// Vibrates a pavlok
    Vibrate(Stimuli),
    /// Lightup a pavlok
    Led(Stimuli4),
}

fn main() {
    let cli = Cli::parse();

    let result =  match &cli.command {
        Commands::Zap(msg) => {
            msg.send(&cli.access_token, "shock")
        },
        Commands::Beep(msg) => {
            msg.send(&cli.access_token, "beep")
        },
        Commands::Vibrate(msg) => {
            msg.send(&cli.access_token, "vibration")
        }
        Commands::Led(msg) => {
            msg.send(&cli.access_token, "led")
        }
    };

    match result {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{:?}", e)
    }


}
