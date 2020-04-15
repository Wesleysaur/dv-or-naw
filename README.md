# Disney Vacation / Not Disney Vacation
A game of guessing whether absurd wikihow articles are real, based on the picture. Inspired by [r/disneyvacation](https://www.reddit.com/r/disneyvacation/), [r/notdisneyvacation](https://www.reddit.com/r/notdisneyvacation/) and [damn.dog](https://damn.dog/). Why wait?
[Click here to access the game here!](https://wesleysaur.github.io/dv-or-naw/)

#### ⚠️ Content Warning: ⚠️ 
Some of the images could be considered NSFW, play at your own risk! They are hotlinked, so the repository and code are fully SFW.

## Building and running
If you don't have the web assembly target and cargo make already you should install it with
```
rustup update
rustup target add wasm32-unknown-unknown
cargo install cargo-make
```
Then you can build the app using `cargo make build` and run the web server with `cargo make serve`.
The annoying stuff was done for me by the kind folks at [seed-rs](⚠https://seed-rs.org/) so show them some love! The project is based off their [seed quickstart repo](https://github.com/seed-rs/seed-quickstart)

### Deploying to github pages
First, build the release version with `cargo make build_release`. Then, Github pages needs your static files in the `docs` directory, so run `deploy.sh` which puts everything in the correct place.

### Contributions
are welcome!!! I'm a rust beginner so PRs for code feedback and additional questions for the quiz are encouraged.
