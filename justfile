default:
    @just --list

build:
    cargo lbuild

run:
    RUST_LOG=debug cargo lrun

test:
    cargo lbuild --tests
    cargo nextest run

docs:
  oranda build
  oranda serve
