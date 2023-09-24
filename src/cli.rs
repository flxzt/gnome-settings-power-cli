use clap::{Parser, Subcommand};

use crate::{keyboard, screen};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Keyboard power settings.
    Keyboard {
        #[command(subcommand)]
        keyboard_command: KeyboardCommands,
    },
    /// Screen power settings.
    Screen {
        #[command(subcommand)]
        screen_command: ScreenCommands,
    },
}

#[derive(Subcommand)]
pub(crate) enum KeyboardCommands {
    /// Print the current keyboard brightness.
    Brightness,
    /// Set a new keyboard brightness.
    SetBrightness { brightness: u32 },
    /// Step up keyboard brightness.
    #[command(name = "brightness+")]
    BrightnessStepUp,
    /// Step down keyboard brightness.
    #[command(name = "brightness-")]
    BrightnessStepDown,
    /// Toggle keyboard brightness.
    BrightnessToggle,
}

#[derive(Subcommand)]
pub(crate) enum ScreenCommands {
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

    let cli = Cli::parse();

    match cli.command {
        Commands::Keyboard { keyboard_command } => {
            let proxy = keyboard::KeyboardProxy::new(&connection).await?;

            match keyboard_command {
                KeyboardCommands::Brightness => {
                    let brightness = proxy.brightness().await?;
                    println!("Current brightness: {brightness}%");
                }
                KeyboardCommands::SetBrightness { brightness } => {
                    proxy.set_brightness(brightness as i32).await?;
                    let new_brightness = proxy.brightness().await?;
                    println!("New brightness: {new_brightness}%");
                }
                KeyboardCommands::BrightnessStepUp => {
                    let new_brightness = proxy.step_up().await?;
                    println!("New brightness: {new_brightness}%");
                }
                KeyboardCommands::BrightnessStepDown => {
                    let new_brightness = proxy.step_down().await?;
                    println!("New brightness: {new_brightness}%");
                }
                KeyboardCommands::BrightnessToggle => {
                    let new_brightness = proxy.toggle().await?;
                    println!("New brightness: {new_brightness}%");
                }
            }
        }
        Commands::Screen { screen_command } => {
            let proxy = screen::ScreenProxy::new(&connection).await?;

            match screen_command {
                ScreenCommands::Brightness => {
                    let brightness = proxy.brightness().await?;
                    println!("Current brightness: {brightness}%");
                }
                ScreenCommands::SetBrightness { brightness } => {
                    proxy.set_brightness(brightness as i32).await?;
                    let new_brightness = proxy.brightness().await?;
                    println!("New brightness: {new_brightness}%");
                }
                ScreenCommands::BrightnessStepUp => {
                    let (new_brightness, _) = proxy.step_up().await?;
                    println!("New brightness: {new_brightness}%");
                }
                ScreenCommands::BrightnessStepDown => {
                    let (new_brightness, _) = proxy.step_down().await?;
                    println!("New brightness: {new_brightness}%");
                }
            }
        }
    }

    Ok(())
}
