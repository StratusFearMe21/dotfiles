use std::path::Path;

use fst::Map;

fn main() {
    let map = Map::from_iter([
        ("/dotfiles/dwl/accel-profile", 0),
        ("/dotfiles/dwl/accel-speed", 1),
        ("/dotfiles/dwl/border-color", 2),
        ("/dotfiles/dwl/border-px", 3),
        ("/dotfiles/dwl/button-map", 4),
        ("/dotfiles/dwl/bypass-surface-visibility", 5),
        ("/dotfiles/dwl/click-method", 6),
        ("/dotfiles/dwl/disable-trackpad-while-typing", 7),
        ("/dotfiles/dwl/drag-lock", 8),
        ("/dotfiles/dwl/focus-color", 9),
        ("/dotfiles/dwl/fullscreen-bg", 10),
        ("/dotfiles/dwl/left-handed", 11),
        ("/dotfiles/dwl/log-level", 12),
        ("/dotfiles/dwl/middle-button-emulation", 13),
        ("/dotfiles/dwl/modkey", 14),
        ("/dotfiles/dwl/mouse-follows-focus", 15),
        ("/dotfiles/dwl/natural-scrolling", 16),
        ("/dotfiles/dwl/repeat-delay", 17),
        ("/dotfiles/dwl/repeat-rate", 18),
        ("/dotfiles/dwl/scroll-method", 19),
        ("/dotfiles/dwl/send-events-mode", 20),
        ("/dotfiles/dwl/sloppy-focus", 21),
        ("/dotfiles/dwl/tag-count", 22),
        ("/dotfiles/dwl/tap-to-click", 23),
        ("/dotfiles/dwl/tap-to-drag", 24),
        ("/dotfiles/dwl/xkb-options", 25),
    ])
    .unwrap();
    std::fs::write(
        Path::new(&std::env::var_os("OUT_DIR").unwrap()).join("fst.fst"),
        map.into_fst().into_inner(),
    )
    .unwrap();
}
