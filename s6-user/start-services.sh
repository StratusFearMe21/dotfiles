#!/bin/sh
s6-rc-init -c /home/${USER}/.local/share/s6/rc/compiled -l /tmp/${USER}/s6-rc /tmp/${USER}/service
s6-rc -l /tmp/${USER}/s6-rc -up change $@