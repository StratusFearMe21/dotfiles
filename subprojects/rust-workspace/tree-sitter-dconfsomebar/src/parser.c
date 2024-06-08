#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 4
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 24
#define ALIAS_COUNT 0
#define TOKEN_COUNT 23
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 1
#define PRODUCTION_ID_COUNT 1

enum ts_symbol_identifiers {
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
  sym_wireplumber_block = 13,
  sym_wireplumber_max_volume = 14,
  sym_color_active = 15,
  sym_color_inactive = 16,
  sym_padding_x = 17,
  sym_padding_y = 18,
  sym_top_bar = 19,
  sym_time_servers = 20,
  sym_bar_show_time = 21,
  sym_divider = 22,
  sym_source_file = 23,
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
  [sym_wireplumber_block] = "wireplumber_block",
  [sym_wireplumber_max_volume] = "wireplumber_max_volume",
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
  [sym_wireplumber_block] = sym_wireplumber_block,
  [sym_wireplumber_max_volume] = sym_wireplumber_max_volume,
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
  [sym_wireplumber_block] = {
    .visible = true,
    .named = true,
  },
  [sym_wireplumber_max_volume] = {
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
      if (eof) ADVANCE(193);
      if (lookahead == '/') ADVANCE(52);
      END_STATE();
    case 1:
      if (lookahead == '-') ADVANCE(34);
      END_STATE();
    case 2:
      if (lookahead == '-') ADVANCE(190);
      END_STATE();
    case 3:
      if (lookahead == '-') ADVANCE(42);
      END_STATE();
    case 4:
      if (lookahead == '-') ADVANCE(78);
      END_STATE();
    case 5:
      if (lookahead == '-') ADVANCE(38);
      END_STATE();
    case 6:
      if (lookahead == '-') ADVANCE(20);
      END_STATE();
    case 7:
      if (lookahead == '-') ADVANCE(163);
      END_STATE();
    case 8:
      if (lookahead == '-') ADVANCE(185);
      END_STATE();
    case 9:
      if (lookahead == '-') ADVANCE(175);
      END_STATE();
    case 10:
      if (lookahead == '-') ADVANCE(128);
      END_STATE();
    case 11:
      if (lookahead == '-') ADVANCE(37);
      END_STATE();
    case 12:
      if (lookahead == '-') ADVANCE(39);
      END_STATE();
    case 13:
      if (lookahead == '-') ADVANCE(179);
      END_STATE();
    case 14:
      if (lookahead == '-') ADVANCE(40);
      END_STATE();
    case 15:
      if (lookahead == '-') ADVANCE(41);
      END_STATE();
    case 16:
      if (lookahead == '/') ADVANCE(33);
      END_STATE();
    case 17:
      if (lookahead == '/') ADVANCE(162);
      END_STATE();
    case 18:
      if (lookahead == 'a') ADVANCE(154);
      END_STATE();
    case 19:
      if (lookahead == 'a') ADVANCE(149);
      if (lookahead == 'r') ADVANCE(86);
      END_STATE();
    case 20:
      if (lookahead == 'a') ADVANCE(50);
      if (lookahead == 'i') ADVANCE(127);
      END_STATE();
    case 21:
      if (lookahead == 'a') ADVANCE(191);
      END_STATE();
    case 22:
      if (lookahead == 'a') ADVANCE(172);
      if (lookahead == 'i') ADVANCE(183);
      END_STATE();
    case 23:
      if (lookahead == 'a') ADVANCE(57);
      END_STATE();
    case 24:
      if (lookahead == 'a') ADVANCE(107);
      END_STATE();
    case 25:
      if (lookahead == 'a') ADVANCE(130);
      END_STATE();
    case 26:
      if (lookahead == 'a') ADVANCE(5);
      END_STATE();
    case 27:
      if (lookahead == 'a') ADVANCE(47);
      END_STATE();
    case 28:
      if (lookahead == 'a') ADVANCE(153);
      END_STATE();
    case 29:
      if (lookahead == 'a') ADVANCE(171);
      END_STATE();
    case 30:
      if (lookahead == 'a') ADVANCE(178);
      END_STATE();
    case 31:
      if (lookahead == 'a') ADVANCE(51);
      END_STATE();
    case 32:
      if (lookahead == 'b') ADVANCE(18);
      END_STATE();
    case 33:
      if (lookahead == 'b') ADVANCE(19);
      if (lookahead == 'c') ADVANCE(133);
      if (lookahead == 'd') ADVANCE(22);
      if (lookahead == 'f') ADVANCE(134);
      if (lookahead == 'm') ADVANCE(64);
      if (lookahead == 'p') ADVANCE(23);
      if (lookahead == 't') ADVANCE(91);
      if (lookahead == 'u') ADVANCE(147);
      if (lookahead == 'w') ADVANCE(87);
      END_STATE();
    case 34:
      if (lookahead == 'b') ADVANCE(108);
      if (lookahead == 'f') ADVANCE(120);
      if (lookahead == 's') ADVANCE(73);
      END_STATE();
    case 35:
      if (lookahead == 'b') ADVANCE(27);
      END_STATE();
    case 36:
      if (lookahead == 'b') ADVANCE(76);
      END_STATE();
    case 37:
      if (lookahead == 'b') ADVANCE(28);
      END_STATE();
    case 38:
      if (lookahead == 'b') ADVANCE(109);
      END_STATE();
    case 39:
      if (lookahead == 'b') ADVANCE(110);
      END_STATE();
    case 40:
      if (lookahead == 'b') ADVANCE(111);
      END_STATE();
    case 41:
      if (lookahead == 'b') ADVANCE(112);
      END_STATE();
    case 42:
      if (lookahead == 'b') ADVANCE(113);
      if (lookahead == 'm') ADVANCE(21);
      END_STATE();
    case 43:
      if (lookahead == 'c') ADVANCE(96);
      END_STATE();
    case 44:
      if (lookahead == 'c') ADVANCE(97);
      END_STATE();
    case 45:
      if (lookahead == 'c') ADVANCE(98);
      END_STATE();
    case 46:
      if (lookahead == 'c') ADVANCE(99);
      END_STATE();
    case 47:
      if (lookahead == 'c') ADVANCE(100);
      END_STATE();
    case 48:
      if (lookahead == 'c') ADVANCE(101);
      END_STATE();
    case 49:
      if (lookahead == 'c') ADVANCE(102);
      END_STATE();
    case 50:
      if (lookahead == 'c') ADVANCE(176);
      END_STATE();
    case 51:
      if (lookahead == 'c') ADVANCE(180);
      END_STATE();
    case 52:
      if (lookahead == 'd') ADVANCE(131);
      END_STATE();
    case 53:
      if (lookahead == 'd') ADVANCE(30);
      END_STATE();
    case 54:
      if (lookahead == 'd') ADVANCE(89);
      END_STATE();
    case 55:
      if (lookahead == 'd') ADVANCE(88);
      END_STATE();
    case 56:
      if (lookahead == 'd') ADVANCE(71);
      END_STATE();
    case 57:
      if (lookahead == 'd') ADVANCE(55);
      END_STATE();
    case 58:
      if (lookahead == 'e') ADVANCE(160);
      END_STATE();
    case 59:
      if (lookahead == 'e') ADVANCE(32);
      END_STATE();
    case 60:
      if (lookahead == 'e') ADVANCE(208);
      END_STATE();
    case 61:
      if (lookahead == 'e') ADVANCE(214);
      END_STATE();
    case 62:
      if (lookahead == 'e') ADVANCE(209);
      END_STATE();
    case 63:
      if (lookahead == 'e') ADVANCE(207);
      END_STATE();
    case 64:
      if (lookahead == 'e') ADVANCE(54);
      END_STATE();
    case 65:
      if (lookahead == 'e') ADVANCE(145);
      END_STATE();
    case 66:
      if (lookahead == 'e') ADVANCE(4);
      END_STATE();
    case 67:
      if (lookahead == 'e') ADVANCE(150);
      END_STATE();
    case 68:
      if (lookahead == 'e') ADVANCE(1);
      END_STATE();
    case 69:
      if (lookahead == 'e') ADVANCE(164);
      END_STATE();
    case 70:
      if (lookahead == 'e') ADVANCE(151);
      END_STATE();
    case 71:
      if (lookahead == 'e') ADVANCE(152);
      END_STATE();
    case 72:
      if (lookahead == 'e') ADVANCE(9);
      END_STATE();
    case 73:
      if (lookahead == 'e') ADVANCE(155);
      END_STATE();
    case 74:
      if (lookahead == 'e') ADVANCE(158);
      END_STATE();
    case 75:
      if (lookahead == 'e') ADVANCE(10);
      END_STATE();
    case 76:
      if (lookahead == 'e') ADVANCE(159);
      END_STATE();
    case 77:
      if (lookahead == 'f') ADVANCE(85);
      END_STATE();
    case 78:
      if (lookahead == 'f') ADVANCE(117);
      END_STATE();
    case 79:
      if (lookahead == 'f') ADVANCE(24);
      END_STATE();
    case 80:
      if (lookahead == 'g') ADVANCE(83);
      END_STATE();
    case 81:
      if (lookahead == 'g') ADVANCE(2);
      END_STATE();
    case 82:
      if (lookahead == 'h') ADVANCE(198);
      END_STATE();
    case 83:
      if (lookahead == 'h') ADVANCE(173);
      END_STATE();
    case 84:
      if (lookahead == 'h') ADVANCE(135);
      END_STATE();
    case 85:
      if (lookahead == 'i') ADVANCE(103);
      END_STATE();
    case 86:
      if (lookahead == 'i') ADVANCE(80);
      if (lookahead == 'o') ADVANCE(188);
      END_STATE();
    case 87:
      if (lookahead == 'i') ADVANCE(157);
      END_STATE();
    case 88:
      if (lookahead == 'i') ADVANCE(125);
      END_STATE();
    case 89:
      if (lookahead == 'i') ADVANCE(26);
      END_STATE();
    case 90:
      if (lookahead == 'i') ADVANCE(56);
      END_STATE();
    case 91:
      if (lookahead == 'i') ADVANCE(118);
      if (lookahead == 'o') ADVANCE(146);
      END_STATE();
    case 92:
      if (lookahead == 'i') ADVANCE(186);
      END_STATE();
    case 93:
      if (lookahead == 'i') ADVANCE(121);
      END_STATE();
    case 94:
      if (lookahead == 'i') ADVANCE(187);
      END_STATE();
    case 95:
      if (lookahead == 'i') ADVANCE(122);
      END_STATE();
    case 96:
      if (lookahead == 'k') ADVANCE(196);
      END_STATE();
    case 97:
      if (lookahead == 'k') ADVANCE(205);
      END_STATE();
    case 98:
      if (lookahead == 'k') ADVANCE(203);
      END_STATE();
    case 99:
      if (lookahead == 'k') ADVANCE(204);
      END_STATE();
    case 100:
      if (lookahead == 'k') ADVANCE(195);
      END_STATE();
    case 101:
      if (lookahead == 'k') ADVANCE(202);
      END_STATE();
    case 102:
      if (lookahead == 'k') ADVANCE(206);
      END_STATE();
    case 103:
      if (lookahead == 'l') ADVANCE(58);
      END_STATE();
    case 104:
      if (lookahead == 'l') ADVANCE(181);
      END_STATE();
    case 105:
      if (lookahead == 'l') ADVANCE(182);
      END_STATE();
    case 106:
      if (lookahead == 'l') ADVANCE(137);
      if (lookahead == 'n') ADVANCE(124);
      END_STATE();
    case 107:
      if (lookahead == 'l') ADVANCE(114);
      END_STATE();
    case 108:
      if (lookahead == 'l') ADVANCE(136);
      END_STATE();
    case 109:
      if (lookahead == 'l') ADVANCE(138);
      END_STATE();
    case 110:
      if (lookahead == 'l') ADVANCE(139);
      END_STATE();
    case 111:
      if (lookahead == 'l') ADVANCE(140);
      END_STATE();
    case 112:
      if (lookahead == 'l') ADVANCE(142);
      END_STATE();
    case 113:
      if (lookahead == 'l') ADVANCE(143);
      END_STATE();
    case 114:
      if (lookahead == 'l') ADVANCE(35);
      END_STATE();
    case 115:
      if (lookahead == 'm') ADVANCE(59);
      END_STATE();
    case 116:
      if (lookahead == 'm') ADVANCE(36);
      END_STATE();
    case 117:
      if (lookahead == 'm') ADVANCE(169);
      END_STATE();
    case 118:
      if (lookahead == 'm') ADVANCE(68);
      END_STATE();
    case 119:
      if (lookahead == 'm') ADVANCE(25);
      END_STATE();
    case 120:
      if (lookahead == 'm') ADVANCE(170);
      END_STATE();
    case 121:
      if (lookahead == 'm') ADVANCE(75);
      END_STATE();
    case 122:
      if (lookahead == 'm') ADVANCE(61);
      END_STATE();
    case 123:
      if (lookahead == 'm') ADVANCE(63);
      END_STATE();
    case 124:
      if (lookahead == 'n') ADVANCE(119);
      END_STATE();
    case 125:
      if (lookahead == 'n') ADVANCE(81);
      END_STATE();
    case 126:
      if (lookahead == 'n') ADVANCE(168);
      END_STATE();
    case 127:
      if (lookahead == 'n') ADVANCE(31);
      END_STATE();
    case 128:
      if (lookahead == 'n') ADVANCE(174);
      END_STATE();
    case 129:
      if (lookahead == 'n') ADVANCE(69);
      END_STATE();
    case 130:
      if (lookahead == 'n') ADVANCE(14);
      END_STATE();
    case 131:
      if (lookahead == 'o') ADVANCE(167);
      END_STATE();
    case 132:
      if (lookahead == 'o') ADVANCE(115);
      END_STATE();
    case 133:
      if (lookahead == 'o') ADVANCE(106);
      END_STATE();
    case 134:
      if (lookahead == 'o') ADVANCE(126);
      END_STATE();
    case 135:
      if (lookahead == 'o') ADVANCE(189);
      END_STATE();
    case 136:
      if (lookahead == 'o') ADVANCE(43);
      END_STATE();
    case 137:
      if (lookahead == 'o') ADVANCE(156);
      END_STATE();
    case 138:
      if (lookahead == 'o') ADVANCE(44);
      END_STATE();
    case 139:
      if (lookahead == 'o') ADVANCE(45);
      END_STATE();
    case 140:
      if (lookahead == 'o') ADVANCE(46);
      END_STATE();
    case 141:
      if (lookahead == 'o') ADVANCE(105);
      END_STATE();
    case 142:
      if (lookahead == 'o') ADVANCE(48);
      END_STATE();
    case 143:
      if (lookahead == 'o') ADVANCE(49);
      END_STATE();
    case 144:
      if (lookahead == 'p') ADVANCE(201);
      END_STATE();
    case 145:
      if (lookahead == 'p') ADVANCE(104);
      END_STATE();
    case 146:
      if (lookahead == 'p') ADVANCE(11);
      END_STATE();
    case 147:
      if (lookahead == 'p') ADVANCE(53);
      END_STATE();
    case 148:
      if (lookahead == 'p') ADVANCE(29);
      END_STATE();
    case 149:
      if (lookahead == 'r') ADVANCE(7);
      if (lookahead == 't') ADVANCE(177);
      END_STATE();
    case 150:
      if (lookahead == 'r') ADVANCE(192);
      END_STATE();
    case 151:
      if (lookahead == 'r') ADVANCE(199);
      END_STATE();
    case 152:
      if (lookahead == 'r') ADVANCE(215);
      END_STATE();
    case 153:
      if (lookahead == 'r') ADVANCE(212);
      END_STATE();
    case 154:
      if (lookahead == 'r') ADVANCE(16);
      END_STATE();
    case 155:
      if (lookahead == 'r') ADVANCE(184);
      END_STATE();
    case 156:
      if (lookahead == 'r') ADVANCE(6);
      END_STATE();
    case 157:
      if (lookahead == 'r') ADVANCE(65);
      END_STATE();
    case 158:
      if (lookahead == 'r') ADVANCE(161);
      END_STATE();
    case 159:
      if (lookahead == 'r') ADVANCE(3);
      END_STATE();
    case 160:
      if (lookahead == 's') ADVANCE(17);
      END_STATE();
    case 161:
      if (lookahead == 's') ADVANCE(213);
      END_STATE();
    case 162:
      if (lookahead == 's') ADVANCE(132);
      END_STATE();
    case 163:
      if (lookahead == 's') ADVANCE(84);
      END_STATE();
    case 164:
      if (lookahead == 's') ADVANCE(166);
      END_STATE();
    case 165:
      if (lookahead == 's') ADVANCE(70);
      END_STATE();
    case 166:
      if (lookahead == 's') ADVANCE(15);
      END_STATE();
    case 167:
      if (lookahead == 't') ADVANCE(77);
      END_STATE();
    case 168:
      if (lookahead == 't') ADVANCE(194);
      END_STATE();
    case 169:
      if (lookahead == 't') ADVANCE(197);
      END_STATE();
    case 170:
      if (lookahead == 't') ADVANCE(200);
      END_STATE();
    case 171:
      if (lookahead == 't') ADVANCE(82);
      END_STATE();
    case 172:
      if (lookahead == 't') ADVANCE(66);
      END_STATE();
    case 173:
      if (lookahead == 't') ADVANCE(129);
      END_STATE();
    case 174:
      if (lookahead == 't') ADVANCE(144);
      END_STATE();
    case 175:
      if (lookahead == 't') ADVANCE(93);
      END_STATE();
    case 176:
      if (lookahead == 't') ADVANCE(92);
      END_STATE();
    case 177:
      if (lookahead == 't') ADVANCE(67);
      END_STATE();
    case 178:
      if (lookahead == 't') ADVANCE(72);
      END_STATE();
    case 179:
      if (lookahead == 't') ADVANCE(95);
      END_STATE();
    case 180:
      if (lookahead == 't') ADVANCE(94);
      END_STATE();
    case 181:
      if (lookahead == 'u') ADVANCE(116);
      END_STATE();
    case 182:
      if (lookahead == 'u') ADVANCE(123);
      END_STATE();
    case 183:
      if (lookahead == 'v') ADVANCE(90);
      END_STATE();
    case 184:
      if (lookahead == 'v') ADVANCE(74);
      END_STATE();
    case 185:
      if (lookahead == 'v') ADVANCE(141);
      END_STATE();
    case 186:
      if (lookahead == 'v') ADVANCE(60);
      END_STATE();
    case 187:
      if (lookahead == 'v') ADVANCE(62);
      END_STATE();
    case 188:
      if (lookahead == 'w') ADVANCE(165);
      END_STATE();
    case 189:
      if (lookahead == 'w') ADVANCE(13);
      END_STATE();
    case 190:
      if (lookahead == 'x') ADVANCE(210);
      if (lookahead == 'y') ADVANCE(211);
      END_STATE();
    case 191:
      if (lookahead == 'x') ADVANCE(8);
      END_STATE();
    case 192:
      if (lookahead == 'y') ADVANCE(12);
      END_STATE();
    case 193:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 194:
      ACCEPT_TOKEN(sym_font);
      if (lookahead == '-') ADVANCE(79);
      END_STATE();
    case 195:
      ACCEPT_TOKEN(sym_font_fallback);
      END_STATE();
    case 196:
      ACCEPT_TOKEN(sym_time_block);
      END_STATE();
    case 197:
      ACCEPT_TOKEN(sym_date_fmt);
      END_STATE();
    case 198:
      ACCEPT_TOKEN(sym_browser_path);
      END_STATE();
    case 199:
      ACCEPT_TOKEN(sym_browser);
      if (lookahead == '-') ADVANCE(148);
      END_STATE();
    case 200:
      ACCEPT_TOKEN(sym_time_fmt);
      END_STATE();
    case 201:
      ACCEPT_TOKEN(sym_update_time_ntp);
      END_STATE();
    case 202:
      ACCEPT_TOKEN(sym_brightness_block);
      END_STATE();
    case 203:
      ACCEPT_TOKEN(sym_battery_block);
      END_STATE();
    case 204:
      ACCEPT_TOKEN(sym_connman_block);
      END_STATE();
    case 205:
      ACCEPT_TOKEN(sym_media_block);
      END_STATE();
    case 206:
      ACCEPT_TOKEN(sym_wireplumber_block);
      END_STATE();
    case 207:
      ACCEPT_TOKEN(sym_wireplumber_max_volume);
      END_STATE();
    case 208:
      ACCEPT_TOKEN(sym_color_active);
      END_STATE();
    case 209:
      ACCEPT_TOKEN(sym_color_inactive);
      END_STATE();
    case 210:
      ACCEPT_TOKEN(sym_padding_x);
      END_STATE();
    case 211:
      ACCEPT_TOKEN(sym_padding_y);
      END_STATE();
    case 212:
      ACCEPT_TOKEN(sym_top_bar);
      END_STATE();
    case 213:
      ACCEPT_TOKEN(sym_time_servers);
      END_STATE();
    case 214:
      ACCEPT_TOKEN(sym_bar_show_time);
      END_STATE();
    case 215:
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
    [sym_wireplumber_block] = ACTIONS(1),
    [sym_wireplumber_max_volume] = ACTIONS(1),
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
    [sym_wireplumber_block] = ACTIONS(5),
    [sym_wireplumber_max_volume] = ACTIONS(5),
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
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_dconfsomebar() {
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
