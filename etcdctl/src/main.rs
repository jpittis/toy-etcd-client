use clap::{Args, Parser, Subcommand};
use etcd_client::Client;

#[derive(Parser, Debug)]
#[command()]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Get(GetCommand),
    Put(PutCommand),
}

#[derive(Args, Debug)]
struct GetCommand {
    key: String,
}

#[derive(Args, Debug)]
struct PutCommand {
    key: String,
    value: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let mut client = Client::connect("http://127.0.0.1:2379").await?;
    match args.command {
        Commands::Get(command) => {
            let result = client.get(command.key.as_bytes().to_vec()).await?;
            println!("{result:?}");
        }
        Commands::Put(command) => {
            let result = client
                .put(
                    command.key.as_bytes().to_vec(),
                    command.value.as_bytes().to_vec(),
                )
                .await?;
            println!("{result:?}");
        }
    }
    Ok(())
}
