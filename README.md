This project is a simple implementation of Pong in Rust using the Ratatui library for the user interface. By default the left player is human and the right is the computer, but you can change that using the '-l' and '-r' options.

## Build and Run

```sh
cargo build -r
./target/release/pong
```

## Controls

Use the following controls to play:

- Player 1: 'w' (up), 's' (down)
- Player 2: 'i' (up), 'k' (down)
- Press 'q' to quit the game.

## Themes

pong uses the [TCA](https://github.com/carmiac/tca-themes) color theme system, so it will use your default TCA theme, with reasonable fallbacks. You can also chose a specific theme by passing it the '-t' flag. e.g. `pong -t "Tokyo Night"`

## FAQ

- Why?
  - Because Pong is a classic.
  - Because I wanted to learn Rust and Ratatui.

- Can I contribute?
  - Sure? Feel free to fork the repository and submit pull requests.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
