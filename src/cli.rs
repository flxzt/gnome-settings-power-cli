use clap::{Parser, Subcommand};

use crate::screen;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Print the current screen brightness.
    Brightness,
    /// Set a new screen brightness.
    SetBrightness { brightness: u32 },
    /// Step up screen brightness.
    #[command(name = "brightness+")]
    BrightnessStepUp,
    /// Step down screen brightness.
    #[command(name = "brightness-")]
    BrightnessStepDown,
}

pub(crate) async fn run() -> anyhow::Result<()> {
    let connection = zbus::Connection::session().await.unwrap();
    let proxy = screen::ScreenProxy::new(&connection).await?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Brightness => {
            let brightness = proxy.brightness().await?;
            println!("Current brightness: {brightness}%");
        }
        Commands::SetBrightness { brightness } => {
            proxy.set_brightness(brightness as i32).await?;
            let new_brightness = proxy.brightness().await?;
            println!("New brightness: {new_brightness}%");
        }
        Commands::BrightnessStepUp => {
            let (new_brightness, _) = proxy.step_up().await?;
            println!("New brightness: {new_brightness}%");
        }
        Commands::BrightnessStepDown => {
            let (new_brightness, _) = proxy.step_down().await?;
            println!("New brightness: {new_brightness}%");
        }
    }

    Ok(())
}
