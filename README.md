# Elixir

Elixir is a reddit-like web application with the goals of being tiny, performant and easy to set up.

## Deploy

Rust is required. [rustup](https://rustup.rs/) is recommended.

```bash
git clone https://github.com/knarkzel/elixir
cd elixir/
cargo run
```

## Built with

- [rust](https://www.rust-lang.org/)
- [rocket](https://rocket.rs/)
- [diesel](http://diesel.rs/)
- [sqlite](https://www.sqlite.org/index.html)
- [sailfish](https://sailfish.netlify.app/en/)
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark)

## Design

- Flairs / Categories
- Threads
- Updoots
- Comments / Posts
- Fast search
- Administration / Moderators

Text only. Media is bloat?
Random ascii art? Sort of like identifier.

Thread:
    - id
    - title
    - creation date
    - unique updoots
    - created by user

A thread is just an id + list of comments. First comment is op. Each comment
has a thread id that they're linked to. Timestamp. Updoots (per comment).
Thread wide search, site wide search.

Users create threads, like chans. Threads with high activity are ranked higher.
After some period of time, these threads are "archived", like a forum (lets say
24 to 48 hours).  Users can post comments to these threads. Link to other
threads. Updoot threads, comments.  Authentication is simple: username,
password. Threads can use "flairs" or be "flaired", for easier searching.
Searching through threads + comments should be blazing fast.
