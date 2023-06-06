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
        ("/dotfiles/dwl/middle-button-emulation", 12),
        ("/dotfiles/dwl/modkey", 13),
        ("/dotfiles/dwl/natural-scrolling", 14),
        ("/dotfiles/dwl/repeat-delay", 15),
        ("/dotfiles/dwl/repeat-rate", 16),
        ("/dotfiles/dwl/scroll-method", 17),
        ("/dotfiles/dwl/send-events-mode", 18),
        ("/dotfiles/dwl/sloppy-focus", 19),
        ("/dotfiles/dwl/tag-count", 20),
        ("/dotfiles/dwl/tap-to-click", 21),
        ("/dotfiles/dwl/tap-to-drag", 22),
        ("/dotfiles/dwl/xkb-options", 23),
    ])
    .unwrap();
    std::fs::write(
        Path::new(&std::env::var_os("OUT_DIR").unwrap()).join("fst.fst"),
        map.into_fst().into_inner(),
    )
    .unwrap();
}
