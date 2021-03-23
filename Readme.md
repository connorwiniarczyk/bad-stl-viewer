# Bad STL Viewer

A really simple STL viewer written in Rust. I wrote this because Clement Hathaway told me it would impress his boss. Please do not confuse it with a good STL viewer

![](screenshots/fighter.png)

## Installing

Make sure that you have Cargo installed and that $HOME/.cargo/bin is in your PATH

```bash
git clone https://github.com/connorwiniarczyk/bad-stl-viewer.git
cd bad-stl-viewer
cargo install --path .
```

## Running

```bash
cd examples
bad-stl fighter.stl
```
