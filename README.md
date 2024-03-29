# My dotfiles

![My desktop](screenshot.png)
![My desktop, empty](screenshot-desktop.png)

This is a `makepkg` package which can be installed on a live cd of Artix/Arch with an
AUR helper.

## What do you get?

- Custom `dwl` build
- All startup programs managed by s6
- Custom status bar inspired by `somebar`, written in Rust
  - Shows current tags, window layout, window title, and status bar text, updated in an extremely lightweight manner
  - Shows volume and brightness progress bars when they are adjusted using the buttons
  - Implements a `dmenu`-like application launcher that reads `.desktop` files and uses the [Helix QueryAtom](#helix-queryatoms) for pattern matching
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

### Does it work?

Yes, I tested it on a fresh install of Artix and worked out all the kinks!

### Per-Machine Configuration *(NEW)*

My DWL and somebar builds now support having differing configurations between machines without
having to re-compile everything. You simply need to install `dconf-editor`

```shell
sudo pacman -S dconf-editor
```

From there you can navigate into the `dotfiles` folder and change various settings from there!

![dconf-editor demo](screenshot-dconf.png)

***Configuration changes are applied instantly and don't require restarting the supported apps***

#### Colors

Colors in the configuration are expressed as Strings, and are interpreted by a CSS parser which supports much of [CSS Color 4](https://developer.chrome.com/articles/high-definition-css-color-guide/) and [CSS Color 5](https://developer.chrome.com/blog/css-relative-color-syntax/) 

![dconf-color-demo](screenshot-color.png)

### Helix QueryAtoms
You can use this syntax in the application launcher to match application names
```rust
enum QueryAtomKind {
    /// Item is a fuzzy match of this behaviour
    ///
    /// Usage: `foo`
    Fuzzy,
    /// Item contains query atom as a continuous substring
    ///
    /// Usage `'foo`
    Substring,
    /// Item starts with query atom
    ///
    /// Usage: `^foo`
    Prefix,
    /// Item ends with query atom
    ///
    /// Usage: `foo$`
    Postfix,
    /// Item is equal to query atom
    ///
    /// Usage `^foo$`
    Exact,
}
```

### Services behavior

These dotfiles and my DWL build leverages `s6` in order to run user services. My DWL build will also use the `logind` API to facilitate a graceful shutdown. Basically, when you run the `shutdown` command directly, everything will stop and be killed by the shutdown daemon. However, if you use `loginctl poweroff`, DWL will gracefully shutdown all `s6` user services, and *then* proceed with the shutdown

Basically, don't just run `shutdown` or `reboot`, use `loginctl` instead.
