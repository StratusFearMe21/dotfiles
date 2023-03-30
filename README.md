# My dotfiles

This is a `makepkg` package which can be installed on a live cd of Artix/Arch with an
AUR helper.

## What do you get?

- Custom `dwl` build
- All startup programs managed by s6
- Custom `somebar` build
  - With a custom integrated status program written in Rust
- Nushell config with a greeting that runs in ~1ms thanks to the custom status bar
- Ayu Dark color scheme throughout
- `foot` config with colorscheme
- Custom wallpaper featuring Ferris the Crab
- Cascade `userChrome.css` with LibreWolf
- `arkenfox-user.js` with a few overrides
- Optimized version of `tuigreet` with greetd as login manager

## How to install
There are 2 ways to use this package

### During installation

1. Once you've gotten to the `chroot` part of the Artix/Arch installation process, make
a user and install `paru-bin`
2. Install all the AUR dependencies using `paru`
3. Install the package with `INSTALL_ARTIX=1`, `HOSTNAME` set to what you want your
hostname to be, and `DOTFILES_USER` set to the user you are installing the dotfiles
for. My timezone is `America/New_York`, so the script will set it to that too.

### On an already installed Arch/Artix distro

Simply install the package with `DOTFILES_USER` set to your current user.

For both of these processes, the environment variables are only needed once, and upgrading
is as simple as upgrading the package.