use anyhow::{anyhow, Context as _};
use poise::{
    command,
    serenity_prelude::{AutocompleteChoice, ClientBuilder, GatewayIntents},
};
use poise_error::{anyhow, on_error};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use tracing::info;

type Context<'a> = poise::Context<'a, (), anyhow::Error>;

struct Kaomoji<'a> {
    text: &'a str,
    keywords: &'a [&'a str],
}

static KAOMOJIS: &[Kaomoji] = &[
    Kaomoji {
        text: "(* ^ ω ^)",
        keywords: &["joy", "happy", "smile"],
    },
    Kaomoji {
        text: "(´ ∀ ` *)",
        keywords: &["joy", "happy", "smile", "grin"],
    },
    Kaomoji {
        text: "٩(◕‿◕｡)۶",
        keywords: &["joy", "happy", "smile", "hands"],
    },
    Kaomoji {
        text: "☆*:.｡.o(≧▽≦)o.｡.:*☆",
        keywords: &["joy", "happy", "smile", "hands", "magic", "spell", "stars"],
    },
];

async fn search<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Iterator<Item = AutocompleteChoice> + use<'a> {
    KAOMOJIS.iter().enumerate().filter_map(move |(i, kaomoji)| {
        if kaomoji.text.contains(partial)
            || kaomoji
                .keywords
                .iter()
                .any(|keyword| keyword.contains(partial))
            || partial.parse().is_ok_and(|maybe_i: usize| maybe_i == i)
        {
            Some(AutocompleteChoice::new(kaomoji.text, i))
        } else {
            None
        }
    })
}

/// Search for kaomoji
#[command(
    slash_command,
    install_context = "Guild | User",
    interaction_context = "Guild | BotDm | PrivateChannel",
    ephemeral
)]
async fn kaomoji(
    ctx: Context<'_>,
    #[description = "The kaomoji you would like"]
    #[autocomplete = "search"]
    kaomoji: usize,
) -> Result<(), anyhow::Error> {
    ctx.say(format!(
        "```\n{}\n```",
        KAOMOJIS
            .get(kaomoji)
            .ok_or(anyhow!("no kaomoji at index {kaomoji}"))?
            .text
            // Make sure kaomojis don't mess with Discord's message formatting
            .replace('*', r#"\*"#)
            .replace('`', r#"\`"#)
    ))
    .await?;

    Ok(())
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("`DISCORD_TOKEN` was not found")?;
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![kaomoji()],
            on_error,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                info!("Registered commands");

                Ok(())
            })
        })
        .build();
    let client = ClientBuilder::new(discord_token, GatewayIntents::empty())
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
