use clap::{Parser, Subcommand};
use mouser_client::MouserApi;
use tokio;

/// Mouser API Command Line Interface
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Manufacturer List
    Mfg { },

    /// Search for a part
    Part {
        /// Part Number
        part_number: String,
        /// Manufacturer ID
        #[arg(short, long)]
        mfg_id: Option<u64>
    },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    dotenvy::dotenv().ok();

    let api_key = std::env::var("MOUSER_API_KEY").expect("MOUSER_API_KEY not set");

    let api = MouserApi::new(api_key);

    match &args.command {
        Commands::Mfg {} => {
            let res = api.search().manufacturer_list().await;
            match res {
                Ok(mfgs) => {
                    for mfg in mfgs {
                        println!("{}: {}", mfg.manufacturer_name, mfg.manufacturer_id.unwrap());
                    }
                }

                Err(e) => {
                    println!("err {:#?}", e);
                }
            }
        }

        Commands::Part { part_number, mfg_id } => {
            let results = api.search().part(part_number.to_string(), *mfg_id).await;

            match results {
                Ok(results) => {
                    for part in results {
                        println!("{:#?}", part);
                    }
                }

                Err(e) => {
                    println!("{:#?}", e);
                }
            }
        }
    }
}