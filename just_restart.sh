#!/bin/bash
sudo setcap 'cap_net_bind_service=+ep' /home/www/centrale/target/release/centrale
systemctl daemon-reload
service centrale stop
service centrale start
