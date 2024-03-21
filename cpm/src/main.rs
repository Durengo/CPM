use clap::Parser;

mod commands;

#[derive(Parser)]
#[clap(author, about, version, long_about)]
struct Cli {
    #[clap(subcommand)]
    command: commands::Commands,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        commands::Commands::Version(add_args) => commands::version::run(add_args),
    }
}
