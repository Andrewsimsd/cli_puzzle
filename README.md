
# 🧩 Maze Traversal CLI

A terminal-based maze puzzle game written in Rust. Explore a procedurally generated maze using simple commands, and uncover the secret message hidden at the exit.

---

## 🎮 Features

- 🔄 Procedurally generated maze with adjustable size
- 💻 Text-based UI
- 🧭 Move using `n`, `s`, `e`, `w`
- 🎯 Reach the goal and reveal a secret message
- 🧱 Optional ASCII maze visualization (`--show-maze`)
- 🏃‍♂️ Handles large mazes efficiently (e.g., 100x100)

---

## 🚀 Usage

### 🔧 Build & Run

```bash
cargo build --release
cargo run --release -- [--show-maze]
```

### 🕹️ Commands

| Key | Action         |
|-----|----------------|
| `n` | Move North     |
| `s` | Move South     |
| `e` | Move East      |
| `w` | Move West      |
| `q` | Quit Game      |

> Tip: Use `--show-maze` to visualize the maze structure while playing.

---

## ⚙️ Configuration

You can change the size of the maze by editing this line in `main()`:

```rust
let mut maze = Maze::new(5, 5); // Set your preferred size
```

For large mazes, try `Maze::new(100, 100)`.

---

## 📦 Dependencies

| Crate  | Version | Description                      |
|--------|---------|----------------------------------|
| `rand` | ^0.8    | Random number generation         |


---

## 💡 Example

```text
Welcome to the Maze Puzzle!
Navigate using: n (north), s (south), e (east), w (west), q (quit)
You are at (0, 0). Move:
```

With `--show-maze` enabled, you'll see:

```
Maze Legend: P=Player, E=Exit, . = Path, # = Wall

P .|. .
--  --  
. . E .
```

---

## 🛠️ Future Ideas

- Fog-of-war effect
- Save/load progress
- Timed mode / leaderboard
- Breadcrumb trails

---

## 📜 License

MIT

---

🦀 Built with Rust and imagination.
