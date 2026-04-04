#!/bin/bash
cd "/home/www/centrale/proxy"
git pull
cd "/home/www/centrale/target/release"
cargo build --release
sudo setcap 'cap_net_bind_service=+ep' /home/www/centrale/target/release/proxy
service centrale restart
systemctl status centrale
