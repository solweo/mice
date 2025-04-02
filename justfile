default:
  @just --list

dev:
  cargo leptos watch 

serve:
  cargo leptos serve -r

cli:
  cargo build --bin mice