FROM rust:1.80.0-bookworm

RUN apt update
RUN apt -y install avr-libc gcc-avr pkg-config avrdude libudev-dev build-essential
RUN rustup toolchain install stable
RUN cargo +stable install ravedude