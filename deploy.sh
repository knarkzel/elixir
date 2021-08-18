#!/bin/sh

pgrep elixir | xargs kill -9
nohup cargo run --release &
watch tail -n 10 nohup.out
