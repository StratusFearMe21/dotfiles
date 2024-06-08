export NO_AT_BRIDGE=1
export HELIX_DISABLE_AUTO_GRAMMAR_BUILD=1
export PARU_CONF=/usr/share/dotfiles/paru.conf
export DOTFILES_USER=$USER
export SDL_VIDEODRIVER=wayland
export SDL_AUDIODRIVER=pipewire
export MOZ_ENABLE_WAYLAND=1
export DBUS_SESSION_BUS_ADDRESS=unix:path=/tmp/dbus-session
export XCURSOR_THEME=Bibata-Modern-Classic
export XCURSOR_PATH=/usr/share/icons:$HOME/.icons:$HOME/.local/share/icons
export CC=clang
export CXX=clang++
export CC_LD=mold
export CXX_LD=mold
export RANLIB=llvm-ranlib
export AR=llvm-ar
export QT_QPA_PLATFORMTHEME=qt5ct
[ -f $HOME/.config/user-dirs.dirs ] && . $HOME/.config/user-dirs.dirs
