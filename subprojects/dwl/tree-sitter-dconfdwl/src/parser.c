#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#ifdef _MSC_VER
#pragma optimize("", off)
#elif defined(__clang__)
#pragma clang optimize off
#elif defined(__GNUC__)
#pragma GCC optimize ("O0")
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 4
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 28
#define ALIAS_COUNT 0
#define TOKEN_COUNT 27
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 1
#define PRODUCTION_ID_COUNT 1

enum {
  sym_accel_profile = 1,
  sym_accel_speed = 2,
  sym_border_color = 3,
  sym_border_px = 4,
  sym_button_map = 5,
  sym_bypass_surface_visibility = 6,
  sym_click_method = 7,
  sym_disable_trackpad_while_typing = 8,
  sym_drag_lock = 9,
  sym_focus_color = 10,
  sym_fullscreen_bg = 11,
  sym_left_handed = 12,
  sym_log_level = 13,
  sym_middle_button_emulation = 14,
  sym_modkey = 15,
  sym_mouse_follows_focus = 16,
  sym_natural_scrolling = 17,
  sym_repeat_delay = 18,
  sym_repeat_rate = 19,
  sym_scroll_method = 20,
  sym_send_events_mode = 21,
  sym_sloppy_focus = 22,
  sym_tag_count = 23,
  sym_tap_to_click = 24,
  sym_tap_to_drag = 25,
  sym_xkb_options = 26,
  sym_source_file = 27,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_accel_profile] = "accel_profile",
  [sym_accel_speed] = "accel_speed",
  [sym_border_color] = "border_color",
  [sym_border_px] = "border_px",
  [sym_button_map] = "button_map",
  [sym_bypass_surface_visibility] = "bypass_surface_visibility",
  [sym_click_method] = "click_method",
  [sym_disable_trackpad_while_typing] = "disable_trackpad_while_typing",
  [sym_drag_lock] = "drag_lock",
  [sym_focus_color] = "focus_color",
  [sym_fullscreen_bg] = "fullscreen_bg",
  [sym_left_handed] = "left_handed",
  [sym_log_level] = "log_level",
  [sym_middle_button_emulation] = "middle_button_emulation",
  [sym_modkey] = "modkey",
  [sym_mouse_follows_focus] = "mouse_follows_focus",
  [sym_natural_scrolling] = "natural_scrolling",
  [sym_repeat_delay] = "repeat_delay",
  [sym_repeat_rate] = "repeat_rate",
  [sym_scroll_method] = "scroll_method",
  [sym_send_events_mode] = "send_events_mode",
  [sym_sloppy_focus] = "sloppy_focus",
  [sym_tag_count] = "tag_count",
  [sym_tap_to_click] = "tap_to_click",
  [sym_tap_to_drag] = "tap_to_drag",
  [sym_xkb_options] = "xkb_options",
  [sym_source_file] = "source_file",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_accel_profile] = sym_accel_profile,
  [sym_accel_speed] = sym_accel_speed,
  [sym_border_color] = sym_border_color,
  [sym_border_px] = sym_border_px,
  [sym_button_map] = sym_button_map,
  [sym_bypass_surface_visibility] = sym_bypass_surface_visibility,
  [sym_click_method] = sym_click_method,
  [sym_disable_trackpad_while_typing] = sym_disable_trackpad_while_typing,
  [sym_drag_lock] = sym_drag_lock,
  [sym_focus_color] = sym_focus_color,
  [sym_fullscreen_bg] = sym_fullscreen_bg,
  [sym_left_handed] = sym_left_handed,
  [sym_log_level] = sym_log_level,
  [sym_middle_button_emulation] = sym_middle_button_emulation,
  [sym_modkey] = sym_modkey,
  [sym_mouse_follows_focus] = sym_mouse_follows_focus,
  [sym_natural_scrolling] = sym_natural_scrolling,
  [sym_repeat_delay] = sym_repeat_delay,
  [sym_repeat_rate] = sym_repeat_rate,
  [sym_scroll_method] = sym_scroll_method,
  [sym_send_events_mode] = sym_send_events_mode,
  [sym_sloppy_focus] = sym_sloppy_focus,
  [sym_tag_count] = sym_tag_count,
  [sym_tap_to_click] = sym_tap_to_click,
  [sym_tap_to_drag] = sym_tap_to_drag,
  [sym_xkb_options] = sym_xkb_options,
  [sym_source_file] = sym_source_file,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_accel_profile] = {
    .visible = true,
    .named = true,
  },
  [sym_accel_speed] = {
    .visible = true,
    .named = true,
  },
  [sym_border_color] = {
    .visible = true,
    .named = true,
  },
  [sym_border_px] = {
    .visible = true,
    .named = true,
  },
  [sym_button_map] = {
    .visible = true,
    .named = true,
  },
  [sym_bypass_surface_visibility] = {
    .visible = true,
    .named = true,
  },
  [sym_click_method] = {
    .visible = true,
    .named = true,
  },
  [sym_disable_trackpad_while_typing] = {
    .visible = true,
    .named = true,
  },
  [sym_drag_lock] = {
    .visible = true,
    .named = true,
  },
  [sym_focus_color] = {
    .visible = true,
    .named = true,
  },
  [sym_fullscreen_bg] = {
    .visible = true,
    .named = true,
  },
  [sym_left_handed] = {
    .visible = true,
    .named = true,
  },
  [sym_log_level] = {
    .visible = true,
    .named = true,
  },
  [sym_middle_button_emulation] = {
    .visible = true,
    .named = true,
  },
  [sym_modkey] = {
    .visible = true,
    .named = true,
  },
  [sym_mouse_follows_focus] = {
    .visible = true,
    .named = true,
  },
  [sym_natural_scrolling] = {
    .visible = true,
    .named = true,
  },
  [sym_repeat_delay] = {
    .visible = true,
    .named = true,
  },
  [sym_repeat_rate] = {
    .visible = true,
    .named = true,
  },
  [sym_scroll_method] = {
    .visible = true,
    .named = true,
  },
  [sym_send_events_mode] = {
    .visible = true,
    .named = true,
  },
  [sym_sloppy_focus] = {
    .visible = true,
    .named = true,
  },
  [sym_tag_count] = {
    .visible = true,
    .named = true,
  },
  [sym_tap_to_click] = {
    .visible = true,
    .named = true,
  },
  [sym_tap_to_drag] = {
    .visible = true,
    .named = true,
  },
  [sym_xkb_options] = {
    .visible = true,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(296);
      if (lookahead == '/') ADVANCE(70);
      END_STATE();
    case 1:
      if (lookahead == '-') ADVANCE(127);
      END_STATE();
    case 2:
      if (lookahead == '-') ADVANCE(227);
      END_STATE();
    case 3:
      if (lookahead == '-') ADVANCE(180);
      END_STATE();
    case 4:
      if (lookahead == '-') ADVANCE(67);
      END_STATE();
    case 5:
      if (lookahead == '-') ADVANCE(84);
      END_STATE();
    case 6:
      if (lookahead == '-') ADVANCE(65);
      END_STATE();
    case 7:
      if (lookahead == '-') ADVANCE(179);
      END_STATE();
    case 8:
      if (lookahead == '-') ADVANCE(284);
      END_STATE();
    case 9:
      if (lookahead == '-') ADVANCE(118);
      END_STATE();
    case 10:
      if (lookahead == '-') ADVANCE(288);
      END_STATE();
    case 11:
      if (lookahead == '-') ADVANCE(49);
      END_STATE();
    case 12:
      if (lookahead == '-') ADVANCE(181);
      END_STATE();
    case 13:
      if (lookahead == '-') ADVANCE(115);
      END_STATE();
    case 14:
      if (lookahead == '-') ADVANCE(48);
      END_STATE();
    case 15:
      if (lookahead == '-') ADVANCE(59);
      END_STATE();
    case 16:
      if (lookahead == '-') ADVANCE(266);
      END_STATE();
    case 17:
      if (lookahead == '-') ADVANCE(159);
      END_STATE();
    case 18:
      if (lookahead == '-') ADVANCE(64);
      END_STATE();
    case 19:
      if (lookahead == '-') ADVANCE(254);
      END_STATE();
    case 20:
      if (lookahead == '-') ADVANCE(275);
      END_STATE();
    case 21:
      if (lookahead == '-') ADVANCE(265);
      END_STATE();
    case 22:
      if (lookahead == '-') ADVANCE(94);
      END_STATE();
    case 23:
      if (lookahead == '-') ADVANCE(161);
      END_STATE();
    case 24:
      if (lookahead == '-') ADVANCE(202);
      END_STATE();
    case 25:
      if (lookahead == '-') ADVANCE(251);
      END_STATE();
    case 26:
      if (lookahead == '-') ADVANCE(107);
      END_STATE();
    case 27:
      if (lookahead == '-') ADVANCE(182);
      END_STATE();
    case 28:
      if (lookahead == '-') ADVANCE(119);
      END_STATE();
    case 29:
      if (lookahead == '/') ADVANCE(31);
      END_STATE();
    case 30:
      if (lookahead == '/') ADVANCE(71);
      END_STATE();
    case 31:
      if (lookahead == 'a') ADVANCE(54);
      if (lookahead == 'b') ADVANCE(195);
      if (lookahead == 'c') ADVANCE(154);
      if (lookahead == 'd') ADVANCE(132);
      if (lookahead == 'f') ADVANCE(198);
      if (lookahead == 'l') ADVANCE(91);
      if (lookahead == 'm') ADVANCE(133);
      if (lookahead == 'n') ADVANCE(35);
      if (lookahead == 'r') ADVANCE(92);
      if (lookahead == 's') ADVANCE(55);
      if (lookahead == 't') ADVANCE(32);
      if (lookahead == 'x') ADVANCE(146);
      END_STATE();
    case 32:
      if (lookahead == 'a') ADVANCE(125);
      END_STATE();
    case 33:
      if (lookahead == 'a') ADVANCE(121);
      END_STATE();
    case 34:
      if (lookahead == 'a') ADVANCE(52);
      END_STATE();
    case 35:
      if (lookahead == 'a') ADVANCE(259);
      END_STATE();
    case 36:
      if (lookahead == 'a') ADVANCE(292);
      END_STATE();
    case 37:
      if (lookahead == 'a') ADVANCE(250);
      END_STATE();
    case 38:
      if (lookahead == 'a') ADVANCE(220);
      END_STATE();
    case 39:
      if (lookahead == 'a') ADVANCE(269);
      END_STATE();
    case 40:
      if (lookahead == 'a') ADVANCE(62);
      END_STATE();
    case 41:
      if (lookahead == 'a') ADVANCE(126);
      END_STATE();
    case 42:
      if (lookahead == 'a') ADVANCE(192);
      END_STATE();
    case 43:
      if (lookahead == 'a') ADVANCE(68);
      END_STATE();
    case 44:
      if (lookahead == 'a') ADVANCE(268);
      END_STATE();
    case 45:
      if (lookahead == 'a') ADVANCE(82);
      END_STATE();
    case 46:
      if (lookahead == 'a') ADVANCE(271);
      END_STATE();
    case 47:
      if (lookahead == 'a') ADVANCE(170);
      END_STATE();
    case 48:
      if (lookahead == 'b') ADVANCE(122);
      END_STATE();
    case 49:
      if (lookahead == 'b') ADVANCE(283);
      END_STATE();
    case 50:
      if (lookahead == 'b') ADVANCE(24);
      END_STATE();
    case 51:
      if (lookahead == 'b') ADVANCE(142);
      END_STATE();
    case 52:
      if (lookahead == 'b') ADVANCE(163);
      END_STATE();
    case 53:
      if (lookahead == 'c') ADVANCE(278);
      END_STATE();
    case 54:
      if (lookahead == 'c') ADVANCE(56);
      END_STATE();
    case 55:
      if (lookahead == 'c') ADVANCE(232);
      if (lookahead == 'e') ADVANCE(186);
      if (lookahead == 'l') ADVANCE(199);
      END_STATE();
    case 56:
      if (lookahead == 'c') ADVANCE(95);
      END_STATE();
    case 57:
      if (lookahead == 'c') ADVANCE(150);
      END_STATE();
    case 58:
      if (lookahead == 'c') ADVANCE(147);
      END_STATE();
    case 59:
      if (lookahead == 'c') ADVANCE(200);
      END_STATE();
    case 60:
      if (lookahead == 'c') ADVANCE(148);
      END_STATE();
    case 61:
      if (lookahead == 'c') ADVANCE(239);
      END_STATE();
    case 62:
      if (lookahead == 'c') ADVANCE(151);
      END_STATE();
    case 63:
      if (lookahead == 'c') ADVANCE(280);
      END_STATE();
    case 64:
      if (lookahead == 'c') ADVANCE(205);
      END_STATE();
    case 65:
      if (lookahead == 'c') ADVANCE(171);
      if (lookahead == 'd') ADVANCE(237);
      END_STATE();
    case 66:
      if (lookahead == 'c') ADVANCE(282);
      END_STATE();
    case 67:
      if (lookahead == 'c') ADVANCE(213);
      if (lookahead == 'p') ADVANCE(290);
      END_STATE();
    case 68:
      if (lookahead == 'c') ADVANCE(105);
      END_STATE();
    case 69:
      if (lookahead == 'c') ADVANCE(241);
      END_STATE();
    case 70:
      if (lookahead == 'd') ADVANCE(194);
      END_STATE();
    case 71:
      if (lookahead == 'd') ADVANCE(287);
      END_STATE();
    case 72:
      if (lookahead == 'd') ADVANCE(298);
      END_STATE();
    case 73:
      if (lookahead == 'd') ADVANCE(308);
      END_STATE();
    case 74:
      if (lookahead == 'd') ADVANCE(303);
      END_STATE();
    case 75:
      if (lookahead == 'd') ADVANCE(316);
      END_STATE();
    case 76:
      if (lookahead == 'd') ADVANCE(149);
      if (lookahead == 'u') ADVANCE(249);
      END_STATE();
    case 77:
      if (lookahead == 'd') ADVANCE(83);
      END_STATE();
    case 78:
      if (lookahead == 'd') ADVANCE(93);
      END_STATE();
    case 79:
      if (lookahead == 'd') ADVANCE(26);
      END_STATE();
    case 80:
      if (lookahead == 'd') ADVANCE(97);
      END_STATE();
    case 81:
      if (lookahead == 'd') ADVANCE(90);
      END_STATE();
    case 82:
      if (lookahead == 'd') ADVANCE(10);
      END_STATE();
    case 83:
      if (lookahead == 'd') ADVANCE(173);
      END_STATE();
    case 84:
      if (lookahead == 'd') ADVANCE(100);
      if (lookahead == 'r') ADVANCE(39);
      END_STATE();
    case 85:
      if (lookahead == 'e') ADVANCE(242);
      END_STATE();
    case 86:
      if (lookahead == 'e') ADVANCE(291);
      END_STATE();
    case 87:
      if (lookahead == 'e') ADVANCE(286);
      END_STATE();
    case 88:
      if (lookahead == 'e') ADVANCE(315);
      END_STATE();
    case 89:
      if (lookahead == 'e') ADVANCE(297);
      END_STATE();
    case 90:
      if (lookahead == 'e') ADVANCE(317);
      END_STATE();
    case 91:
      if (lookahead == 'e') ADVANCE(114);
      if (lookahead == 'o') ADVANCE(120);
      END_STATE();
    case 92:
      if (lookahead == 'e') ADVANCE(222);
      END_STATE();
    case 93:
      if (lookahead == 'e') ADVANCE(240);
      END_STATE();
    case 94:
      if (lookahead == 'e') ADVANCE(178);
      END_STATE();
    case 95:
      if (lookahead == 'e') ADVANCE(162);
      END_STATE();
    case 96:
      if (lookahead == 'e') ADVANCE(72);
      END_STATE();
    case 97:
      if (lookahead == 'e') ADVANCE(73);
      END_STATE();
    case 98:
      if (lookahead == 'e') ADVANCE(260);
      END_STATE();
    case 99:
      if (lookahead == 'e') ADVANCE(153);
      END_STATE();
    case 100:
      if (lookahead == 'e') ADVANCE(166);
      END_STATE();
    case 101:
      if (lookahead == 'e') ADVANCE(9);
      END_STATE();
    case 102:
      if (lookahead == 'e') ADVANCE(108);
      END_STATE();
    case 103:
      if (lookahead == 'e') ADVANCE(96);
      END_STATE();
    case 104:
      if (lookahead == 'e') ADVANCE(11);
      END_STATE();
    case 105:
      if (lookahead == 'e') ADVANCE(8);
      END_STATE();
    case 106:
      if (lookahead == 'e') ADVANCE(44);
      END_STATE();
    case 107:
      if (lookahead == 'e') ADVANCE(285);
      END_STATE();
    case 108:
      if (lookahead == 'e') ADVANCE(191);
      END_STATE();
    case 109:
      if (lookahead == 'e') ADVANCE(189);
      END_STATE();
    case 110:
      if (lookahead == 'e') ADVANCE(20);
      END_STATE();
    case 111:
      if (lookahead == 'e') ADVANCE(270);
      END_STATE();
    case 112:
      if (lookahead == 'e') ADVANCE(21);
      END_STATE();
    case 113:
      if (lookahead == 'f') ADVANCE(131);
      END_STATE();
    case 114:
      if (lookahead == 'f') ADVANCE(267);
      END_STATE();
    case 115:
      if (lookahead == 'f') ADVANCE(211);
      END_STATE();
    case 116:
      if (lookahead == 'f') ADVANCE(43);
      END_STATE();
    case 117:
      if (lookahead == 'f') ADVANCE(144);
      END_STATE();
    case 118:
      if (lookahead == 'f') ADVANCE(217);
      END_STATE();
    case 119:
      if (lookahead == 'f') ADVANCE(215);
      END_STATE();
    case 120:
      if (lookahead == 'g') ADVANCE(23);
      END_STATE();
    case 121:
      if (lookahead == 'g') ADVANCE(321);
      END_STATE();
    case 122:
      if (lookahead == 'g') ADVANCE(307);
      END_STATE();
    case 123:
      if (lookahead == 'g') ADVANCE(313);
      END_STATE();
    case 124:
      if (lookahead == 'g') ADVANCE(304);
      END_STATE();
    case 125:
      if (lookahead == 'g') ADVANCE(15);
      if (lookahead == 'p') ADVANCE(16);
      END_STATE();
    case 126:
      if (lookahead == 'g') ADVANCE(17);
      END_STATE();
    case 127:
      if (lookahead == 'h') ADVANCE(42);
      END_STATE();
    case 128:
      if (lookahead == 'h') ADVANCE(208);
      END_STATE();
    case 129:
      if (lookahead == 'h') ADVANCE(209);
      END_STATE();
    case 130:
      if (lookahead == 'h') ADVANCE(145);
      END_STATE();
    case 131:
      if (lookahead == 'i') ADVANCE(152);
      END_STATE();
    case 132:
      if (lookahead == 'i') ADVANCE(246);
      if (lookahead == 'r') ADVANCE(41);
      END_STATE();
    case 133:
      if (lookahead == 'i') ADVANCE(77);
      if (lookahead == 'o') ADVANCE(76);
      END_STATE();
    case 134:
      if (lookahead == 'i') ADVANCE(57);
      END_STATE();
    case 135:
      if (lookahead == 'i') ADVANCE(184);
      END_STATE();
    case 136:
      if (lookahead == 'i') ADVANCE(51);
      END_STATE();
    case 137:
      if (lookahead == 'i') ADVANCE(185);
      END_STATE();
    case 138:
      if (lookahead == 'i') ADVANCE(201);
      END_STATE();
    case 139:
      if (lookahead == 'i') ADVANCE(60);
      END_STATE();
    case 140:
      if (lookahead == 'i') ADVANCE(247);
      END_STATE();
    case 141:
      if (lookahead == 'i') ADVANCE(264);
      END_STATE();
    case 142:
      if (lookahead == 'i') ADVANCE(160);
      END_STATE();
    case 143:
      if (lookahead == 'i') ADVANCE(203);
      END_STATE();
    case 144:
      if (lookahead == 'i') ADVANCE(169);
      END_STATE();
    case 145:
      if (lookahead == 'i') ADVANCE(176);
      END_STATE();
    case 146:
      if (lookahead == 'k') ADVANCE(50);
      END_STATE();
    case 147:
      if (lookahead == 'k') ADVANCE(305);
      END_STATE();
    case 148:
      if (lookahead == 'k') ADVANCE(320);
      END_STATE();
    case 149:
      if (lookahead == 'k') ADVANCE(86);
      END_STATE();
    case 150:
      if (lookahead == 'k') ADVANCE(3);
      END_STATE();
    case 151:
      if (lookahead == 'k') ADVANCE(226);
      END_STATE();
    case 152:
      if (lookahead == 'l') ADVANCE(85);
      END_STATE();
    case 153:
      if (lookahead == 'l') ADVANCE(309);
      END_STATE();
    case 154:
      if (lookahead == 'l') ADVANCE(134);
      END_STATE();
    case 155:
      if (lookahead == 'l') ADVANCE(29);
      END_STATE();
    case 156:
      if (lookahead == 'l') ADVANCE(135);
      END_STATE();
    case 157:
      if (lookahead == 'l') ADVANCE(158);
      END_STATE();
    case 158:
      if (lookahead == 'l') ADVANCE(248);
      END_STATE();
    case 159:
      if (lookahead == 'l') ADVANCE(206);
      END_STATE();
    case 160:
      if (lookahead == 'l') ADVANCE(141);
      END_STATE();
    case 161:
      if (lookahead == 'l') ADVANCE(87);
      END_STATE();
    case 162:
      if (lookahead == 'l') ADVANCE(2);
      END_STATE();
    case 163:
      if (lookahead == 'l') ADVANCE(110);
      END_STATE();
    case 164:
      if (lookahead == 'l') ADVANCE(204);
      END_STATE();
    case 165:
      if (lookahead == 'l') ADVANCE(156);
      END_STATE();
    case 166:
      if (lookahead == 'l') ADVANCE(36);
      END_STATE();
    case 167:
      if (lookahead == 'l') ADVANCE(207);
      END_STATE();
    case 168:
      if (lookahead == 'l') ADVANCE(196);
      END_STATE();
    case 169:
      if (lookahead == 'l') ADVANCE(89);
      END_STATE();
    case 170:
      if (lookahead == 'l') ADVANCE(25);
      END_STATE();
    case 171:
      if (lookahead == 'l') ADVANCE(139);
      END_STATE();
    case 172:
      if (lookahead == 'l') ADVANCE(177);
      END_STATE();
    case 173:
      if (lookahead == 'l') ADVANCE(104);
      END_STATE();
    case 174:
      if (lookahead == 'l') ADVANCE(168);
      END_STATE();
    case 175:
      if (lookahead == 'l') ADVANCE(46);
      END_STATE();
    case 176:
      if (lookahead == 'l') ADVANCE(112);
      END_STATE();
    case 177:
      if (lookahead == 'l') ADVANCE(27);
      END_STATE();
    case 178:
      if (lookahead == 'm') ADVANCE(281);
      END_STATE();
    case 179:
      if (lookahead == 'm') ADVANCE(38);
      END_STATE();
    case 180:
      if (lookahead == 'm') ADVANCE(98);
      END_STATE();
    case 181:
      if (lookahead == 'm') ADVANCE(216);
      END_STATE();
    case 182:
      if (lookahead == 'm') ADVANCE(111);
      END_STATE();
    case 183:
      if (lookahead == 'n') ADVANCE(310);
      END_STATE();
    case 184:
      if (lookahead == 'n') ADVANCE(123);
      END_STATE();
    case 185:
      if (lookahead == 'n') ADVANCE(124);
      END_STATE();
    case 186:
      if (lookahead == 'n') ADVANCE(79);
      END_STATE();
    case 187:
      if (lookahead == 'n') ADVANCE(243);
      END_STATE();
    case 188:
      if (lookahead == 'n') ADVANCE(258);
      END_STATE();
    case 189:
      if (lookahead == 'n') ADVANCE(274);
      END_STATE();
    case 190:
      if (lookahead == 'n') ADVANCE(7);
      END_STATE();
    case 191:
      if (lookahead == 'n') ADVANCE(14);
      END_STATE();
    case 192:
      if (lookahead == 'n') ADVANCE(80);
      END_STATE();
    case 193:
      if (lookahead == 'n') ADVANCE(22);
      END_STATE();
    case 194:
      if (lookahead == 'o') ADVANCE(257);
      END_STATE();
    case 195:
      if (lookahead == 'o') ADVANCE(233);
      if (lookahead == 'u') ADVANCE(263);
      if (lookahead == 'y') ADVANCE(223);
      END_STATE();
    case 196:
      if (lookahead == 'o') ADVANCE(289);
      END_STATE();
    case 197:
      if (lookahead == 'o') ADVANCE(190);
      END_STATE();
    case 198:
      if (lookahead == 'o') ADVANCE(53);
      if (lookahead == 'u') ADVANCE(157);
      END_STATE();
    case 199:
      if (lookahead == 'o') ADVANCE(224);
      END_STATE();
    case 200:
      if (lookahead == 'o') ADVANCE(277);
      END_STATE();
    case 201:
      if (lookahead == 'o') ADVANCE(187);
      END_STATE();
    case 202:
      if (lookahead == 'o') ADVANCE(225);
      END_STATE();
    case 203:
      if (lookahead == 'o') ADVANCE(183);
      END_STATE();
    case 204:
      if (lookahead == 'o') ADVANCE(230);
      END_STATE();
    case 205:
      if (lookahead == 'o') ADVANCE(164);
      END_STATE();
    case 206:
      if (lookahead == 'o') ADVANCE(58);
      END_STATE();
    case 207:
      if (lookahead == 'o') ADVANCE(231);
      END_STATE();
    case 208:
      if (lookahead == 'o') ADVANCE(74);
      END_STATE();
    case 209:
      if (lookahead == 'o') ADVANCE(75);
      END_STATE();
    case 210:
      if (lookahead == 'o') ADVANCE(6);
      END_STATE();
    case 211:
      if (lookahead == 'o') ADVANCE(63);
      END_STATE();
    case 212:
      if (lookahead == 'o') ADVANCE(172);
      END_STATE();
    case 213:
      if (lookahead == 'o') ADVANCE(167);
      END_STATE();
    case 214:
      if (lookahead == 'o') ADVANCE(193);
      END_STATE();
    case 215:
      if (lookahead == 'o') ADVANCE(66);
      END_STATE();
    case 216:
      if (lookahead == 'o') ADVANCE(81);
      END_STATE();
    case 217:
      if (lookahead == 'o') ADVANCE(174);
      END_STATE();
    case 218:
      if (lookahead == 'o') ADVANCE(165);
      END_STATE();
    case 219:
      if (lookahead == 'o') ADVANCE(117);
      END_STATE();
    case 220:
      if (lookahead == 'p') ADVANCE(301);
      END_STATE();
    case 221:
      if (lookahead == 'p') ADVANCE(295);
      END_STATE();
    case 222:
      if (lookahead == 'p') ADVANCE(106);
      END_STATE();
    case 223:
      if (lookahead == 'p') ADVANCE(37);
      END_STATE();
    case 224:
      if (lookahead == 'p') ADVANCE(221);
      END_STATE();
    case 225:
      if (lookahead == 'p') ADVANCE(261);
      END_STATE();
    case 226:
      if (lookahead == 'p') ADVANCE(45);
      END_STATE();
    case 227:
      if (lookahead == 'p') ADVANCE(236);
      if (lookahead == 's') ADVANCE(229);
      END_STATE();
    case 228:
      if (lookahead == 'p') ADVANCE(137);
      END_STATE();
    case 229:
      if (lookahead == 'p') ADVANCE(103);
      END_STATE();
    case 230:
      if (lookahead == 'r') ADVANCE(306);
      END_STATE();
    case 231:
      if (lookahead == 'r') ADVANCE(299);
      END_STATE();
    case 232:
      if (lookahead == 'r') ADVANCE(212);
      END_STATE();
    case 233:
      if (lookahead == 'r') ADVANCE(78);
      END_STATE();
    case 234:
      if (lookahead == 'r') ADVANCE(116);
      END_STATE();
    case 235:
      if (lookahead == 'r') ADVANCE(47);
      END_STATE();
    case 236:
      if (lookahead == 'r') ADVANCE(219);
      END_STATE();
    case 237:
      if (lookahead == 'r') ADVANCE(33);
      END_STATE();
    case 238:
      if (lookahead == 'r') ADVANCE(40);
      END_STATE();
    case 239:
      if (lookahead == 'r') ADVANCE(102);
      END_STATE();
    case 240:
      if (lookahead == 'r') ADVANCE(4);
      END_STATE();
    case 241:
      if (lookahead == 'r') ADVANCE(218);
      END_STATE();
    case 242:
      if (lookahead == 's') ADVANCE(30);
      END_STATE();
    case 243:
      if (lookahead == 's') ADVANCE(322);
      END_STATE();
    case 244:
      if (lookahead == 's') ADVANCE(318);
      END_STATE();
    case 245:
      if (lookahead == 's') ADVANCE(312);
      END_STATE();
    case 246:
      if (lookahead == 's') ADVANCE(34);
      END_STATE();
    case 247:
      if (lookahead == 's') ADVANCE(136);
      END_STATE();
    case 248:
      if (lookahead == 's') ADVANCE(61);
      END_STATE();
    case 249:
      if (lookahead == 's') ADVANCE(101);
      END_STATE();
    case 250:
      if (lookahead == 's') ADVANCE(252);
      END_STATE();
    case 251:
      if (lookahead == 's') ADVANCE(69);
      END_STATE();
    case 252:
      if (lookahead == 's') ADVANCE(19);
      END_STATE();
    case 253:
      if (lookahead == 's') ADVANCE(12);
      END_STATE();
    case 254:
      if (lookahead == 's') ADVANCE(279);
      END_STATE();
    case 255:
      if (lookahead == 's') ADVANCE(18);
      END_STATE();
    case 256:
      if (lookahead == 's') ADVANCE(28);
      END_STATE();
    case 257:
      if (lookahead == 't') ADVANCE(113);
      END_STATE();
    case 258:
      if (lookahead == 't') ADVANCE(319);
      END_STATE();
    case 259:
      if (lookahead == 't') ADVANCE(276);
      END_STATE();
    case 260:
      if (lookahead == 't') ADVANCE(128);
      END_STATE();
    case 261:
      if (lookahead == 't') ADVANCE(138);
      END_STATE();
    case 262:
      if (lookahead == 't') ADVANCE(197);
      END_STATE();
    case 263:
      if (lookahead == 't') ADVANCE(262);
      END_STATE();
    case 264:
      if (lookahead == 't') ADVANCE(293);
      END_STATE();
    case 265:
      if (lookahead == 't') ADVANCE(294);
      END_STATE();
    case 266:
      if (lookahead == 't') ADVANCE(210);
      END_STATE();
    case 267:
      if (lookahead == 't') ADVANCE(1);
      END_STATE();
    case 268:
      if (lookahead == 't') ADVANCE(5);
      END_STATE();
    case 269:
      if (lookahead == 't') ADVANCE(88);
      END_STATE();
    case 270:
      if (lookahead == 't') ADVANCE(129);
      END_STATE();
    case 271:
      if (lookahead == 't') ADVANCE(143);
      END_STATE();
    case 272:
      if (lookahead == 't') ADVANCE(214);
      END_STATE();
    case 273:
      if (lookahead == 't') ADVANCE(272);
      END_STATE();
    case 274:
      if (lookahead == 't') ADVANCE(253);
      END_STATE();
    case 275:
      if (lookahead == 't') ADVANCE(238);
      END_STATE();
    case 276:
      if (lookahead == 'u') ADVANCE(235);
      END_STATE();
    case 277:
      if (lookahead == 'u') ADVANCE(188);
      END_STATE();
    case 278:
      if (lookahead == 'u') ADVANCE(255);
      END_STATE();
    case 279:
      if (lookahead == 'u') ADVANCE(234);
      END_STATE();
    case 280:
      if (lookahead == 'u') ADVANCE(244);
      END_STATE();
    case 281:
      if (lookahead == 'u') ADVANCE(175);
      END_STATE();
    case 282:
      if (lookahead == 'u') ADVANCE(245);
      END_STATE();
    case 283:
      if (lookahead == 'u') ADVANCE(273);
      END_STATE();
    case 284:
      if (lookahead == 'v') ADVANCE(140);
      END_STATE();
    case 285:
      if (lookahead == 'v') ADVANCE(109);
      END_STATE();
    case 286:
      if (lookahead == 'v') ADVANCE(99);
      END_STATE();
    case 287:
      if (lookahead == 'w') ADVANCE(155);
      END_STATE();
    case 288:
      if (lookahead == 'w') ADVANCE(130);
      END_STATE();
    case 289:
      if (lookahead == 'w') ADVANCE(256);
      END_STATE();
    case 290:
      if (lookahead == 'x') ADVANCE(300);
      END_STATE();
    case 291:
      if (lookahead == 'y') ADVANCE(311);
      END_STATE();
    case 292:
      if (lookahead == 'y') ADVANCE(314);
      END_STATE();
    case 293:
      if (lookahead == 'y') ADVANCE(302);
      END_STATE();
    case 294:
      if (lookahead == 'y') ADVANCE(228);
      END_STATE();
    case 295:
      if (lookahead == 'y') ADVANCE(13);
      END_STATE();
    case 296:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 297:
      ACCEPT_TOKEN(sym_accel_profile);
      END_STATE();
    case 298:
      ACCEPT_TOKEN(sym_accel_speed);
      END_STATE();
    case 299:
      ACCEPT_TOKEN(sym_border_color);
      END_STATE();
    case 300:
      ACCEPT_TOKEN(sym_border_px);
      END_STATE();
    case 301:
      ACCEPT_TOKEN(sym_button_map);
      END_STATE();
    case 302:
      ACCEPT_TOKEN(sym_bypass_surface_visibility);
      END_STATE();
    case 303:
      ACCEPT_TOKEN(sym_click_method);
      END_STATE();
    case 304:
      ACCEPT_TOKEN(sym_disable_trackpad_while_typing);
      END_STATE();
    case 305:
      ACCEPT_TOKEN(sym_drag_lock);
      END_STATE();
    case 306:
      ACCEPT_TOKEN(sym_focus_color);
      END_STATE();
    case 307:
      ACCEPT_TOKEN(sym_fullscreen_bg);
      END_STATE();
    case 308:
      ACCEPT_TOKEN(sym_left_handed);
      END_STATE();
    case 309:
      ACCEPT_TOKEN(sym_log_level);
      END_STATE();
    case 310:
      ACCEPT_TOKEN(sym_middle_button_emulation);
      END_STATE();
    case 311:
      ACCEPT_TOKEN(sym_modkey);
      END_STATE();
    case 312:
      ACCEPT_TOKEN(sym_mouse_follows_focus);
      END_STATE();
    case 313:
      ACCEPT_TOKEN(sym_natural_scrolling);
      END_STATE();
    case 314:
      ACCEPT_TOKEN(sym_repeat_delay);
      END_STATE();
    case 315:
      ACCEPT_TOKEN(sym_repeat_rate);
      END_STATE();
    case 316:
      ACCEPT_TOKEN(sym_scroll_method);
      END_STATE();
    case 317:
      ACCEPT_TOKEN(sym_send_events_mode);
      END_STATE();
    case 318:
      ACCEPT_TOKEN(sym_sloppy_focus);
      END_STATE();
    case 319:
      ACCEPT_TOKEN(sym_tag_count);
      END_STATE();
    case 320:
      ACCEPT_TOKEN(sym_tap_to_click);
      END_STATE();
    case 321:
      ACCEPT_TOKEN(sym_tap_to_drag);
      END_STATE();
    case 322:
      ACCEPT_TOKEN(sym_xkb_options);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_accel_profile] = ACTIONS(1),
    [sym_accel_speed] = ACTIONS(1),
    [sym_border_color] = ACTIONS(1),
    [sym_border_px] = ACTIONS(1),
    [sym_button_map] = ACTIONS(1),
    [sym_bypass_surface_visibility] = ACTIONS(1),
    [sym_click_method] = ACTIONS(1),
    [sym_disable_trackpad_while_typing] = ACTIONS(1),
    [sym_drag_lock] = ACTIONS(1),
    [sym_focus_color] = ACTIONS(1),
    [sym_fullscreen_bg] = ACTIONS(1),
    [sym_left_handed] = ACTIONS(1),
    [sym_log_level] = ACTIONS(1),
    [sym_middle_button_emulation] = ACTIONS(1),
    [sym_modkey] = ACTIONS(1),
    [sym_mouse_follows_focus] = ACTIONS(1),
    [sym_natural_scrolling] = ACTIONS(1),
    [sym_repeat_delay] = ACTIONS(1),
    [sym_repeat_rate] = ACTIONS(1),
    [sym_scroll_method] = ACTIONS(1),
    [sym_send_events_mode] = ACTIONS(1),
    [sym_sloppy_focus] = ACTIONS(1),
    [sym_tag_count] = ACTIONS(1),
    [sym_tap_to_click] = ACTIONS(1),
    [sym_tap_to_drag] = ACTIONS(1),
    [sym_xkb_options] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(3),
    [sym_accel_profile] = ACTIONS(3),
    [sym_accel_speed] = ACTIONS(3),
    [sym_border_color] = ACTIONS(3),
    [sym_border_px] = ACTIONS(3),
    [sym_button_map] = ACTIONS(3),
    [sym_bypass_surface_visibility] = ACTIONS(3),
    [sym_click_method] = ACTIONS(3),
    [sym_disable_trackpad_while_typing] = ACTIONS(3),
    [sym_drag_lock] = ACTIONS(3),
    [sym_focus_color] = ACTIONS(3),
    [sym_fullscreen_bg] = ACTIONS(3),
    [sym_left_handed] = ACTIONS(3),
    [sym_log_level] = ACTIONS(3),
    [sym_middle_button_emulation] = ACTIONS(3),
    [sym_modkey] = ACTIONS(3),
    [sym_mouse_follows_focus] = ACTIONS(3),
    [sym_natural_scrolling] = ACTIONS(3),
    [sym_repeat_delay] = ACTIONS(3),
    [sym_repeat_rate] = ACTIONS(3),
    [sym_scroll_method] = ACTIONS(3),
    [sym_send_events_mode] = ACTIONS(3),
    [sym_sloppy_focus] = ACTIONS(3),
    [sym_tag_count] = ACTIONS(3),
    [sym_tap_to_click] = ACTIONS(3),
    [sym_tap_to_drag] = ACTIONS(3),
    [sym_xkb_options] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 1,
    ACTIONS(5), 1,
      ts_builtin_sym_end,
  [4] = 1,
    ACTIONS(7), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 4,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [7] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_dconfdwl(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
