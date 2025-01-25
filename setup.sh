#!/bin/bash

set -e  # Exit immediately if a command exits with a non-zero status

# System Update
echo "🔄 Updating system packages..."
sudo apt update && sudo apt upgrade -y

# Core Development Tools
echo "🛠 Installing core development tools..."
sudo apt install -y build-essential curl wget git htop zsh \
    software-properties-common apt-transport-https ca-certificates gnupg

# Python Setup
echo "🐍 Setting up Python development environment..."
sudo apt install -y python3 python3-pip python3-venv
echo "   Installing PyEnv..."
curl https://pyenv.run | bash
echo "   Add the following to your ~/.bashrc or ~/.zshrc:"
echo '   export PATH="$HOME/.pyenv/bin:$PATH"'
echo '   eval "$(pyenv init -)"'
echo '   eval "$(pyenv virtualenv-init -)"'

echo "   Installing Poetry..."
curl -sSL https://install.python-poetry.org | python3 -
echo "   Poetry installed. Verify with 'poetry --version'"

# Rust Installation
echo "🦀 Setting up Rust development environment..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
echo "   Add Rust to PATH by running: source \$HOME/.cargo/env"
echo "   Install additional tools with: rustup component add..."

# Go Installation
echo "🟢 Installing Golang..."
sudo add-apt-repository ppa:longsleep/golang-backports -y
sudo apt update
sudo apt install -y golang-go
echo "   Go installed. Verify with 'go version'"

# Docker Setup
echo "🐳 Installing Docker and Docker Compose..."
sudo install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
sudo chmod a+r /etc/apt/keyrings/docker.gpg

echo \
  "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

sudo apt update
sudo apt install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

echo "   Adding current user to docker group..."
sudo usermod -aG docker $USER
echo "   IMPORTANT: Log out and log back in for docker group changes to take effect"

# VS Code
echo "💻 Installing VS Code..."
wget -qO- https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor > packages.microsoft.gpg
sudo install -D -o root -g root -m 644 packages.microsoft.gpg /etc/apt/keyrings/packages.microsoft.gpg
sudo sh -c 'echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/packages.microsoft.gpg] https://packages.microsoft.com/repos/code stable main" > /etc/apt/sources.list.d/vscode.list'
rm -f packages.microsoft.gpg
sudo apt update
sudo apt install -y code

# Google Chrome
echo "🌐 Installing Google Chrome..."
wget https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb
sudo dpkg -i google-chrome-stable_current_amd64.deb
sudo apt install -f -y
rm google-chrome-stable_current_amd64.deb

# Terminator
echo "📡 Installing Terminator..."
sudo apt install -y terminator

# Vim
echo "📝 Installing Vim..."
sudo apt install -y vim

# Additional Development Libraries
echo "📚 Installing additional development libraries..."
sudo apt install -y \
    zlib1g-dev \
    libbz2-dev \
    libreadline-dev \
    libsqlite3-dev \
    libncursesw5-dev \
    xz-utils \
    tk-dev \
    libxml2-dev \
    libxmlsec1-dev \
    libffi-dev \
    liblzma-dev

# Post-installation message
echo "✨ Development Environment Setup Complete! ✨"
echo "MANUAL STEPS REQUIRED:"
echo "1. Configure PyEnv paths in ~/.bashrc or ~/.zshrc"
echo "2. Install Zed editor manually (not available via apt)"
echo "3. Install V2rayN client manually"
echo "4. Restart terminal or run 'source ~/.bashrc'"
echo "5. Verify installations with version checks"
