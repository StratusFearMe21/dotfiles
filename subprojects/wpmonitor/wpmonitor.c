/* WirePlumber
 *
 * Copyright © 2019-2020 Collabora Ltd.
 *    @author George Kiagiadakis <george.kiagiadakis@collabora.com>
 *
 * SPDX-License-Identifier: MIT
 */

#include <locale.h>
#include <pipewire/extensions/session-manager/keys.h>
#include <pipewire/keys.h>
#include <spa/utils/defs.h>
#include <stdio.h>
#include <wp/wp.h>
#include <wpmonitor-generated.h>

typedef struct _WpCtl WpCtl;
struct _WpCtl {
  WpPlugin parent;

  WpPlugin *def_nodes_api;
  WpPlugin *mixer_api;
  WpObjectManager *om;
  guint pending_plugins;
};

G_DECLARE_FINAL_TYPE(WpCtl, wp_ctl, WP, CTL, WpPlugin)
G_DEFINE_TYPE(WpCtl, wp_ctl, WP_TYPE_PLUGIN);

G_DEFINE_QUARK(wpctl - error, wpctl_error_domain)

static void wp_ctl_init(WpCtl *self) {}

static void wp_ctl_clear(WpPlugin *plugin) {
  WpCtl *self = WP_CTL(plugin);
  g_clear_object(&self->om);
  g_clear_object(&self->def_nodes_api);
  g_clear_object(&self->mixer_api);
}

static gboolean monitor_prepare(WpCtl *self, GError **error) {
  wp_object_manager_add_interest(self->om, WP_TYPE_NODE,
                                 WP_CONSTRAINT_TYPE_PW_PROPERTY, "media.class",
                                 "=s", "Audio/Sink", NULL);
  wp_object_manager_request_object_features(
      self->om, WP_TYPE_GLOBAL_PROXY, WP_PIPEWIRE_OBJECT_FEATURES_MINIMAL);
  return TRUE;
}

typedef struct {
  WpCtl *self;
  guint32 default_node;
  WpMonitorOrgWireplumberDefaultNode *dbus;
  gboolean waiting;
} print_context;

static void print_controls(guint32 id, print_context *context,
                           gboolean is_default) {
  g_autoptr(GVariant) dict = NULL;

  if (context->self->mixer_api)
    g_signal_emit_by_name(context->self->mixer_api, "get-volume", id, &dict);

  if (dict) {
    gboolean mute = FALSE;
    gdouble volume = 1.0;
    if (g_variant_lookup(dict, "mute", "b", &mute) &&
        g_variant_lookup(dict, "volume", "d", &volume)) {
      if (is_default) {
        wp_monitor_org_wireplumber_default_node_set_volume(context->dbus, dict);
        if (context->waiting) {
          static const char nl[] = {'\n'};
          write(3, &nl, 1);
          context->waiting = FALSE;
        }
      }
    }
  }
}

static void print_dev_node(const GValue *item, gpointer data) {
  WpPipewireObject *obj = g_value_get_object(item);
  print_context *context = data;
  guint32 id = wp_proxy_get_bound_id(WP_PROXY(obj));
  gboolean is_default = (context->default_node == id);
  const gchar *name = NULL;

  // if (cmdline.status.display_nicknames)
  //   name = wp_pipewire_object_get_property (obj, PW_KEY_NODE_NICK);
  // else if (cmdline.status.display_names)
  //   name = wp_pipewire_object_get_property (obj, PW_KEY_NODE_NAME);

  if (!name)
    name = wp_pipewire_object_get_property(obj, PW_KEY_NODE_DESCRIPTION);

  print_controls(id, context, is_default);
}

static void monitor_print(print_context *context) {

  /* sessions */
  g_autoptr(WpIterator) child_it = NULL;

  child_it = wp_object_manager_new_filtered_iterator(
      context->self->om, WP_TYPE_NODE, WP_CONSTRAINT_TYPE_PW_PROPERTY,
      PW_KEY_MEDIA_CLASS, "#s", "Audio/Sink", NULL);
  wp_iterator_foreach(child_it, print_dev_node, (gpointer)context);
  g_clear_pointer(&child_it, wp_iterator_unref);
}

static void onMixerChanged(print_context *self, uint32_t id) {
  if (self->default_node == id)
    monitor_print(self);
}

static void onDefaultNodesApiChanged(print_context *context) {
  context->default_node = -1;
  if (context->self->def_nodes_api)
    g_signal_emit_by_name(context->self->def_nodes_api, "get-default-node",
                          "Audio/Sink", &context->default_node);
  monitor_print(context);
}

static gboolean on_set_volume(WpMonitorOrgWireplumberDefaultNode *interface,
                              GDBusMethodInvocation *invocation,
                              const gdouble in, gpointer data) {
  print_context *context = data;
  GVariant *variant = g_variant_new("d", in);
  gboolean res = FALSE;

  g_signal_emit_by_name(context->self->mixer_api, "set-volume",
                        context->default_node, variant, &res);

  wp_monitor_org_wireplumber_default_node_complete_set_volume(context->dbus,
                                                              invocation, res);

  return TRUE;
}

static void on_name_acquired(GDBusConnection *connection, const gchar *name,
                             gpointer user_data) {
  g_autoptr(GError) error = NULL;
  print_context *context = user_data;
  if (context->self->def_nodes_api)
    g_signal_emit_by_name(context->self->def_nodes_api, "get-default-node",
                          "Audio/Sink", &context->default_node);

  monitor_print(context);

  g_signal_connect_swapped(context->self->mixer_api, "changed",
                           (GCallback)onMixerChanged, context);
  g_signal_connect_swapped(context->self->def_nodes_api, "changed",
                           (GCallback)onDefaultNodesApiChanged, context);
  g_signal_connect(context->dbus, "handle-set-volume", (GCallback)on_set_volume,
                   context);

  if (!g_dbus_interface_skeleton_export(
          G_DBUS_INTERFACE_SKELETON(context->dbus), connection, "/", &error)) {
    printf("Couldn't export skeleton");
  }
}

static void monitor_run(WpObjectManager *om, WpCtl *self) {
  wp_object_update_features(WP_OBJECT(self), WP_PLUGIN_FEATURE_ENABLED, 0);
  print_context *context = g_new(print_context, 1);

  context->self = self;
  context->dbus = wp_monitor_org_wireplumber_default_node_skeleton_new();
  context->default_node = -1;
  context->waiting = TRUE;

  g_bus_own_name(G_BUS_TYPE_SESSION, "org.wireplumber.DefaultNode",
                 G_BUS_NAME_OWNER_FLAGS_NONE, NULL, on_name_acquired, NULL,
                 (gpointer)context, NULL);
}

static WpCore *core;

static void on_plugin_activated(WpObject *p, GAsyncResult *res, WpCtl *ctl) {
  g_autoptr(GError) error = NULL;

  if (!wp_object_activate_finish(p, res, &error)) {
    fprintf(stderr, "%s", error->message);
    return;
  }
}

static void on_plugin_loaded(WpCore *core, GAsyncResult *res, WpCtl *self) {
  GError *error = NULL;

  if (!wp_core_load_component_finish(core, res, &error)) {
    fprintf(stderr, "%s\n", error->message);
  }

  --self->pending_plugins;

  if (self->pending_plugins == 0) {
    self->def_nodes_api = wp_plugin_find(core, "default-nodes-api");
    wp_object_activate(WP_OBJECT(self->def_nodes_api),
                       WP_PLUGIN_FEATURE_ENABLED, NULL,
                       (GAsyncReadyCallback)on_plugin_activated, &self);

    self->mixer_api = wp_plugin_find(core, "mixer-api");
    g_object_set(G_OBJECT(self->mixer_api), "scale", 1 /* cubic */, NULL);
    self->pending_plugins++;
    wp_object_activate(WP_OBJECT(self->mixer_api), WP_PLUGIN_FEATURE_ENABLED,
                       NULL, (GAsyncReadyCallback)on_plugin_activated, &self);

    /* run */
    wp_core_install_object_manager(core, self->om);
    g_signal_connect_object(self->om, "installed", (GCallback)monitor_run, self,
                            0);
  }
}

static void wp_ctl_enable(WpPlugin *plugin, WpTransition *transition) {
  WpCtl *self = WP_CTL(plugin);
  g_autoptr(GError) error = NULL;
  g_return_if_fail(core);

  setlocale(LC_ALL, "");
  setlocale(LC_NUMERIC, "C");

  self->om = wp_object_manager_new();

  /* prepare the subcommand */
  if (!monitor_prepare(self, &error)) {
    fprintf(stderr, "%s\n", error->message);
    return;
  }

  /* load required API modules */
  self->pending_plugins++;
  wp_core_load_component(core, "libwireplumber-module-default-nodes-api",
                         "module", NULL, NULL, NULL,
                         (GAsyncReadyCallback)on_plugin_loaded, self);
  self->pending_plugins++;
  wp_core_load_component(core, "libwireplumber-module-mixer-api", "module",
                         NULL, NULL, NULL,
                         (GAsyncReadyCallback)on_plugin_loaded, self);
}

static void wp_ctl_class_init(WpCtlClass *klass) {
  WpPluginClass *plugin_class = (WpPluginClass *)klass;

  plugin_class->enable = wp_ctl_enable;
  plugin_class->disable = wp_ctl_clear;
}

WP_PLUGIN_EXPORT GObject *
wireplumber__module_init(WpCore *wp_core, GVariant *args, GError **error) {
  core = wp_core;
  return G_OBJECT(g_object_new(wp_ctl_get_type(), "name", "wpmonitor", "core",
                               wp_core, NULL));
}
