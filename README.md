# beggar_my_rust
## How to install/run
1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone this repo
3. cd into it
4. `cargo run --release`

## How to use
When you run the program, you will have a prompt giving you three options. You can test a specific game, generate a single random game, or have it infinitely generate games and output the best it finds. When running it infinitely, you can choose how many thread it spawns. Are you not doing anything computatoinally expensive with your computer and just want to run it in the background while you do other stuff? Have it spawn 1 thread. Want it to go as fast as possible? The code will tell you how many CPU cores you have and you can use that number. If you want to stop it, Ctrl+c should work. You can also just close the terminal.

## Why did you make this
I was watching [Matt Parker's video](https://www.youtube.com/watch?v=1HwKCvsdXiw) and had some time to kill. I knew I could make a fast brute force solver using Rust and multithreading.

## Why do the numbers not line up exactly wth [Richard Mann's website](https://www.richardpmann.com/beggar-my-neighbour-records.html)?
I spent a bit of time trying to make that the case. I would get one working, then another would be off by 1 or 2. I'm not sure exactly how they count it. I'm not even sure it's consistent. But given that it seems to always be 0-2 off, I figured it's not that big of a deal, and I just counted it the way that makes sense to me
