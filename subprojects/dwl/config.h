/* Autostart */
static const char *const autostart[] = {
				"/etc/s6-user/start-default", NULL,
        NULL /* terminate */
};

static const Rule rules[] = {
	/* app_id     title       tags mask     isfloating   monitor */
	/* examples:
	{ "Gimp",     NULL,       0,            1,           -1 },
	*/
	// { "firefox",  NULL,       1 << 8,       0,           -1 },
};

/* layout(s) */
static const Layout layouts[] = {
	/* symbol     arrange function */
	{ "[]=",      tile },
	{ "><>",      NULL },    /* no layout function means floating behavior */
	{ "[M]",      monocle },
};

/* monitors */
static const MonitorRule monrules[] = {
	/* name       mfact nmaster scale layout       rotate/reflect                x    y */
	/* example of a HiDPI laptop monitor:
	{ "eDP-1",    0.5,  1,      2,    &layouts[0], WL_OUTPUT_TRANSFORM_NORMAL    -1,  -1 },
	*/
	/* defaults */
	{ "LG Electronics LG TV", 0.55, 1, 2, &layouts[0], WL_OUTPUT_TRANSFORM_NORMAL, -1, -1 },
	{ NULL,       0.55, 1,      1,    &layouts[0], WL_OUTPUT_TRANSFORM_NORMAL,   -1,  -1 },
};

#define TAGKEYS(KEY,SKEY,TAG) \
	{ 1, 0,                    KEY,            view,            {.ui = 1 << TAG} }, \
	{ 1, WLR_MODIFIER_CTRL,  KEY,            toggleview,      {.ui = 1 << TAG} }, \
	{ 1, WLR_MODIFIER_SHIFT, SKEY,           tag,             {.ui = 1 << TAG} }, \
	{ 1, WLR_MODIFIER_CTRL|WLR_MODIFIER_SHIFT,SKEY,toggletag, {.ui = 1 << TAG} }

#define TAGKEYSALT(KEY,SKEY,TAG) \
	{ 1, WLR_MODIFIER_ALT,                    KEY,            view,            {.ui = 1 << TAG} }, \
	{ 1, WLR_MODIFIER_CTRL|WLR_MODIFIER_ALT,  KEY,            toggleview,      {.ui = 1 << TAG} }, \
	{ 1, WLR_MODIFIER_SHIFT|WLR_MODIFIER_ALT, SKEY,           tag,             {.ui = 1 << TAG} }, \
	{ 1, WLR_MODIFIER_CTRL|WLR_MODIFIER_SHIFT|WLR_MODIFIER_ALT,SKEY,toggletag, {.ui = 1 << TAG} }

/* helper for spawning shell commands in the pre dwm-5.0 fashion */
#define SHCMD(cmd) { .v = (const char*[]){ "/bin/sh", "-c", cmd, NULL } }

/* commands */
static const char *termcmd[]    = { "footclient", NULL };
static const char *thunarcmd[]  = { "thunar", NULL };
static const char *grimcmd[] = { "dash", "-c", "grim -g \"$(slurp -o)\"", NULL };
static const char *lockcmd[] = { "waylock", NULL };
static const char *connmanoffline[] = { "execlineb", "-c", "ifelse -X { pipeline { connmanctl state } grep -q \"OfflineMode = False\" } { connmanctl enable offline } connmanctl disable offline", NULL };
static const char *audioplay[] = { "playerctl", "-p", "playerctld", "play", NULL };
static const char *audiopause[] = { "playerctl", "-p", "playerctld", "pause", NULL };
static const char *audiostop[] = { "playerctl", "-p", "playerctld", "stop", NULL };
static const char *audionext[] = { "playerctl", "-p", "playerctld", "next", NULL };
static const char *audioprev[] = { "playerctl", "-p", "playerctld", "previous", NULL };

static const SwitchCmd switches[] = {
	// { WLR_SWITCH_STATE_ON, WLR_SWITCH_TYPE_LID, spawn, {.v = lockcmd} },
};

static const Key keys[] = {
	/* Note that Shift changes certain key codes: c -> C, 2 -> at, etc. */
	/* modifier                  key                 function        argument */
	{ 1, 0,                    XKB_KEY_space,      wob,          {.w = ZNET_TAPESOFTWARE_DWL_WM_V1_WOB_COMMAND_LAUNCH_APP} },
	{ 1, 0,                    XKB_KEY_p,      wob,          {.w = ZNET_TAPESOFTWARE_DWL_WM_V1_WOB_COMMAND_POWER_BUTTON} },
	{ 1, 0,                    XKB_KEY_Return,     spawn,          {.v = termcmd} },
	{ 1, WLR_MODIFIER_SHIFT, XKB_KEY_N,          spawn,          {.v = thunarcmd} },
	{ 1, 0,                    XKB_KEY_s,          spawn,          {.v = grimcmd} },
	{ 1, 0,                    XKB_KEY_j,          focusstack,     {.i = +1} },
	{ 1, 0,                    XKB_KEY_k,          focusstack,     {.i = -1} },
	{ 1, 0,                    XKB_KEY_i,          incnmaster,     {.i = +1} },
	{ 1, 0,                    XKB_KEY_d,          incnmaster,     {.i = -1} },
	{ 1, 0,                    XKB_KEY_h,          setmfact,       {.f = -0.05} },
	{ 1, 0,                   XKB_KEY_l,          setmfact,       {.f = +0.05} },
	{ 1, 0,                   XKB_KEY_w,          zoom,           {0} },
	{ 1, 0,                   XKB_KEY_Tab,        view,           {0} },
	{ 1, WLR_MODIFIER_SHIFT, XKB_KEY_Q,          killclient,     {0} },
	{ 1, 0,                   XKB_KEY_t,          setlayout,      {.v = &layouts[0]} },
	{ 1, 0,                   XKB_KEY_f,          setlayout,      {.v = &layouts[1]} },
	{ 1, 0,                   XKB_KEY_m,          setlayout,      {.v = &layouts[2]} },
	{ 1, WLR_MODIFIER_SHIFT, XKB_KEY_space,      togglefloating, {0} },
	{ 1, 0,                   XKB_KEY_e,          togglefullscreen, {0} },
	{ 1, 0,                   XKB_KEY_0,          view,           {.ui = ~0} },
	{ 1, WLR_MODIFIER_SHIFT, XKB_KEY_parenright, tag,            {.ui = ~0} },
	{ 1, 0,                   XKB_KEY_comma,      focusmon,       {.i = WLR_DIRECTION_LEFT} },
	{ 1, 0,                   XKB_KEY_period,     focusmon,       {.i = WLR_DIRECTION_RIGHT} },
	{ 1, WLR_MODIFIER_SHIFT, XKB_KEY_less,       tagmon,         {.i = WLR_DIRECTION_LEFT} },
	{ 1, WLR_MODIFIER_SHIFT, XKB_KEY_greater,    tagmon,         {.i = WLR_DIRECTION_RIGHT} },
	{ 1, WLR_MODIFIER_SHIFT, XKB_KEY_Return,     wob,            {.w = ZNET_TAPESOFTWARE_DWL_WM_V1_WOB_COMMAND_LAUNCH_BROWSER } },
	{ 0, 0,                         XKB_KEY_XF86AudioRaiseVolume,  wob, {.w = ZNET_TAPESOFTWARE_DWL_WM_V1_WOB_COMMAND_VOLUME_UP } },
	{ 0, 0,                         XKB_KEY_XF86AudioLowerVolume,  wob, {.w = ZNET_TAPESOFTWARE_DWL_WM_V1_WOB_COMMAND_VOLUME_DOWN } },
	{ 0, 0,                         XKB_KEY_XF86AudioPlay,         spawn, {.v = audioplay} },
	{ 0, 0,                         XKB_KEY_XF86AudioPause,        spawn, {.v = audiopause} },
	{ 0, 0,                         XKB_KEY_XF86AudioStop,         spawn, {.v = audiostop} },
	{ 0, 0,                         XKB_KEY_XF86AudioNext,         spawn, {.v = audionext} },
	{ 0, 0,                         XKB_KEY_XF86AudioPrev,         spawn, {.v = audioprev} },
	{ 0, 0,                         XKB_KEY_XF86MonBrightnessUp,   wob, {.w = ZNET_TAPESOFTWARE_DWL_WM_V1_WOB_COMMAND_LIGHT_UP } },
	{ 0, 0,                         XKB_KEY_XF86MonBrightnessDown, wob, {.w = ZNET_TAPESOFTWARE_DWL_WM_V1_WOB_COMMAND_LIGHT_DOWN } },
	{ 0, 0,                         0x1008ffb5,                    spawn, { .v = connmanoffline } },
	TAGKEYS(          XKB_KEY_1, XKB_KEY_exclam,                     0),
	TAGKEYS(          XKB_KEY_2, XKB_KEY_at,                         1),
	TAGKEYS(          XKB_KEY_3, XKB_KEY_numbersign,                 2),
	TAGKEYS(          XKB_KEY_4, XKB_KEY_dollar,                     3),
	TAGKEYS(          XKB_KEY_5, XKB_KEY_percent,                    4),
	TAGKEYS(          XKB_KEY_6, XKB_KEY_asciicircum,                5),
	TAGKEYS(          XKB_KEY_7, XKB_KEY_ampersand,                  6),
	TAGKEYS(          XKB_KEY_8, XKB_KEY_asterisk,                   7),
	TAGKEYS(          XKB_KEY_9, XKB_KEY_parenleft,                  8),
TAGKEYSALT(          XKB_KEY_1, XKB_KEY_exclam,                     9),
TAGKEYSALT(          XKB_KEY_2, XKB_KEY_at,                         10),
TAGKEYSALT(          XKB_KEY_3, XKB_KEY_numbersign,                 11),
TAGKEYSALT(          XKB_KEY_4, XKB_KEY_dollar,                     12),
TAGKEYSALT(          XKB_KEY_5, XKB_KEY_percent,                    13),
TAGKEYSALT(          XKB_KEY_6, XKB_KEY_asciicircum,                14),
TAGKEYSALT(          XKB_KEY_7, XKB_KEY_ampersand,                  15),
TAGKEYSALT(          XKB_KEY_8, XKB_KEY_asterisk,                   16),
TAGKEYSALT(          XKB_KEY_9, XKB_KEY_parenleft,                  17),
	{ 1, WLR_MODIFIER_SHIFT, XKB_KEY_C,          quit,           {0} },

	/* Ctrl-Alt-Backspace and Ctrl-Alt-Fx used to be handled by X server */
	{ 0, WLR_MODIFIER_CTRL|WLR_MODIFIER_ALT,XKB_KEY_Terminate_Server, quit, {0} },
#define CHVT(n) { 0, WLR_MODIFIER_CTRL|WLR_MODIFIER_ALT,XKB_KEY_XF86Switch_VT_##n, chvt, {.ui = (n)} }
	CHVT(1), CHVT(2), CHVT(3), CHVT(4), CHVT(5), CHVT(6),
	CHVT(7), CHVT(8), CHVT(9), CHVT(10), CHVT(11), CHVT(12),
};

static const Button buttons[] = {
	{ BTN_LEFT,   moveresize,     {.ui = CurMove} },
	{ BTN_MIDDLE, togglefloating, {0} },
	{ BTN_RIGHT,  moveresize,     {.ui = CurResize} },
};
