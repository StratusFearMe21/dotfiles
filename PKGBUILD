pkgname=dotfiles
pkgver=1.1.9
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
  nushell
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
  tofi
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
)
makedepends=(
  mold
  clang
  llvm
  rust
  meson
)
provides=('wlroots' 'default-cursors')
conflicts=('wlroots' 'default-cursors')
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

package() {
  rm -rf "$srcdir/rust-build"
  echo '*' > "$pkgdir/../.gitignore"
  cd "$srcdir"
  meson install -C build --no-rebuild --destdir="$pkgdir"
  rm -rf "$pkgdir/usr/lib/librustbar.a"
  rm -rf "$pkgdir/usr/lib/libdbus_dwl.a"
  mkdir -p "$pkgdir/etc/mpv/scripts"
  ln -sf /usr/lib/liblistenbrainz_mpv.so "$pkgdir/etc/mpv/scripts/liblistenbrainz_mpv.so"
  ln -sf /usr/share/mpv/scripts/autoload.lua "$pkgdir/etc/mpv/scripts/autoload.lua"
  install -Dm644 ../mpv.conf "$pkgdir/etc/mpv"
  install -Dm644 ../myprofile.sh "$pkgdir/etc/profile.d/myprofile.sh"
  install -Dm644 ../cargo-env.sh "$pkgdir/etc/profile.d/cargo-env.sh"
  install -Dm644 ../rust.png "$pkgdir/usr/share/backgrounds/rust.png"
  install -dm755 "$pkgdir/etc/s6-user"
  cp -r ../s6-user "$pkgdir/etc"
  install -Dm755 ../s6-db-reload-user "$pkgdir/usr/bin/s6-db-reload-user"
  install -Dm755 ../wsetup "$pkgdir/etc/greetd/wsetup"
  install -Dm755 ../spawn-shell "$pkgdir/etc/greetd/spawn-shell"
  install -Dm644 ../greetd-config.toml "$pkgdir/etc/greetd/config.dotfile.toml"
  install -Dm644 ../dwl.desktop "$pkgdir/usr/share/wayland-sessions/dwl.desktop"
  install -Dm644 ../shell.desktop "$pkgdir/usr/share/wayland-sessions/shell.desktop"
  install -Dm644 ../doas.conf "$pkgdir/etc/doas.conf"
  install -Dm644 ../foot.ini "$pkgdir/usr/share/dotfiles/foot.ini"
  install -Dm755 ../env.nu "$pkgdir/usr/share/dotfiles/env.nu"
  install -Dm755 ../config.nu "$pkgdir/usr/share/dotfiles/config.nu"
  install -dm755 "$pkgdir/usr/share/dotfiles/chrome"
  cp -r ../chrome "$pkgdir/usr/share/dotfiles"
  cp -r ../helix "$pkgdir/usr/share/dotfiles"
  cp -r ../pipewire "$pkgdir/etc"
  install -Dm644 ../user-overrides.js "$pkgdir/usr/share/dotfiles/user-overrides.js"
  install -Dm644 ../tofi-config "$pkgdir/usr/share/dotfiles/tofi-config" 
  install -Dm644 ../gtk-3.0-settings.ini "$pkgdir/usr/share/dotfiles/gtk-3.0-settings.ini"
  install -Dm644 ../dconf.ini "$pkgdir/usr/share/dotfiles/dconf.ini"
  install -Dm644 ../index.theme "$pkgdir/usr/share/icons/default/index.theme"
  install -Dm644 ../dconf/dotfiles.somebar.gschema.xml "$pkgdir/usr/share/glib-2.0/schemas/dotfiles.somebar.gschema.xml"
  install -Dm644 ../dconf/dotfiles.dwl.gschema.xml "$pkgdir/usr/share/glib-2.0/schemas/dotfiles.dwl.gschema.xml"
  install -Dm755 ../link-dotfiles "$pkgdir/usr/share/dotfiles/link-dotfiles"
}