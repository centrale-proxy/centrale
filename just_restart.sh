#!/bin/bash
sudo setcap 'cap_net_bind_service=+ep' /home/www/centrale/target/release/load_balancer
systemctl daemon-reload
service centrale stop
service centrale start
service load_balancer stop
service load_balancer start
service writer stop
service writer start
