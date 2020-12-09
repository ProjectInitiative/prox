#!/usr/bin/env sh
mv ./prox-server /usr/local/bin/ && echo "Program installed."
mv ./prox-server-daemon.service /etc/systemd/system/ && echo "Service installed."

systemctl enable prox-server-daemon && echo "System service enabled."