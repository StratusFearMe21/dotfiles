pkgname=dotfiles
pkgver=0.9.1
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
  librewolf
  thunar
  thunar-archive-plugin
  thunar-volman
  tumbler
  ffmpegthumbnailer
  libgepub
  libgsf
  poppler-glib
  dash
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
  nushell
  grim
  opendoas
  gvfs
  mate-polkit
  pipewire
  pipewire-pulse
  pipewire-media-session
  pueue
  swaybg
  xorg-xwayland
  dashbinsh
  ttf-fira-code
  ttf-firacode-nerd
  arkenfox-user.js
  tofi
)
makedepends=(
  mold
  clang
  llvm
  rust
  meson
)
provides=('wlroots')
conflicts=('wlroots')

build() {
  CC=clang AR=llvm-ar CXX=clang++ CC_LD=mold CXX_LD=mold meson setup build \
    "$srcdir/.." \
    -Db_lto=true \
    --buildtype=release \
    --strip \
    --prefix=/usr \
    -Ddwl:xwayland=enabled

  meson compile -C build
  cargo build --release --target-dir "$srcdir/build/tuigreet/build" --manifest-path "$srcdir/../tuigreet/Cargo.toml"
}

package() {
  echo '*' > "$pkgdir/.gitignore"
  cd "$srcdir"
  meson install -C build --no-rebuild --destdir="$pkgdir"
  mkdir -p "$pkgdir/etc/profile.d"
  cp ../myprofile.sh "$pkgdir/etc/profile.d"
  cp ../cargo-env.sh "$pkgdir/etc/profile.d"
  mkdir -p "$pkgdir/usr/share/backgrounds"
  cp ../rust.png "$pkgdir/usr/share/backgrounds"
  mkdir -p "$pkgdir/etc/s6-user"
  cp -r ../s6/* "$pkgdir/etc/s6-user/"
  cp ../s6-db-reload-user "$pkgdir/usr/bin"
  cp "$srcdir/build/tuigreet/build/release/tuigreet" "$pkgdir/usr/bin"
  mkdir -p "$pkgdir/etc/greetd"
  cp ../wsetup.sh "$pkgdir/etc/greetd"
  cp ../greetd-config.toml "$pkgdir/etc/greetd/config.dotfile.toml"
  mkdir -p "$pkgdir/usr/share/wayland-sessions"
  cp ../dwl.desktop "$pkgdir/usr/share/wayland-sessions"
  cp ../doas.conf "$pkgdir/etc/doas.conf"
  mkdir -p "$pkgdir/usr/share/dotfiles"
  cp ../foot.ini "$pkgdir/usr/share/dotfiles"
  cp ../env.nu "$pkgdir/usr/share/dotfiles"
  cp ../config.nu "$pkgdir/usr/share/dotfiles"
  cp -r ../chrome "$pkgdir/usr/share/dotfiles"
  cp ../user-overrides.js "$pkgdir/usr/share/dotfiles"
  cp ../helix-config.toml "$pkgdir/usr/share/dotfiles"
  cp ../tofi-config "$pkgdir/usr/share/dotfiles" 
  cp ../link-dotfiles "$pkgdir/usr/share/dotfiles"
}