# Kaomoji

The Discord kaomoji search engine.

Just type `/kaomoji` and start describing the kaomoji you want!

<img width="663" alt="Screenshot 2025-01-24 at 3 22 34 PM" src="https://github.com/user-attachments/assets/f0c48c3c-4593-4299-a08d-7630a2bd8da8" />

Then, you can copy the kaomoji with formatting characters escaped so that it
doesn't mess with Discord's message formatting when you send it.

<img width="684" alt="Screenshot 2025-01-24 at 3 20 23 PM" src="https://github.com/user-attachments/assets/6b9f5681-9aa0-477a-b52f-676329eff0d6" />

As of **January 24th, 2025**, there are only **37** kaomojis. I'm gonna be
adding a ton more in the coming days. (^.^')

If you'd like to, it's really easy to **add your own kaomoji**! Just open
`src/main.rs` and add your kaomoji to the `KAOMOJIS` static item near the top of
the file. For example:

```rs
static KAOMOJIS: &[Kaomoji] = &[
    // other kaomojis...
    Kaomoji {
        text: "(* ^ ω ^)",
        keywords: &["happy", "smiley", "cute"],
    },
];
```

Next, you have to make a GitHub pull request. If you're not comfortable editing
code and making a pull request, you can also **just ask me to add a kaomoji**
and usually I will! You can [create an issue], [message me on Discord], or
email me at <valentinegb@icloud.com>.

[create an issue]: https://github.com/valentinegb/kaomoji/issues/new
[message me on Discord]: https://discord.com/users/1016154932354744330
