pkgname=dotfiles
pkgver=0.9.7
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
  doas-sudo-shim
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
  echo '*' > "$pkgdir/../.gitignore"
  cd "$srcdir"
  meson install -C build --no-rebuild --destdir="$pkgdir"
  rm -rf "$pkgdir/usr/lib/librustbar.a"
  install -Dm644 ../myprofile.sh "$pkgdir/etc/profile.d/myprofile.sh"
  install -Dm644 ../cargo-env.sh "$pkgdir/etc/profile.d/cargo-env.sh"
  install -Dm644 ../rust.png "$pkgdir/usr/share/backgrounds/rust.png"
  install -dm755 "$pkgdir/etc/s6-user"
  cp -r ../s6-user "$pkgdir/etc"
  install -Dm755 ../s6-db-reload-user "$pkgdir/usr/bin/s6-db-reload-user"
  install -Dm755 "$srcdir/build/tuigreet/build/release/tuigreet" "$pkgdir/usr/bin/tuigreet"
  install -Dm755 ../wsetup.sh "$pkgdir/etc/greetd/wsetup.sh"
  install -Dm644 ../greetd-config.toml "$pkgdir/etc/greetd/config.dotfile.toml"
  install -Dm644 ../dwl.desktop "$pkgdir/usr/share/wayland-sessions/dwl.desktop"
  install -Dm644 ../doas.conf "$pkgdir/etc/doas.conf"
  install -Dm644 ../foot.ini "$pkgdir/usr/share/dotfiles/foot.ini"
  install -Dm755 ../env.nu "$pkgdir/usr/share/dotfiles/env.nu"
  install -Dm755 ../config.nu "$pkgdir/usr/share/dotfiles/config.nu"
  install -dm755 "$pkgdir/usr/share/dotfiles/chrome"
  cp -r ../chrome "$pkgdir/usr/share/dotfiles"
  install -Dm644 ../user-overrides.js "$pkgdir/usr/share/dotfiles/user-overrides.js"
  install -Dm644 ../helix-config.toml "$pkgdir/usr/share/dotfiles/helix-config.toml"
  install -Dm644 ../tofi-config "$pkgdir/usr/share/dotfiles/tofi-config" 
  install -Dm755 ../link-dotfiles "$pkgdir/usr/share/dotfiles/link-dotfiles"
}