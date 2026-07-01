#!/bin/bash
cd "/home/www/centrale/centrale"
git pull
git rebase
cd "/home/www/centrale/target/release"
cargo build --release
sudo setcap 'cap_net_bind_service=+ep' /home/www/centrale/target/release/load_balancer
service centrale stop
service centrale start
service load_balancer stop
service load_balancer start
journalctl -u load_balancer.service -f
