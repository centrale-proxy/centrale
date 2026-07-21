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
service centrale_admin stop
service centrale_admin start
journalctl -u centrale_admin.service -f --no-hostnameme
