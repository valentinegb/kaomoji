# Kaomoji

The Discord kaomoji search engine.

Just type `/kaomoji` and start describing the kaomoji you want!

<!-- screenshot here -->

Then, you can copy the kaomoji with formatting characters escaped so that it
doesn't mess with Discord's message formatting when you send it.

<!-- screenshot here -->

As of **January 24th, 2025**, there are only **4** kaomojis. I'm gonna be adding
a ton more in the coming days. (^^')

If you'd like to, it's really easy to **add your own kaomoji**! Just open
`src/main.rs` and add your kaomoji to the `KAOMOJIS` static item near the top of
the file. For example:

```rs
static KAOMOJIS: &[Kaomoji] = &[
    // other komojis...
    Kaomoji {
        text: "(* ^ ω ^)",
        keywords: &["joyful", "happy", "smiley"],
    },
];
```

Next, you have to make a GitHub pull request. If you're not comfortable editing
code and making a pull request, you can also **just ask me to add a kaomoji**
and usually I will! You can [create an issue], [message me on Discord], or
email me at <valentinegb@icloud.com>.

[create an issue]: https://github.com/valentinegb/kaomoji/issues/new
[message me on Discord]: https://discord.com/users/1016154932354744330
