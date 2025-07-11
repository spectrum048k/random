# Use the official Ubuntu base image
FROM ubuntu:22.04

# Avoid interactive prompts during package install
ENV DEBIAN_FRONTEND=noninteractive

# Install Python, pip, Node.js, and npm
RUN apt-get update \
    && apt-get install -y python3 python3-pip nodejs npm \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install Rust and Cargo
RUN apt-get update \
    && apt-get install -y cargo rustc \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set up a working directory
WORKDIR /workspace

# (Optional) Set default Python and Node versions
RUN ln -sf /usr/bin/python3 /usr/bin/python

# (Optional) Upgrade pip
RUN pip3 install --upgrade pip
