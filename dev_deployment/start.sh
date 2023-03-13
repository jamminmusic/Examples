#!/bin/sh
set -e

# set env secrets
set -o allexport
source ./.env
set +o allexport

konsole --hold -e docker-compose up &

sleep 20
# create tls cert - also using dns cert with caddy in container (likely redundant but there for later)
docker-compose exec tailscale sh -c "tailscale --socket /tmp/tailscaled.sock cert some.tailnet.ts.net"

sleep 1
# populate vault
docker-compose exec vault sh -c "vault kv put -mount=secret spotify "client_id=$CLIENT_ID client_secret=$CLIENT_SECRET auth_url=https://accounts.spotify.com/authorize token_url=https://accounts.spotify.com/api/token redirect_url=https://jamminmusic.dev:8080 scope='user-read-email user-read-private'""

# deploy app
docker-compose exec wash sh -c "wash app put /etc/wadm/wadm.yaml"
sleep 1
docker-compose exec wash sh -c "wash app deploy oauth2 0.0.1 -o json"
