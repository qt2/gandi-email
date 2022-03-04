use anyhow::Result;
use clap::{Parser, Subcommand};
use dialoguer::theme::ColorfulTheme;
use gandi_email::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_to_string, File},
    io::Write,
    path::PathBuf,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    api_key: String,
    default_domain: String,
    default_mailbox: String,
}

impl Config {
    fn dotfile_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap();
        path.push(".gandi-email");
        path
    }

    /// Read config dotfile.
    pub fn from_dotfile() -> Result<Self> {
        let config = read_to_string(Self::dotfile_path())?;
        let config = toml::from_str(&config).unwrap();
        Ok(config)
    }

    /// Create config from dialogue.
    pub fn from_dialogue() -> Result<Self> {
        let mut config = Self::from_dotfile().unwrap_or_default();

        let theme = ColorfulTheme::default();
        config.api_key = dialoguer::Password::with_theme(&theme)
            .with_prompt("API Key (Hidden)")
            .allow_empty_password(true)
            .interact()
            .map(|v| {
                if v.is_empty() {
                    config.api_key
                } else {
                    v.trim().to_string()
                }
            })?;
        config.default_domain = dialoguer::Input::with_theme(&theme)
            .with_prompt("Default domain")
            .default(config.default_domain)
            .interact_text()?;
        config.default_mailbox = dialoguer::Input::with_theme(&theme)
            .with_prompt("Default mailbox ID")
            .default(config.default_mailbox)
            .interact_text()?;

        Ok(config)
    }

    /// Save config to dotfile.
    pub fn save(&self) -> Result<()> {
        let mut f = File::create(Self::dotfile_path())?;
        writeln!(f, "{}", toml::to_string(self).unwrap())?;
        Ok(())
    }
}

/// CLI tool for Gandi Email API
#[derive(Parser)]
#[clap(version, author)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set up your config
    Config,
    /// List domains
    Domains,
    /// List mailboxes
    Mailboxes {
        #[clap(short, long)]
        domain: Option<String>,
    },
    /// Manage aliases
    Alias {
        #[clap(subcommand)]
        command: AliasCommands,
        #[clap(short, long, global = true)]
        domain: Option<String>,
        #[clap(short, long, global = true)]
        mailbox_id: Option<String>,
    },
}

#[derive(Subcommand)]
enum AliasCommands {
    List,
    Create { alias: String },
    Delete { alias: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if matches!(&args.command, Commands::Config) {
        Config::from_dialogue()?.save()?;
        return Ok(());
    }

    let config = Config::from_dotfile().expect("Call `gandi-email config` first.");
    let api = GandiEmailAPI::new(config.api_key);
    match &args.command {
        Commands::Domains => {
            println!("{:#?}", api.domains().await?);
        }
        Commands::Mailboxes { domain } => {
            println!(
                "{:#?}",
                api.mailboxes(domain.as_ref().unwrap_or(&config.default_domain))
                    .await?
            );
        }
        Commands::Alias {
            command,
            domain,
            mailbox_id,
        } => match command {
            AliasCommands::List => {
                println!(
                    "{:#?}",
                    api.aliases(
                        domain.as_ref().unwrap_or(&config.default_domain),
                        mailbox_id.as_ref().unwrap_or(&config.default_mailbox)
                    )
                    .await?
                );
            }
            AliasCommands::Create { alias } => {
                api.create_alias(
                    domain.as_ref().unwrap_or(&config.default_domain),
                    mailbox_id.as_ref().unwrap_or(&config.default_mailbox),
                    alias,
                )
                .await?;
            }
            AliasCommands::Delete { alias } => {
                api.delete_alias(
                    domain.as_ref().unwrap_or(&config.default_domain),
                    mailbox_id.as_ref().unwrap_or(&config.default_mailbox),
                    alias,
                )
                .await?;
            }
        },
        _ => unreachable!(),
    }

    Ok(())
}
