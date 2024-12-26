use anyhow::Context as _;
use poise::{serenity_prelude as serenity, serenity_prelude::{ClientBuilder, GatewayIntents}};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;

pub mod commands;
pub mod models;
pub mod schema;

type Pool = diesel_async::pooled_connection::bb8::Pool<diesel_async::AsyncPgConnection>;
struct Data {
    pool: Pool,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

const REGISTER_GLOBALLY: bool = false;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore, #[shuttle_shared_db::Postgres(local_uri = "postgres://andrew:Andrew07%23@localhost:5432/rpg-bot")] pool: Pool) -> ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::create_character()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                if REGISTER_GLOBALLY {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                } else {
                    poise::builtins::register_in_guild(ctx, &framework.options().commands, serenity::GuildId::new(1320826396968226816)).await?;
                }

                Ok(Data { pool })
            })
        })
        .build();

    let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged())
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
