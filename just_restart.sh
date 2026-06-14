#!/bin/bash
sudo setcap 'cap_net_bind_service=+ep' /home/www/centrale/target/release/proxy
service centrale stop
service centrale start
journalctl -u centrale.service -f
