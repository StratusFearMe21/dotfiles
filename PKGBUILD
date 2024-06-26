pkgname=dotfiles
pkgver=1.23.0
pkgrel=1
pkgdesc='All my dotfiles as one package'
arch=('any')
install=dotfiles.install
license=('gpl')
depends=(
  wayland
  libxcb
  xcb-util-wm
  xcb-util-renderutil
  libinput
  libdisplay-info
  seatd
  vulkan-icd-loader
  libglvnd
  mesa
  xcb-util-errors
  pixman
  libxkbcommon
  pango
  dbus
  libseccomp
  libinih
  connman
  wpa_supplicant
  playerctl
  foot
  firedragon
  thunar
  thunar-archive-plugin
  thunar-volman
  tumbler
  ffmpegthumbnailer
  libgepub
  libgsf
  poppler-glib
  dash-static-musl
  pamixer
  waylock
  light
  s6-rc
  greetd
  helix
  bash-language-server
  clang
  gopls
  lua-language-server
  python-lsp-server
  autopep8
  flake8
  python-mccabe
  python-pycodestyle
  python-pyflakes
  python-pylint
  python-rope
  python-whatthepatch
  yapf
  rust-analyzer
  taplo
  typescript-language-server
  vscode-css-languageserver
  vscode-html-languageserver
  yaml-language-server
  rmtrash
  upower
  exa
  bat
  starship
  fish
  grim
  opendoas
  gvfs
  polkit
  pipewire
  pipewire-pulse
  wireplumber
  pueue
  wbg
  xorg-xwayland
  dashbinsh
  ttf-fira-code
  ttf-firacode-nerd
  arkenfox-user.js
  doas-sudo-shim
  xdg-user-dirs
  execline
  ntp
  mpv
  gtk3
  glib2
  dconf
  bibata-cursor-theme
  materia-gtk-theme
  papirus-icon-theme
  qt5ct
  yazi
  elogind
  wtype
  wl-clipboard
  mako
  noto-fonts-cjk
  noto-fonts
  giac
  gamescope
  # profile-sync-daemon
)
makedepends=(
  mold
  clang
  llvm
  rust
  meson
)
provides=('default-cursors')
conflicts=('default-cursors')
recursion=0

build() {
  export srcdir
  CC=clang AR=llvm-ar CXX=clang++ CC_LD=mold CXX_LD=mold meson setup build \
    "$srcdir/.." \
    -Db_lto=true \
    --buildtype=release \
    --strip \
    --prefix=/usr \
    -Ddwl:xwayland=enabled

  meson compile -C build
  if [[ -f "$srcdir/rust-build" && $recursion -eq 0 ]]; then
    rm -rf "$srcdir/rust-build"
    recursion=1
    build
  fi
}

install-e() {
  echo Installing "$2" to "$3"
  install $@
}

cp-e() {
  echo Installing "$2" to "$3"
  install "$1" "$3"
  cp -r "$2" "$3/.."
}

package() {
  rm -rf "$srcdir/rust-build"
  echo '*' > "$pkgdir/../.gitignore"
  cp-e -dm755 "$srcdir/build/subprojects/rust-workspace/dist/usr" "$pkgdir/usr"
  cd "$srcdir"
  meson install -C build --no-rebuild --destdir="$pkgdir" --tags=dotfiles
  rm -rf "$pkgdir/usr/lib/librustbar.a"
  rm -rf "$pkgdir/usr/lib/libdbus_dwl.a"
  mkdir -p "$pkgdir/etc/mpv/scripts"
  ln -sf /usr/lib/liblistenbrainz_mpv.so "$pkgdir/etc/mpv/scripts/liblistenbrainz_mpv.so"
  install-e -Dm644 ../mpv.conf "$pkgdir/etc/mpv"
  install-e -Dm644 ../myprofile.sh "$pkgdir/etc/profile.d/myprofile.sh"
  install-e -Dm644 ../cargo-env.sh "$pkgdir/etc/profile.d/cargo-env.sh"
  install-e -Dm644 ../rust.png "$pkgdir/usr/share/backgrounds/rust.png"
  cp-e -dm755 ../s6-user "$pkgdir/etc/s6-user" 
  install-e -Dm755 ../s6-db-reload-user "$pkgdir/usr/bin/s6-db-reload-user"
  install-e -Dm755 ../wsetup "$pkgdir/etc/greetd/wsetup"
  install-e -Dm755 ../spawn-shell "$pkgdir/etc/greetd/spawn-shell"
  install-e -Dm644 ../greetd-config.toml "$pkgdir/etc/greetd/config.dotfile.toml"
  install-e -Dm644 ../dwl.desktop "$pkgdir/usr/share/wayland-sessions/dwl.desktop"
  install-e -Dm644 ../shell.desktop "$pkgdir/usr/share/wayland-sessions/shell.desktop"
  install-e -Dm644 ../raw-shell.desktop "$pkgdir/usr/share/wayland-sessions/raw-shell.desktop"
  install-e -Dm644 ../paru.conf "$pkgdir/usr/share/dotfiles/paru.conf"
  install-e -Dm644 ../doas.conf "$pkgdir/etc/doas.conf"
  install-e -Dm644 ../foot.ini "$pkgdir/usr/share/dotfiles/foot.ini"
  install-e -Dm644 ../scdaemon.conf "$pkgdir/etc/gnupg/scdaemon.conf"
  install-e -Dm644 ../zathurarc "$pkgdir/etc/zathurarc"
  install-e -Dm755 ../config.fish "$pkgdir/etc/fish/conf.d/dotfiles.fish"
  install-e -Dm755 ../cd.fish "$pkgdir/etc/fish/functions/cd.fish"
  install-e -Dm644 ../wireplumber.conf "$pkgdir/usr/share/wireplumber/wireplumber.conf"
  cp-e -dm755 ../helix "$pkgdir/usr/share/dotfiles/helix"
  cp-e -dm755 ../pipewire "$pkgdir/etc/pipewire"
  install-e -Dm644 ../user-overrides.js "$pkgdir/usr/share/dotfiles/user-overrides.js"
  install-e -Dm644 ../gtk-3.0-settings.ini "$pkgdir/usr/share/dotfiles/gtk-3.0-settings.ini"
  install-e -Dm644 ../dconf.ini "$pkgdir/usr/share/dotfiles/dconf.ini"
  install-e -Dm644 ../mako.config "$pkgdir/usr/share/dotfiles/mako.config"
  install-e -Dm644 ../index.theme "$pkgdir/usr/share/icons/default/index.theme"
  install-e -Dm644 ../dconf/dotfiles.somebar.gschema.xml "$pkgdir/usr/share/glib-2.0/schemas/dotfiles.somebar.gschema.xml"
  install-e -Dm644 ../dconf/dotfiles.dwl.gschema.xml "$pkgdir/usr/share/glib-2.0/schemas/dotfiles.dwl.gschema.xml"
  install-e -Dm755 ../link-dotfiles "$pkgdir/usr/share/dotfiles/link-dotfiles"
  install-e -Dm644 ../dwl-portals.conf "$pkgdir/usr/share/xdg-desktop-portal/dwl-portals.conf"
}
