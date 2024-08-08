use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
#[command(subcommand_help_heading = "MyAwesomeHeading")]
enum Commands {
    
    #[command(subcommand_help_heading = "FirstHeading")]
    Subcommand1(Subcommand1Args),
    #[command(subcommand_help_heading = "SecondHeading")]
    Subcommand2(Subcommand2Args),
}

#[derive(Parser)]
struct Subcommand1Args {
    #[command(subcommand)]
    sub: Subcommands1,
}

#[derive(Subcommand)]
enum Subcommands1 {
    /// Do A.
    A,
   /// Do B.
    B(IntArg)
}

#[derive(Parser)]
struct IntArg {
    arg: u32,
}

#[derive(Parser)]
struct Subcommand2Args;

fn main() {
    let cli = CLI::parse(); 
    
    match &cli.command {
        Commands::Subcommand1(arg) => {
           match &arg.sub {
                Subcommands1::B(_) => {
                    println!("test");
                },
                Subcommands1::A => {
                    println!("a");
                }
            }
        },
        Commands::Subcommand2(..) => {
           println!("two");
        }
        }
    }
