# Setup

For local dev will be leveraging router for dns bindings, but there is potential to use dnsmasq if configured correctly.

e.g.:

- on linux requires dnsmasq with entry like `address=/.jammin.dev/127.0.0.1` in /etc/dnsmasq.conf
- then add or uncomment `name_servers=127.0.0.1` in /etc/resolvconf.conf <https://manpages.ubuntu.com/manpages/trusty/man5/resolvconf.conf.5.html>
