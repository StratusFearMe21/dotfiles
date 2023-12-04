alias hx helix
alias rm rmtrash
alias u "upower -d"
alias ls "exa -la"
alias cat bat
alias s cat_sock

function giac
    /sbin/giac 2> /dev/null
end

starship init fish | source

export GPG_TTY=(tty)
bind -k nul accept-autosuggestion

function mark_prompt_start --on-event fish_prompt
    echo -en "\e]133;A\e\\"
end

function fish_greeting
    cat_sock
end
