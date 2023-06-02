export NO_AT_BRIDGE=1
export DOTFILES_USER=$USER
export SDL_VIDEODRIVER=wayland
export SDL_AUDIODRIVER=pipewire
export MOZ_ENABLE_WAYLAND=1
export DBUS_SESSION_BUS_ADDRESS=unix:path=/tmp/dbus-session
export XCURSOR_THEME=Bibata-Modern-Classic
export CC=clang
export CXX=clang++
export CC_LD=mold
export CXX_LD=mold
export RANLIB=llvm-ranlib
export AR=llvm-ar
[ -f $HOME/.config/user-dirs.dirs ] && . $HOME/.config/user-dirs.dirs