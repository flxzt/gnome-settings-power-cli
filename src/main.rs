pub(crate) mod cli;
pub(crate) mod keyboard;
pub(crate) mod screen;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::run().await
}
