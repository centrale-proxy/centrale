#!/bin/bash
cd "/home/www/centrale/centrale"
git pull
git rebase
cd "/home/www/centrale/target/release"
cargo build --release
sudo setcap 'cap_net_bind_service=+ep' /home/www/centrale/target/release/centrale
service centrale stop
service centrale start
journalctl -u centrale.service -f
