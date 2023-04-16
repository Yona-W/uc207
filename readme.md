
### Uc207 - Discord chatbot manager

Note - Not production-ready in any way whatsoever! I'm using this to learn Rust and I have no idea what I'm doing.

Also it relies on oobabooga's textgen frontend, which is still in development and doesn't have a stable API. It might break at any moment.

Usage:
- Have a discord bot token in `DISCORD_TOKEN` environment variable
- Have `config.json` and `prompt_template.txt` in the current working directory
- Have a `characters` folder with character definition json files. See the example in the `data` directory.
- `cargo run` and invite it to a server!