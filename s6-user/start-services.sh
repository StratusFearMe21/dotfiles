#!/bin/sh
s6-rc-init -c /etc/s6-user/rc/compiled -l /tmp/${USER}/s6-rc /tmp/${USER}/service
s6-rc -l /tmp/${USER}/s6-rc -up change $@