#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 4
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 22
#define ALIAS_COUNT 0
#define TOKEN_COUNT 21
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 1
#define PRODUCTION_ID_COUNT 1

enum {
  sym_font = 1,
  sym_font_fallback = 2,
  sym_time_block = 3,
  sym_date_fmt = 4,
  sym_browser_path = 5,
  sym_browser = 6,
  sym_time_fmt = 7,
  sym_update_time_ntp = 8,
  sym_brightness_block = 9,
  sym_battery_block = 10,
  sym_connman_block = 11,
  sym_media_block = 12,
  sym_color_active = 13,
  sym_color_inactive = 14,
  sym_padding_x = 15,
  sym_padding_y = 16,
  sym_top_bar = 17,
  sym_time_servers = 18,
  sym_bar_show_time = 19,
  sym_divider = 20,
  sym_source_file = 21,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_font] = "font",
  [sym_font_fallback] = "font_fallback",
  [sym_time_block] = "time_block",
  [sym_date_fmt] = "date_fmt",
  [sym_browser_path] = "browser_path",
  [sym_browser] = "browser",
  [sym_time_fmt] = "time_fmt",
  [sym_update_time_ntp] = "update_time_ntp",
  [sym_brightness_block] = "brightness_block",
  [sym_battery_block] = "battery_block",
  [sym_connman_block] = "connman_block",
  [sym_media_block] = "media_block",
  [sym_color_active] = "color_active",
  [sym_color_inactive] = "color_inactive",
  [sym_padding_x] = "padding_x",
  [sym_padding_y] = "padding_y",
  [sym_top_bar] = "top_bar",
  [sym_time_servers] = "time_servers",
  [sym_bar_show_time] = "bar_show_time",
  [sym_divider] = "divider",
  [sym_source_file] = "source_file",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_font] = sym_font,
  [sym_font_fallback] = sym_font_fallback,
  [sym_time_block] = sym_time_block,
  [sym_date_fmt] = sym_date_fmt,
  [sym_browser_path] = sym_browser_path,
  [sym_browser] = sym_browser,
  [sym_time_fmt] = sym_time_fmt,
  [sym_update_time_ntp] = sym_update_time_ntp,
  [sym_brightness_block] = sym_brightness_block,
  [sym_battery_block] = sym_battery_block,
  [sym_connman_block] = sym_connman_block,
  [sym_media_block] = sym_media_block,
  [sym_color_active] = sym_color_active,
  [sym_color_inactive] = sym_color_inactive,
  [sym_padding_x] = sym_padding_x,
  [sym_padding_y] = sym_padding_y,
  [sym_top_bar] = sym_top_bar,
  [sym_time_servers] = sym_time_servers,
  [sym_bar_show_time] = sym_bar_show_time,
  [sym_divider] = sym_divider,
  [sym_source_file] = sym_source_file,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_font] = {
    .visible = true,
    .named = true,
  },
  [sym_font_fallback] = {
    .visible = true,
    .named = true,
  },
  [sym_time_block] = {
    .visible = true,
    .named = true,
  },
  [sym_date_fmt] = {
    .visible = true,
    .named = true,
  },
  [sym_browser_path] = {
    .visible = true,
    .named = true,
  },
  [sym_browser] = {
    .visible = true,
    .named = true,
  },
  [sym_time_fmt] = {
    .visible = true,
    .named = true,
  },
  [sym_update_time_ntp] = {
    .visible = true,
    .named = true,
  },
  [sym_brightness_block] = {
    .visible = true,
    .named = true,
  },
  [sym_battery_block] = {
    .visible = true,
    .named = true,
  },
  [sym_connman_block] = {
    .visible = true,
    .named = true,
  },
  [sym_media_block] = {
    .visible = true,
    .named = true,
  },
  [sym_color_active] = {
    .visible = true,
    .named = true,
  },
  [sym_color_inactive] = {
    .visible = true,
    .named = true,
  },
  [sym_padding_x] = {
    .visible = true,
    .named = true,
  },
  [sym_padding_y] = {
    .visible = true,
    .named = true,
  },
  [sym_top_bar] = {
    .visible = true,
    .named = true,
  },
  [sym_time_servers] = {
    .visible = true,
    .named = true,
  },
  [sym_bar_show_time] = {
    .visible = true,
    .named = true,
  },
  [sym_divider] = {
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
      if (eof) ADVANCE(168);
      if (lookahead == '/') ADVANCE(46);
      END_STATE();
    case 1:
      if (lookahead == '-') ADVANCE(31);
      END_STATE();
    case 2:
      if (lookahead == '-') ADVANCE(166);
      END_STATE();
    case 3:
      if (lookahead == '-') ADVANCE(69);
      END_STATE();
    case 4:
      if (lookahead == '-') ADVANCE(34);
      END_STATE();
    case 5:
      if (lookahead == '-') ADVANCE(18);
      END_STATE();
    case 6:
      if (lookahead == '-') ADVANCE(142);
      END_STATE();
    case 7:
      if (lookahead == '-') ADVANCE(154);
      END_STATE();
    case 8:
      if (lookahead == '-') ADVANCE(112);
      END_STATE();
    case 9:
      if (lookahead == '-') ADVANCE(33);
      END_STATE();
    case 10:
      if (lookahead == '-') ADVANCE(35);
      END_STATE();
    case 11:
      if (lookahead == '-') ADVANCE(158);
      END_STATE();
    case 12:
      if (lookahead == '-') ADVANCE(36);
      END_STATE();
    case 13:
      if (lookahead == '-') ADVANCE(37);
      END_STATE();
    case 14:
      if (lookahead == '/') ADVANCE(30);
      END_STATE();
    case 15:
      if (lookahead == '/') ADVANCE(141);
      END_STATE();
    case 16:
      if (lookahead == 'a') ADVANCE(135);
      END_STATE();
    case 17:
      if (lookahead == 'a') ADVANCE(130);
      if (lookahead == 'r') ADVANCE(77);
      END_STATE();
    case 18:
      if (lookahead == 'a') ADVANCE(44);
      if (lookahead == 'i') ADVANCE(111);
      END_STATE();
    case 19:
      if (lookahead == 'a') ADVANCE(151);
      if (lookahead == 'i') ADVANCE(160);
      END_STATE();
    case 20:
      if (lookahead == 'a') ADVANCE(51);
      END_STATE();
    case 21:
      if (lookahead == 'a') ADVANCE(93);
      END_STATE();
    case 22:
      if (lookahead == 'a') ADVANCE(114);
      END_STATE();
    case 23:
      if (lookahead == 'a') ADVANCE(134);
      END_STATE();
    case 24:
      if (lookahead == 'a') ADVANCE(4);
      END_STATE();
    case 25:
      if (lookahead == 'a') ADVANCE(42);
      END_STATE();
    case 26:
      if (lookahead == 'a') ADVANCE(150);
      END_STATE();
    case 27:
      if (lookahead == 'a') ADVANCE(157);
      END_STATE();
    case 28:
      if (lookahead == 'a') ADVANCE(45);
      END_STATE();
    case 29:
      if (lookahead == 'b') ADVANCE(16);
      END_STATE();
    case 30:
      if (lookahead == 'b') ADVANCE(17);
      if (lookahead == 'c') ADVANCE(117);
      if (lookahead == 'd') ADVANCE(19);
      if (lookahead == 'f') ADVANCE(118);
      if (lookahead == 'm') ADVANCE(57);
      if (lookahead == 'p') ADVANCE(20);
      if (lookahead == 't') ADVANCE(81);
      if (lookahead == 'u') ADVANCE(128);
      END_STATE();
    case 31:
      if (lookahead == 'b') ADVANCE(95);
      if (lookahead == 'f') ADVANCE(105);
      if (lookahead == 's') ADVANCE(64);
      END_STATE();
    case 32:
      if (lookahead == 'b') ADVANCE(25);
      END_STATE();
    case 33:
      if (lookahead == 'b') ADVANCE(23);
      END_STATE();
    case 34:
      if (lookahead == 'b') ADVANCE(96);
      END_STATE();
    case 35:
      if (lookahead == 'b') ADVANCE(97);
      END_STATE();
    case 36:
      if (lookahead == 'b') ADVANCE(98);
      END_STATE();
    case 37:
      if (lookahead == 'b') ADVANCE(99);
      END_STATE();
    case 38:
      if (lookahead == 'c') ADVANCE(86);
      END_STATE();
    case 39:
      if (lookahead == 'c') ADVANCE(87);
      END_STATE();
    case 40:
      if (lookahead == 'c') ADVANCE(88);
      END_STATE();
    case 41:
      if (lookahead == 'c') ADVANCE(89);
      END_STATE();
    case 42:
      if (lookahead == 'c') ADVANCE(90);
      END_STATE();
    case 43:
      if (lookahead == 'c') ADVANCE(91);
      END_STATE();
    case 44:
      if (lookahead == 'c') ADVANCE(155);
      END_STATE();
    case 45:
      if (lookahead == 'c') ADVANCE(159);
      END_STATE();
    case 46:
      if (lookahead == 'd') ADVANCE(115);
      END_STATE();
    case 47:
      if (lookahead == 'd') ADVANCE(79);
      END_STATE();
    case 48:
      if (lookahead == 'd') ADVANCE(27);
      END_STATE();
    case 49:
      if (lookahead == 'd') ADVANCE(78);
      END_STATE();
    case 50:
      if (lookahead == 'd') ADVANCE(63);
      END_STATE();
    case 51:
      if (lookahead == 'd') ADVANCE(49);
      END_STATE();
    case 52:
      if (lookahead == 'e') ADVANCE(139);
      END_STATE();
    case 53:
      if (lookahead == 'e') ADVANCE(29);
      END_STATE();
    case 54:
      if (lookahead == 'e') ADVANCE(181);
      END_STATE();
    case 55:
      if (lookahead == 'e') ADVANCE(187);
      END_STATE();
    case 56:
      if (lookahead == 'e') ADVANCE(182);
      END_STATE();
    case 57:
      if (lookahead == 'e') ADVANCE(47);
      END_STATE();
    case 58:
      if (lookahead == 'e') ADVANCE(131);
      END_STATE();
    case 59:
      if (lookahead == 'e') ADVANCE(3);
      END_STATE();
    case 60:
      if (lookahead == 'e') ADVANCE(132);
      END_STATE();
    case 61:
      if (lookahead == 'e') ADVANCE(1);
      END_STATE();
    case 62:
      if (lookahead == 'e') ADVANCE(143);
      END_STATE();
    case 63:
      if (lookahead == 'e') ADVANCE(133);
      END_STATE();
    case 64:
      if (lookahead == 'e') ADVANCE(136);
      END_STATE();
    case 65:
      if (lookahead == 'e') ADVANCE(7);
      END_STATE();
    case 66:
      if (lookahead == 'e') ADVANCE(138);
      END_STATE();
    case 67:
      if (lookahead == 'e') ADVANCE(8);
      END_STATE();
    case 68:
      if (lookahead == 'f') ADVANCE(76);
      END_STATE();
    case 69:
      if (lookahead == 'f') ADVANCE(102);
      END_STATE();
    case 70:
      if (lookahead == 'f') ADVANCE(21);
      END_STATE();
    case 71:
      if (lookahead == 'g') ADVANCE(74);
      END_STATE();
    case 72:
      if (lookahead == 'g') ADVANCE(2);
      END_STATE();
    case 73:
      if (lookahead == 'h') ADVANCE(173);
      END_STATE();
    case 74:
      if (lookahead == 'h') ADVANCE(153);
      END_STATE();
    case 75:
      if (lookahead == 'h') ADVANCE(120);
      END_STATE();
    case 76:
      if (lookahead == 'i') ADVANCE(92);
      END_STATE();
    case 77:
      if (lookahead == 'i') ADVANCE(71);
      if (lookahead == 'o') ADVANCE(164);
      END_STATE();
    case 78:
      if (lookahead == 'i') ADVANCE(109);
      END_STATE();
    case 79:
      if (lookahead == 'i') ADVANCE(24);
      END_STATE();
    case 80:
      if (lookahead == 'i') ADVANCE(50);
      END_STATE();
    case 81:
      if (lookahead == 'i') ADVANCE(103);
      if (lookahead == 'o') ADVANCE(127);
      END_STATE();
    case 82:
      if (lookahead == 'i') ADVANCE(162);
      END_STATE();
    case 83:
      if (lookahead == 'i') ADVANCE(106);
      END_STATE();
    case 84:
      if (lookahead == 'i') ADVANCE(163);
      END_STATE();
    case 85:
      if (lookahead == 'i') ADVANCE(107);
      END_STATE();
    case 86:
      if (lookahead == 'k') ADVANCE(171);
      END_STATE();
    case 87:
      if (lookahead == 'k') ADVANCE(180);
      END_STATE();
    case 88:
      if (lookahead == 'k') ADVANCE(178);
      END_STATE();
    case 89:
      if (lookahead == 'k') ADVANCE(179);
      END_STATE();
    case 90:
      if (lookahead == 'k') ADVANCE(170);
      END_STATE();
    case 91:
      if (lookahead == 'k') ADVANCE(177);
      END_STATE();
    case 92:
      if (lookahead == 'l') ADVANCE(52);
      END_STATE();
    case 93:
      if (lookahead == 'l') ADVANCE(100);
      END_STATE();
    case 94:
      if (lookahead == 'l') ADVANCE(119);
      if (lookahead == 'n') ADVANCE(108);
      END_STATE();
    case 95:
      if (lookahead == 'l') ADVANCE(121);
      END_STATE();
    case 96:
      if (lookahead == 'l') ADVANCE(122);
      END_STATE();
    case 97:
      if (lookahead == 'l') ADVANCE(123);
      END_STATE();
    case 98:
      if (lookahead == 'l') ADVANCE(124);
      END_STATE();
    case 99:
      if (lookahead == 'l') ADVANCE(125);
      END_STATE();
    case 100:
      if (lookahead == 'l') ADVANCE(32);
      END_STATE();
    case 101:
      if (lookahead == 'm') ADVANCE(53);
      END_STATE();
    case 102:
      if (lookahead == 'm') ADVANCE(148);
      END_STATE();
    case 103:
      if (lookahead == 'm') ADVANCE(61);
      END_STATE();
    case 104:
      if (lookahead == 'm') ADVANCE(22);
      END_STATE();
    case 105:
      if (lookahead == 'm') ADVANCE(149);
      END_STATE();
    case 106:
      if (lookahead == 'm') ADVANCE(67);
      END_STATE();
    case 107:
      if (lookahead == 'm') ADVANCE(55);
      END_STATE();
    case 108:
      if (lookahead == 'n') ADVANCE(104);
      END_STATE();
    case 109:
      if (lookahead == 'n') ADVANCE(72);
      END_STATE();
    case 110:
      if (lookahead == 'n') ADVANCE(147);
      END_STATE();
    case 111:
      if (lookahead == 'n') ADVANCE(28);
      END_STATE();
    case 112:
      if (lookahead == 'n') ADVANCE(152);
      END_STATE();
    case 113:
      if (lookahead == 'n') ADVANCE(62);
      END_STATE();
    case 114:
      if (lookahead == 'n') ADVANCE(12);
      END_STATE();
    case 115:
      if (lookahead == 'o') ADVANCE(146);
      END_STATE();
    case 116:
      if (lookahead == 'o') ADVANCE(101);
      END_STATE();
    case 117:
      if (lookahead == 'o') ADVANCE(94);
      END_STATE();
    case 118:
      if (lookahead == 'o') ADVANCE(110);
      END_STATE();
    case 119:
      if (lookahead == 'o') ADVANCE(137);
      END_STATE();
    case 120:
      if (lookahead == 'o') ADVANCE(165);
      END_STATE();
    case 121:
      if (lookahead == 'o') ADVANCE(38);
      END_STATE();
    case 122:
      if (lookahead == 'o') ADVANCE(39);
      END_STATE();
    case 123:
      if (lookahead == 'o') ADVANCE(40);
      END_STATE();
    case 124:
      if (lookahead == 'o') ADVANCE(41);
      END_STATE();
    case 125:
      if (lookahead == 'o') ADVANCE(43);
      END_STATE();
    case 126:
      if (lookahead == 'p') ADVANCE(176);
      END_STATE();
    case 127:
      if (lookahead == 'p') ADVANCE(9);
      END_STATE();
    case 128:
      if (lookahead == 'p') ADVANCE(48);
      END_STATE();
    case 129:
      if (lookahead == 'p') ADVANCE(26);
      END_STATE();
    case 130:
      if (lookahead == 'r') ADVANCE(6);
      if (lookahead == 't') ADVANCE(156);
      END_STATE();
    case 131:
      if (lookahead == 'r') ADVANCE(167);
      END_STATE();
    case 132:
      if (lookahead == 'r') ADVANCE(174);
      END_STATE();
    case 133:
      if (lookahead == 'r') ADVANCE(188);
      END_STATE();
    case 134:
      if (lookahead == 'r') ADVANCE(185);
      END_STATE();
    case 135:
      if (lookahead == 'r') ADVANCE(14);
      END_STATE();
    case 136:
      if (lookahead == 'r') ADVANCE(161);
      END_STATE();
    case 137:
      if (lookahead == 'r') ADVANCE(5);
      END_STATE();
    case 138:
      if (lookahead == 'r') ADVANCE(140);
      END_STATE();
    case 139:
      if (lookahead == 's') ADVANCE(15);
      END_STATE();
    case 140:
      if (lookahead == 's') ADVANCE(186);
      END_STATE();
    case 141:
      if (lookahead == 's') ADVANCE(116);
      END_STATE();
    case 142:
      if (lookahead == 's') ADVANCE(75);
      END_STATE();
    case 143:
      if (lookahead == 's') ADVANCE(145);
      END_STATE();
    case 144:
      if (lookahead == 's') ADVANCE(60);
      END_STATE();
    case 145:
      if (lookahead == 's') ADVANCE(13);
      END_STATE();
    case 146:
      if (lookahead == 't') ADVANCE(68);
      END_STATE();
    case 147:
      if (lookahead == 't') ADVANCE(169);
      END_STATE();
    case 148:
      if (lookahead == 't') ADVANCE(172);
      END_STATE();
    case 149:
      if (lookahead == 't') ADVANCE(175);
      END_STATE();
    case 150:
      if (lookahead == 't') ADVANCE(73);
      END_STATE();
    case 151:
      if (lookahead == 't') ADVANCE(59);
      END_STATE();
    case 152:
      if (lookahead == 't') ADVANCE(126);
      END_STATE();
    case 153:
      if (lookahead == 't') ADVANCE(113);
      END_STATE();
    case 154:
      if (lookahead == 't') ADVANCE(83);
      END_STATE();
    case 155:
      if (lookahead == 't') ADVANCE(82);
      END_STATE();
    case 156:
      if (lookahead == 't') ADVANCE(58);
      END_STATE();
    case 157:
      if (lookahead == 't') ADVANCE(65);
      END_STATE();
    case 158:
      if (lookahead == 't') ADVANCE(85);
      END_STATE();
    case 159:
      if (lookahead == 't') ADVANCE(84);
      END_STATE();
    case 160:
      if (lookahead == 'v') ADVANCE(80);
      END_STATE();
    case 161:
      if (lookahead == 'v') ADVANCE(66);
      END_STATE();
    case 162:
      if (lookahead == 'v') ADVANCE(54);
      END_STATE();
    case 163:
      if (lookahead == 'v') ADVANCE(56);
      END_STATE();
    case 164:
      if (lookahead == 'w') ADVANCE(144);
      END_STATE();
    case 165:
      if (lookahead == 'w') ADVANCE(11);
      END_STATE();
    case 166:
      if (lookahead == 'x') ADVANCE(183);
      if (lookahead == 'y') ADVANCE(184);
      END_STATE();
    case 167:
      if (lookahead == 'y') ADVANCE(10);
      END_STATE();
    case 168:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 169:
      ACCEPT_TOKEN(sym_font);
      if (lookahead == '-') ADVANCE(70);
      END_STATE();
    case 170:
      ACCEPT_TOKEN(sym_font_fallback);
      END_STATE();
    case 171:
      ACCEPT_TOKEN(sym_time_block);
      END_STATE();
    case 172:
      ACCEPT_TOKEN(sym_date_fmt);
      END_STATE();
    case 173:
      ACCEPT_TOKEN(sym_browser_path);
      END_STATE();
    case 174:
      ACCEPT_TOKEN(sym_browser);
      if (lookahead == '-') ADVANCE(129);
      END_STATE();
    case 175:
      ACCEPT_TOKEN(sym_time_fmt);
      END_STATE();
    case 176:
      ACCEPT_TOKEN(sym_update_time_ntp);
      END_STATE();
    case 177:
      ACCEPT_TOKEN(sym_brightness_block);
      END_STATE();
    case 178:
      ACCEPT_TOKEN(sym_battery_block);
      END_STATE();
    case 179:
      ACCEPT_TOKEN(sym_connman_block);
      END_STATE();
    case 180:
      ACCEPT_TOKEN(sym_media_block);
      END_STATE();
    case 181:
      ACCEPT_TOKEN(sym_color_active);
      END_STATE();
    case 182:
      ACCEPT_TOKEN(sym_color_inactive);
      END_STATE();
    case 183:
      ACCEPT_TOKEN(sym_padding_x);
      END_STATE();
    case 184:
      ACCEPT_TOKEN(sym_padding_y);
      END_STATE();
    case 185:
      ACCEPT_TOKEN(sym_top_bar);
      END_STATE();
    case 186:
      ACCEPT_TOKEN(sym_time_servers);
      END_STATE();
    case 187:
      ACCEPT_TOKEN(sym_bar_show_time);
      END_STATE();
    case 188:
      ACCEPT_TOKEN(sym_divider);
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
    [sym_font] = ACTIONS(1),
    [sym_font_fallback] = ACTIONS(1),
    [sym_time_block] = ACTIONS(1),
    [sym_date_fmt] = ACTIONS(1),
    [sym_browser_path] = ACTIONS(1),
    [sym_browser] = ACTIONS(1),
    [sym_time_fmt] = ACTIONS(1),
    [sym_update_time_ntp] = ACTIONS(1),
    [sym_brightness_block] = ACTIONS(1),
    [sym_battery_block] = ACTIONS(1),
    [sym_connman_block] = ACTIONS(1),
    [sym_media_block] = ACTIONS(1),
    [sym_color_active] = ACTIONS(1),
    [sym_color_inactive] = ACTIONS(1),
    [sym_padding_x] = ACTIONS(1),
    [sym_padding_y] = ACTIONS(1),
    [sym_top_bar] = ACTIONS(1),
    [sym_time_servers] = ACTIONS(1),
    [sym_bar_show_time] = ACTIONS(1),
    [sym_divider] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(3),
    [sym_font] = ACTIONS(3),
    [sym_font_fallback] = ACTIONS(5),
    [sym_time_block] = ACTIONS(5),
    [sym_date_fmt] = ACTIONS(5),
    [sym_browser_path] = ACTIONS(5),
    [sym_browser] = ACTIONS(3),
    [sym_time_fmt] = ACTIONS(5),
    [sym_update_time_ntp] = ACTIONS(5),
    [sym_brightness_block] = ACTIONS(5),
    [sym_battery_block] = ACTIONS(5),
    [sym_connman_block] = ACTIONS(5),
    [sym_media_block] = ACTIONS(5),
    [sym_color_active] = ACTIONS(5),
    [sym_color_inactive] = ACTIONS(5),
    [sym_padding_x] = ACTIONS(5),
    [sym_padding_y] = ACTIONS(5),
    [sym_top_bar] = ACTIONS(5),
    [sym_time_servers] = ACTIONS(5),
    [sym_bar_show_time] = ACTIONS(5),
    [sym_divider] = ACTIONS(5),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 1,
    ACTIONS(7), 1,
      ts_builtin_sym_end,
  [4] = 1,
    ACTIONS(9), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 4,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = false}}, SHIFT(2),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [7] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [9] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_dconfsomebar(void) {
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
