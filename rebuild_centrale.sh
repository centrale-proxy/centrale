#!/bin/bash
cd "/home/www/centrale/proxy"
git pull
git rebase
cd "/home/www/centrale/target/release"
cargo build --release
sudo setcap 'cap_net_bind_service=+ep' /home/www/centrale/target/release/proxy
service centrale start
systemctl status centrale
