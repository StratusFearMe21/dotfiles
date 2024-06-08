/* Taken from https://github.com/djpohly/dwl/issues/466 */
#define COLOR(hex)    { ((hex >> 24) & 0xFF) / 255.0f, \
                        ((hex >> 16) & 0xFF) / 255.0f, \
                        ((hex >> 8) & 0xFF) / 255.0f, \
                        (hex & 0xFF) / 255.0f }
/* appearance */
static int sloppyfocus               = 1;  /* focus follows mouse */
static int bypass_surface_visibility = 0;  /* 1 means idle inhibitors will disable idle tracking even if it's surface isn't visible  */
static unsigned int borderpx         = 1;  /* border pixel of windows */
static float rootcolor[]             = COLOR(0x222222ff);
static float bordercolor[]           = COLOR(0x565b14ff);
static float focuscolor[]            = COLOR(0xea6c73ff);
static float urgentcolor[]           = COLOR(0xd95757ff);
/* This conforms to the xdg-protocol. Set the alpha to zero to restore the old behavior */
static float fullscreen_bg[]         = {0.1f, 0.1f, 0.1f, 1.0f}; /* You can also use glsl colors */

/* tagging - TAGCOUNT must be no greater than 31 */
static int tagcount = 9;

/* logging */
static int log_level = WLR_ERROR;

static const Rule rules[] = {
	/* app_id             title       tags mask     isfloating   monitor */
	/* examples: */
	{ "Gimp_EXAMPLE",     NULL,       0,            1,           -1 }, /* Start on currently visible tags floating, not tiled */
	{ "firefox_EXAMPLE",  NULL,       1 << 8,       0,           -1 }, /* Start on ONLY tag "9" */
};

/* layout(s) */
static const Layout layouts[] = {
	/* symbol     arrange function */
	{ "[]=",      tile },
	{ "><>",      NULL },    /* no layout function means floating behavior */
	{ "[M]",      monocle },
};

/* monitors */
/* (x=-1, y=-1) is reserved as an "autoconfigure" monitor position indicator */
/* WARNING: negative values other than (-1, -1) cause problems with xwayland clients' menus */
/* NOTE: ALWAYS add a fallback rule, even if you are completely sure it won't be used */
static const MonitorRule monrules[] = {
	/* name       mfact  nmaster scale layout       rotate/reflect                x    y */
	/* example of a HiDPI laptop monitor:
	{ "eDP-1",    0.5f,  1,      2,    &layouts[0], WL_OUTPUT_TRANSFORM_NORMAL,   -1,  -1 },
	*/
	/* defaults */
  {"LG Electronics LG TV", 0.55f, 1, 2, &layouts[0],
     WL_OUTPUT_TRANSFORM_NORMAL, -1, -1},
	{ NULL,       0.55f, 1,      1,    &layouts[0], WL_OUTPUT_TRANSFORM_NORMAL,   -1,  -1 },
};

/* keyboard */
static struct xkb_rule_names xkb_rules = {
	/* can specify fields: rules, model, layout, variant, options */
	/* example:
	.options = "ctrl:nocaps",
	*/
	.options = "caps:swapescape,compose:ralt"
};

static int repeat_rate = 25;
static int repeat_delay = 600;

/* Trackpad */
static int tap_to_click = 1;
static int tap_and_drag = 1;
static int drag_lock = 1;
static int natural_scrolling = 0;
static int disable_while_typing = 1;
static int left_handed = 0;
static int middle_button_emulation = 1;
/* You can choose between:
LIBINPUT_CONFIG_SCROLL_NO_SCROLL
LIBINPUT_CONFIG_SCROLL_2FG
LIBINPUT_CONFIG_SCROLL_EDGE
LIBINPUT_CONFIG_SCROLL_ON_BUTTON_DOWN
*/
static enum libinput_config_scroll_method scroll_method = LIBINPUT_CONFIG_SCROLL_2FG;

/* You can choose between:
LIBINPUT_CONFIG_CLICK_METHOD_NONE
LIBINPUT_CONFIG_CLICK_METHOD_BUTTON_AREAS
LIBINPUT_CONFIG_CLICK_METHOD_CLICKFINGER
*/
static enum libinput_config_click_method click_method = LIBINPUT_CONFIG_CLICK_METHOD_BUTTON_AREAS;

/* You can choose between:
LIBINPUT_CONFIG_SEND_EVENTS_ENABLED
LIBINPUT_CONFIG_SEND_EVENTS_DISABLED
LIBINPUT_CONFIG_SEND_EVENTS_DISABLED_ON_EXTERNAL_MOUSE
*/
static uint32_t send_events_mode = LIBINPUT_CONFIG_SEND_EVENTS_ENABLED;

/* You can choose between:
LIBINPUT_CONFIG_ACCEL_PROFILE_FLAT
LIBINPUT_CONFIG_ACCEL_PROFILE_ADAPTIVE
*/
static enum libinput_config_accel_profile accel_profile = LIBINPUT_CONFIG_ACCEL_PROFILE_ADAPTIVE;
static double accel_speed = 0.0;

/* You can choose between:
LIBINPUT_CONFIG_TAP_MAP_LRM -- 1/2/3 finger tap maps to left/right/middle
LIBINPUT_CONFIG_TAP_MAP_LMR -- 1/2/3 finger tap maps to left/middle/right
*/
static enum libinput_config_tap_button_map button_map = LIBINPUT_CONFIG_TAP_MAP_LRM;

/* If you want to use the windows key for MODKEY, use WLR_MODIFIER_LOGO */

#define TAGKEYS(KEY,SKEY,TAG) \
	{ 1, 0,                    KEY,            view,            {.ui = 1 << TAG} }, \
	{ 1, WLR_MODIFIER_CTRL,  KEY,            toggleview,      {.ui = 1 << TAG} }, \
	{ 1, WLR_MODIFIER_SHIFT, SKEY,           tag,             {.ui = 1 << TAG} }, \
	{ 1, WLR_MODIFIER_CTRL|WLR_MODIFIER_SHIFT,SKEY,toggletag, {.ui = 1 << TAG} }

/* helper for spawning shell commands in the pre dwm-5.0 fashion */
#define SHCMD(cmd) { .v = (const char*[]){ "/bin/sh", "-c", cmd, NULL } }

/* commands */
static const char *termcmd[] = {"footclient", NULL};
static const char *thunarcmd[] = {"thunar", NULL};
static const char *grimcmd[] = {"sh", "-c", "grim -g \"$(slurp -o)\"", NULL};
static const char *lockcmd[] = {"waylock", NULL};
static const char *connmanoffline[] = {
    "execlineb", "-c",
    "ifelse -X { pipeline { connmanctl state } grep -q \"OfflineMode = False\" "
    "} { connmanctl enable offline } connmanctl disable offline",
    NULL};
static const char *audioplay[] = {"playerctl", "-p", "playerctld", "play",
                                  NULL};
static const char *audiopause[] = {"playerctl", "-p", "playerctld", "pause",
                                   NULL};
static const char *audiostop[] = {"playerctl", "-p", "playerctld", "stop",
                                  NULL};
static const char *audionext[] = {"playerctl", "-p", "playerctld", "next",
                                  NULL};
static const char *audioprev[] = {"playerctl", "-p", "playerctld", "previous",
                                  NULL};
static const char *writeraw[] = {"sh", "-c",
                                 "wl-paste | tr -d '\\n\\r\\v' | wtype -"};

static const Key keys[] = {
	/* Note that Shift changes certain key codes: c -> C, 2 -> at, etc. */
	/* modifier                  key                 function        argument */
	{ 1, 0,                    XKB_KEY_space,      wob,            {.w = ZNET_TAPESOFTWARE_DWL_WM_V1_WOB_COMMAND_LAUNCH_APP} },
	{ 1, 0,                    XKB_KEY_p,          wob,            {.w = ZNET_TAPESOFTWARE_DWL_WM_V1_WOB_COMMAND_POWER_BUTTON} },
	{ 1, 0,                    XKB_KEY_b,          wob,            {.w = ZNET_TAPESOFTWARE_DWL_WM_V1_WOB_COMMAND_OVERLAY} },
	{ 1, 0,                    XKB_KEY_Return,     spawn,          {.v = termcmd} },
	{ 1, WLR_MODIFIER_SHIFT,   XKB_KEY_N,          spawn,          {.v = thunarcmd} },
	{ 1, 0,                    XKB_KEY_s,          spawn,          {.v = grimcmd}},
	{ 1, 0,                    XKB_KEY_j,          focusstack,     {.i = +1} },
	{ 1, 0,                    XKB_KEY_k,          focusstack,     {.i = -1} },
	{ 1, 0,                    XKB_KEY_i,          incnmaster,     {.i = +1} },
	{ 1, 0,                    XKB_KEY_d,          incnmaster,     {.i = -1} },
	{ 1, 0,                    XKB_KEY_h,          setmfact,       {.f = -0.05} },
	{ 1, 0,                    XKB_KEY_l,          setmfact,       {.f = +0.05} },
	{ 1, 0,                    XKB_KEY_w,          zoom,           {0} },
	{ 1, 0,                    XKB_KEY_Tab,        view,           {0} },
	{ 1, WLR_MODIFIER_SHIFT,   XKB_KEY_Q,          killclient,     {0} },
	{ 1, 0,                    XKB_KEY_v,          spawn,          {.v = writeraw} },
	{ 1, 0,                    XKB_KEY_t,          setlayout,      {.v = &layouts[0]} },
	{ 1, 0,                    XKB_KEY_f,          setlayout,      {.v = &layouts[1]} },
	{ 1, 0,                    XKB_KEY_m,          setlayout,      {.v = &layouts[2]} },
	{ 1, WLR_MODIFIER_SHIFT,   XKB_KEY_space,      togglefloating, {0} },
	{ 1, 0,                    XKB_KEY_e,          togglefullscreen,{0} },
	{ 1, 0,                    XKB_KEY_0,          view,           {.ui = ~0} },
	{ 1, WLR_MODIFIER_SHIFT,   XKB_KEY_parenright, tag,            {.ui = ~0} },
	{ 1, 0,                    XKB_KEY_comma,      focusmon,       {.i = WLR_DIRECTION_LEFT} },
	{ 1, 0,                    XKB_KEY_period,     focusmon,       {.i = WLR_DIRECTION_RIGHT} },
	{ 1, WLR_MODIFIER_SHIFT,   XKB_KEY_less,       tagmon,         {.i = WLR_DIRECTION_LEFT} },
	{ 1, WLR_MODIFIER_SHIFT,   XKB_KEY_greater,    tagmon,         {.i = WLR_DIRECTION_RIGHT} },
  { 1, WLR_MODIFIER_SHIFT,   XKB_KEY_Return,     wob,            {.w = ZNET_TAPESOFTWARE_DWL_WM_V1_WOB_COMMAND_LAUNCH_BROWSER} },
  { 0, 0,                    XKB_KEY_XF86AudioRaiseVolume, wob,  {.w = ZNET_TAPESOFTWARE_DWL_WM_V1_WOB_COMMAND_VOLUME_UP} },
  { 0, 0,                    XKB_KEY_XF86AudioLowerVolume, wob,  {.w = ZNET_TAPESOFTWARE_DWL_WM_V1_WOB_COMMAND_VOLUME_DOWN} },
  { 0, 0,                    XKB_KEY_XF86AudioPlay,        spawn,{.v = audioplay} },
  { 0, 0,                    XKB_KEY_XF86AudioPause,       spawn,{.v = audiopause} },
  { 0, 0,                    XKB_KEY_XF86AudioStop,        spawn,{.v = audiostop} },
  { 0, 0,                    XKB_KEY_XF86AudioNext,        spawn,{.v = audionext} },
  { 0, 0,                    XKB_KEY_XF86AudioPrev,        spawn,{.v = audioprev} },
  { 0, 0,                    XKB_KEY_XF86MonBrightnessUp,  wob,  {.w = ZNET_TAPESOFTWARE_DWL_WM_V1_WOB_COMMAND_LIGHT_UP} },
  { 0, 0,                    XKB_KEY_XF86MonBrightnessDown,wob,  {.w = ZNET_TAPESOFTWARE_DWL_WM_V1_WOB_COMMAND_LIGHT_DOWN} },
  { 0, 0,                    0x1008ffb5,                   spawn,{.v = connmanoffline} },
	TAGKEYS(          XKB_KEY_1, XKB_KEY_exclam,                     0),
	TAGKEYS(          XKB_KEY_2, XKB_KEY_at,                         1),
	TAGKEYS(          XKB_KEY_3, XKB_KEY_numbersign,                 2),
	TAGKEYS(          XKB_KEY_4, XKB_KEY_dollar,                     3),
	TAGKEYS(          XKB_KEY_5, XKB_KEY_percent,                    4),
	TAGKEYS(          XKB_KEY_6, XKB_KEY_asciicircum,                5),
	TAGKEYS(          XKB_KEY_7, XKB_KEY_ampersand,                  6),
	TAGKEYS(          XKB_KEY_8, XKB_KEY_asterisk,                   7),
	TAGKEYS(          XKB_KEY_9, XKB_KEY_parenleft,                  8),
	{ 1, WLR_MODIFIER_SHIFT, XKB_KEY_C,          quit,           {0} },

	/* Ctrl-Alt-Backspace and Ctrl-Alt-Fx used to be handled by X server */
	{ 0, WLR_MODIFIER_CTRL|WLR_MODIFIER_ALT,XKB_KEY_Terminate_Server, quit, {0} },
	/* Ctrl-Alt-Fx is used to switch to another VT, if you don't know what a VT is
	 * do not remove them.
	 */
#define CHVT(n) { 0, WLR_MODIFIER_CTRL|WLR_MODIFIER_ALT,XKB_KEY_XF86Switch_VT_##n, chvt, {.ui = (n)} }
	CHVT(1), CHVT(2), CHVT(3), CHVT(4), CHVT(5), CHVT(6),
	CHVT(7), CHVT(8), CHVT(9), CHVT(10), CHVT(11), CHVT(12),
};

static const Button buttons[] = {
	{ BTN_LEFT,   moveresize,     {.ui = CurMove} },
	{ BTN_MIDDLE, togglefloating, {0} },
	{ BTN_RIGHT,  moveresize,     {.ui = CurResize} },
};
