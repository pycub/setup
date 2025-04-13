# Enable Powerlevel10k instant prompt. Should stay close to the top of ~/.zshrc.
# Initialization code that may require console input (password prompts, [y/n]
# confirmations, etc.) must go above this block; everything else may go below.
if [[ -r "${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh" ]]; then
  source "${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh"
fi

# Oh My Zsh configuration
export ZSH="~/.oh-my-zsh"
export ZSH_PLUGINS_DIR="${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/plugins"
export FZF_BASE=$ZSH_PLUGINS_DIR/fzf
ZSH_THEME="powerlevel10k/powerlevel10k"
plugins=(
    git
    docker
    kubectl
    zsh-autosuggestions
    zsh-syntax-highlighting
    zsh-vi-mode
    pyenv-lazy
    zsh-rust
    fzf
)

source $ZSH/oh-my-zsh.sh

# Python development
export PATH="$HOME/.local/bin:$PATH"
export PYENV_ROOT="$HOME/.pyenv"

# Rust development
export PATH="$HOME/.cargo/bin:$PATH"
export RUST_SRC_PATH="$(rustc --print sysroot)/lib/rustlib/src/rust/library"

# Node
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # This loads nvm bash_completion


# Aliases
alias py=python
alias ipy=ipython
alias cargo-watch='cargo watch -x check -x test -x run'
alias k=kubectl
alias ll='exa -l --git --icons'
alias gs='git status'

# Initialize pyenv
if command -v pyenv &> /dev/null; then
    eval "$(pyenv init -)"
fi

# To customize prompt, run 
[[ ! -f ~/.p10k.zsh ]] || source ~/.p10k.zsh
