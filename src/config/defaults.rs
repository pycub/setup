// src/config/defaults.rs
//! Default configuration templates for various installers

/// Default ZSH config template
pub const DEFAULT_ZSHRC: &str = r#"# Path to your oh-my-zsh installation.
export ZSH="$HOME/.oh-my-zsh"

# Set theme
ZSH_THEME="robbyrussell"

# Plugins
plugins=(git zsh-autosuggestions zsh-syntax-highlighting)

source $ZSH/oh-my-zsh.sh

# User configuration
export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"

# NVM setup
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"

# Pyenv setup
export PYENV_ROOT="$HOME/.pyenv"
command -v pyenv >/dev/null || export PATH="$PYENV_ROOT/bin:$PATH"
eval "$(pyenv init -)"

# Poetry setup
export PATH="$HOME/.poetry/bin:$PATH"

# Aliases
alias ll="ls -la"
alias zshconfig="code ~/.zshrc"
alias ohmyzsh="code ~/.oh-my-zsh"
"#;

/// Default Tmux config template
pub const DEFAULT_TMUX_CONF: &str = r#"# Improve colors
set -g default-terminal "screen-256color"

# Set scrollback buffer to 10000
set -g history-limit 10000

# Customize the status line
set -g status-fg green
set -g status-bg black

# Enable mouse mode
set -g mouse on

# remap prefix from 'C-b' to 'C-a'
unbind C-b
set-option -g prefix C-a
bind-key C-a send-prefix

# split panes using | and -
bind | split-window -h
bind - split-window -v
unbind '"'
unbind %

# reload config file
bind r source-file ~/.tmux.conf \; display "Reloaded!"

# switch panes using Alt-arrow without prefix
bind -n M-Left select-pane -L
bind -n M-Right select-pane -R
bind -n M-Up select-pane -U
bind -n M-Down select-pane -D

# List of plugins
set -g @plugin 'tmux-plugins/tpm'
set -g @plugin 'tmux-plugins/tmux-sensible'
set -g @plugin 'tmux-plugins/tmux-resurrect'
set -g @plugin 'tmux-plugins/tmux-continuum'

# Initialize TMUX plugin manager (keep this line at the very bottom of tmux.conf)
run '~/.tmux/plugins/tpm/tpm'
"#;

/// Default Alacritty config template
pub const DEFAULT_ALACRITTY_CONFIG: &str = r#"window:
  padding:
    x: 10
    y: 10
  dynamic_padding: true
  decorations: full
  opacity: 0.95

font:
  normal:
    family: "JetBrains Mono"
    style: Regular
  bold:
    family: "JetBrains Mono"
    style: Bold
  italic:
    family: "JetBrains Mono"
    style: Italic
  size: 12.0

colors:
  primary:
    background: '#282c34'
    foreground: '#abb2bf'
  normal:
    black:   '#282c34'
    red:     '#e06c75'
    green:   '#98c379'
    yellow:  '#e5c07b'
    blue:    '#61afef'
    magenta: '#c678dd'
    cyan:    '#56b6c2'
    white:   '#abb2bf'
"#;

/// Default Zed settings template
pub const DEFAULT_ZED_SETTINGS: &str = r#"{
  "theme": "One Dark",
  "buffer_font_size": 14,
  "buffer_font_family": "JetBrains Mono",
  "tab_size": 2,
  "relative_line_numbers": true,
  "auto_update": true,
  "vim_mode": true,
  "telemetry": {
    "metrics": false
  }
}
"#;
