#!/bin/bash
cd "/home/www/centrale/proxy"
git pull
cd "/home/www/centrale/target/release"
cargo build --release
sudo service centrale restart
systemctl status centrale
