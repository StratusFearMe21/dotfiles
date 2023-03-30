#!/bin/sh
s6-rc-init -c /home/${USER}/.local/share/s6/rc/compiled -l /tmp/s6-rc /tmp/s6-user-service
s6-rc -l /tmp/s6-rc -up change $@