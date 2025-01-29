// Kaomoji, Discord bot
// Copyright (C) 2025  Valentine Briese
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// You may reach me via electronic mail at <valentinegb@icloud.com>.

use anyhow::Context as _;
use poise::{
    command,
    serenity_prelude::{AutocompleteChoice, ClientBuilder, GatewayIntents},
};
use poise_error::{anyhow, on_error, UserError};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use tracing::info;

type Context<'a> = poise::Context<'a, (), anyhow::Error>;

struct Kaomoji<'a> {
    text: &'a str,
    keywords: &'a [&'a str],
}

// Try to use longer keywords, since, for example, "joy" will still match
// "joyful" since it's contained in it, but "joyful" would not match "joy".
static KAOMOJIS: &[Kaomoji] = &[
    Kaomoji {
        text: "(^_^)",
        keywords: &["content", "blank", "original", "simple"],
    },
    Kaomoji {
        text: "(*_*)",
        keywords: &["blank", "simple", "stars"],
    },
    Kaomoji {
        text: "(T_T)",
        keywords: &["simple", "crying", "sadness", "unimpressed"],
    },
    Kaomoji {
        text: "(x_x)",
        keywords: &["simple", "stresed", "dead"],
    },
    Kaomoji {
        text: "(-_-;)",
        keywords: &["simple", "stresed", "nervous", "blank"],
    },
    Kaomoji {
        text: "(^.^)",
        keywords: &["simple", "cute", "happy"],
    },
    Kaomoji {
        text: "(^-^)",
        keywords: &["simple", "cute", "happy"],
    },
    Kaomoji {
        text: "(>.<)",
        keywords: &["simple", "cute", "embarrassed"],
    },
    Kaomoji {
        text: "(o_O)",
        keywords: &["simple", "confused", "spooked", "shaken"],
    },
    Kaomoji {
        text: "(O.O)",
        keywords: &["simple", "listening", "curious", "cute"],
    },
    Kaomoji {
        text: "(o.O)",
        keywords: &[
            "simple",
            "listening",
            "curious",
            "cute",
            "confused",
            "spooked",
            "shaken",
        ],
    },
    Kaomoji {
        text: "(e_e)",
        keywords: &["simple", "unimpressed", "bored", "tired", "sleepy"],
    },
    Kaomoji {
        text: "(e.e)",
        keywords: &["simple", "unimpressed", "bored", "tired", "sleepy", "cute"],
    },
    Kaomoji {
        text: "(^ム^)",
        keywords: &["nose", "content"],
    },
    Kaomoji {
        text: "(`Д´)",
        keywords: &["angry", "enraged", "outraged", "mad"],
    },
    Kaomoji {
        text: "(益)",
        keywords: &["angry", "enraged", "outraged", "mad"],
    },
    Kaomoji {
        text: "(◕‿◕✿)",
        keywords: &["happy", "smiley", "smiling", "cute", "flower", "pretty"],
    },
    Kaomoji {
        text: "(❤ω❤)",
        keywords: &["cute", "hearts", "love"],
    },
    Kaomoji {
        text: "(づ ◕‿◕ )づ",
        keywords: &["cute", "hug", "hands", "happy", "smiley", "smiling", "arms"],
    },
    Kaomoji {
        text: "(▰˘◡˘▰)",
        keywords: &["content", "happy", "smiley", "smiling", "blushing"],
    },
    Kaomoji {
        text: "ᶘᵒᴥᵒᶅ",
        keywords: &[
            "creature", "hands", "animal", "content", "happy", "smiley", "smiling", "cute", "arms",
        ],
    },
    Kaomoji {
        text: "t(-_-t)",
        keywords: &[
            "middle", "finger", "bird", "flip-off", "blank", "simple", "hands", "blank", "arms",
        ],
    },
    Kaomoji {
        text: "( ͡° ͜ʖ ͡°)",
        keywords: &[
            "lenny", "smiley", "smiling", "nose", "eyebrows", "content", "happy",
        ],
    },
    Kaomoji {
        text: "(* ^ ω ^)",
        keywords: &["happy", "smiley", "smiling", "cute"],
    },
    Kaomoji {
        text: "(´ ∀ ` *)",
        keywords: &["happy", "smiley", "smiling", "tired"],
    },
    Kaomoji {
        text: "٩(◕‿◕｡)۶",
        keywords: &["happy", "smiley", "smiling", "hands", "cute", "arms"],
    },
    Kaomoji {
        text: "☆*:.｡.o(≧▽≦)o.｡.:*☆",
        keywords: &[
            "happy", "smiley", "smiling", "hands", "magical", "spell", "stars", "sparkles",
        ],
    },
    Kaomoji {
        text: "(o^▽^o)",
        keywords: &["happy", "smiley", "smiling", "cheeks"],
    },
    Kaomoji {
        text: "(⌒▽⌒)☆",
        keywords: &["happy", "smiley", "smiling", "star", "sparkle"],
    },
    Kaomoji {
        text: "<(￣︶￣)>",
        keywords: &[
            "happy", "smiley", "smiling", "content", "hug", "hands", "basking", "arms",
        ],
    },
    Kaomoji {
        text: "。.:☆*:･'(*⌒―⌒*)))",
        keywords: &["content", "star", "sparkles", "magical"],
    },
    Kaomoji {
        text: "ヽ(・∀・)ﾉ",
        keywords: &["happy", "smiley", "smiling", "waving", "hands", "arms"],
    },
    Kaomoji {
        text: "(´｡• ω •｡`)",
        keywords: &["happy", "smiley", "smiling", "cheeks", "cute", "eyebrows"],
    },
    Kaomoji {
        text: "(￣ω￣)",
        keywords: &[
            "happy", "smiley", "smiling", "cute", "content", "tired", "sleepy", "sleeping",
        ],
    },
    Kaomoji {
        text: "｀;:゛;｀;･(°ε° )",
        keywords: &["blowing", "raspberries", "spitting"],
    },
    Kaomoji {
        text: "(o･ω･o)",
        keywords: &["cute", "smiling", "smiley", "happy", "cheeks"],
    },
    Kaomoji {
        text: "(＠＾◡＾)",
        keywords: &["smiling", "smiley", "happy", "ear", "flower", "rose"],
    },
];

async fn search<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Iterator<Item = AutocompleteChoice> + use<'a> {
    KAOMOJIS.iter().enumerate().filter_map(move |(i, kaomoji)| {
        if kaomoji.text.contains(partial)
            || partial.split_whitespace().all(|maybe_keyword| {
                kaomoji
                    .keywords
                    .iter()
                    .any(|keyword| keyword.contains(maybe_keyword))
            })
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
    #[description = "Type keywords which describe a kaomoji, the kaomoji text itself, or the index of a kaomoji"]
    #[autocomplete = "search"]
    index: usize,
) -> Result<(), anyhow::Error> {
    let kaomoji = KAOMOJIS
        .get(index)
        .ok_or(UserError::from(format!("no kaomoji at index {index}")))?;

    ctx.say(format!(
        "```\n{}\n```\n-# **Index:** {index}\n-# **Keywords:** {}",
        kaomoji
            .text
            // Make sure kaomojis don't mess with Discord's message formatting
            .replace('*', r#"\*"#)
            .replace('`', r#"\`"#)
            .replace('_', r#"\_"#),
        kaomoji.keywords.join(", "),
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
                info!("Completed setup");

                Ok(())
            })
        })
        .build();
    let client_builder =
        ClientBuilder::new(discord_token, GatewayIntents::empty()).framework(framework);
    #[cfg(not(debug_assertions))]
    let client_builder = client_builder.event_handler_arc({
        use std::time::Duration;

        use topgg::Autoposter;

        let topgg_token = secret_store
            .get("TOPGG_TOKEN")
            .context("`TOPGG_TOKEN` was not found")?;
        let topgg_client = topgg::Client::new(topgg_token);
        let autoposter = Autoposter::serenity(&topgg_client, Duration::from_secs(1800));

        autoposter.handler()
    });
    let client = client_builder
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
