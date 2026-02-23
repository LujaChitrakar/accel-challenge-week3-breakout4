use clap::{Parser, Subcommand};
mod commands;
mod error;
mod index;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Write,
    Branch { name: Option<String> }, // if no name list all branches else create one with name provided
    Switch { name: String },         // switch HEAD to the given branch
    Add { paths: Vec<String> },
    Commit { message: String },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::init()?,
        Commands::Branch { name } => commands::branch::branch(name)?,
        Commands::Switch { name } => commands::switch::switch(name)?,
        Commands::Add { paths } => commands::add::add(paths)?,
        Commands::Write => {}
        Commands::Commit { message } => {
            let tree_hash = commands::write_tree::write_tree()?;
            let parent_hash = commands::commit_tree::get_parent()?;
            commands::commit_tree::commit_tree(
                tree_hash.as_str(),
                Some(parent_hash),
                message.as_str(),
            )?
        }
    }

    Ok(())
}
