module.exports = grammar({
  name: 'dconfsomebar',

  extras: _ => [],

  rules: {
    // TODO: add the actual grammar rules
    source_file: $ => choice(
      $.font,
      $.font_fallback,
      $.time_block,
      $.date_fmt,
      $.browser_path,
      $.browser,
      $.time_fmt,
      $.update_time_ntp,
      $.brightness_block,
      $.battery_block,
      $.connman_block,
      $.media_block,
      $.color_active,
      $.color_inactive,
      $.padding_x,
      $.padding_y,
      $.top_bar,
      $.time_servers,
      $.bar_show_time,
      $.divider,
    ),
    font: _ => "/dotfiles/somebar/font", 
    font_fallback: _ => "/dotfiles/somebar/font-fallback", 
    time_block: _ => "/dotfiles/somebar/time-block", 
    date_fmt: _ => "/dotfiles/somebar/date-fmt", 
    browser_path: _ => "/dotfiles/somebar/browser-path", 
    browser: _ => "/dotfiles/somebar/browser", 
    time_fmt: _ => "/dotfiles/somebar/time-fmt", 
    update_time_ntp: _ => "/dotfiles/somebar/update-time-ntp", 
    brightness_block: _ => "/dotfiles/somebar/brightness-block", 
    battery_block: _ => "/dotfiles/somebar/battery-block", 
    connman_block: _ => "/dotfiles/somebar/connman-block", 
    media_block: _ => "/dotfiles/somebar/media-block", 
    color_active: _ => "/dotfiles/somebar/color-active", 
    color_inactive: _ => "/dotfiles/somebar/color-inactive", 
    padding_x: _ => "/dotfiles/somebar/padding-x", 
    padding_y: _ => "/dotfiles/somebar/padding-y", 
    top_bar: _ => "/dotfiles/somebar/top-bar", 
    time_servers: _ => "/dotfiles/somebar/time-servers", 
    bar_show_time: _ => "/dotfiles/somebar/bar-show-time", 
    divider: _ => "/dotfiles/somebar/divider",
  }
});

