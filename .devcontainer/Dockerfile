FROM rust:1.80.0-bookworm

# https://code.visualstudio.com/remote/advancedcontainers/add-nonroot-user
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Create the user
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    #
    # [Optional] Add sudo support. Omit if you don't need to install software after connecting.
    && apt-get update \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

# ********************************************************
# * Anything else you want to do like clean up goes here *
# ********************************************************

# [Optional] Set the default user. Omit if you want to keep the default as root.
USER $USERNAME

# Install required packages and Rust toolchains
RUN sudo apt update
RUN sudo apt -y install avr-libc gcc-avr pkg-config avrdude libudev-dev build-essential
RUN rustup toolchain install stable
RUN rustup toolchain install nightly
RUN cargo +stable install ravedude

# Echo statements for debugging
RUN echo "Finished installing packages and Rust toolchains"