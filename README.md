<h1 align="center">TODO List</h1>
<p align="center">
  A simple command-line TODO list app written in Rust built as a learning project to practice the language.
</p>
<p align="center">
  <a href="README.ua.md">Читати українською</a>
</p>
<p align="center">
  <img src="https://img.shields.io/badge/Rust-learning%20project-orange?style=flat-square&logo=rust" />
  <img src="https://img.shields.io/badge/status-in%20progress-yellow?style=flat-square" />
</p>

---

## About

This is a beginner Rust project. The goal was to get hands-on experience with:

- Structs and enums
- Pattern matching
- Working with `Vec` and `String`

No external crates just the standard library.

---

## Usage

```bash
cargo run
```

The app starts in interactive mode just type commands and press Enter:

```
> add Buy groceries
> done 1
> remove 2
> edit 1 Buy milk
```

### Commands

| Command | Description |
|---|---|
| `add [Task Name]` | Add a new task |
| `done [Task ID]` | Mark a task as done |
| `remove [Task ID]` | Remove a task |
| `edit [Task ID] [New Name]` | Edit a task name |

---

## Roadmap

- [x] Add tasks
- [ ] Mark tasks as done
- [ ] Remove tasks
- [ ] Edit task name
- [ ] Save tasks to a file (persistence)
- [ ] Load tasks on startup
