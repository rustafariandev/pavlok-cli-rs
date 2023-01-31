use clap::{Args, Parser, Subcommand};
use pavlok;

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
    let client = pavlok::blocking::Client::new(cli.access_token);
    let result =  match &cli.command {
        Commands::Zap(msg) => {
            client.shock(msg.strength, &msg.reason)
        },
        Commands::Beep(msg) => {
            client.beep(msg.strength, &msg.reason)
        },
        Commands::Vibrate(msg) => {
            client.vibrate(msg.strength, &msg.reason)
        }
        Commands::Led(msg) => {
            client.led(msg.strength, &msg.reason)
        }
    };

    match result {
        Ok(_) => println!("Action send successfully"),
        Err(e) => println!("An error happened {}", e)
    }
}
