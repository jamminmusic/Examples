# oauth2

THIS IS A WORK IN PROCESS - NOT COMPLETED - NOT FOR PRODUCTION.

## Link Definition Notes

### Redis

Starting Redis:

- `redis-server`
- default url: `127.0.0.1:6379`
- provider oci ref: `wasmcloud.azurecr.io/kvredis:0.16.3`
- linkname = "redis"

<https://github.com/wasmCloud/capability-providers/tree/main/kvredis>

### Hashicorp Vault

Starting Vault:

- `vault server -dev -dev-root-token-id="root"`
- `export VAULT_ADDR='http://127.0.0.1:8200'`
- default addr: `http://127.0.0.1:8200`
- provider oci ref: `wasmcloud.azurecr.io/kv-vault:0.2.3`
- linkname = "vault"

Set Secrets for Test:

- `vault kv put -mount=secret spotify client_id="some-id" client_secret="some_secret" auth_url="https://accounts.spotify.com/authorize" token_url="https://accounts.spotify.com/api/token" redirect_url="https://oauth.pstmn.io/v1/callback" scope="user-read-email user-read-private"`

<https://github.com/wasmCloud/capability-providers/tree/main/kv-vault>

### Nats Messaging

Provider oci ref: `wasmcloud.azurecr.io/nats_messaging:0.15.0`
Link-Def is based on NGS for Messaging server.

<https://github.com/wasmCloud/capability-providers/tree/main/nats>

### Jammin Nats Messaging

Provider oci ref: from file for now
jammin:interfaces:messaging
Link-Def is based on NGS for Messaging server.
start nats server: nats-server -p 5000 -m 8222
{
    "SUBSCRIPTION": "oauth2.login.*,
    "MESSAGE_COUNT"="2"
    "URI": "tls://connect.ngs.global", // or 0.0.0.0:5000 (port other than one used, nats default is 4222)
    "CLIENT_SEED": "found in /home/$user/.local/share/nats/nsc/keys/creds/Synadia Communications Inc./jammin_music",
    "CLIENT_JWT": "found in /home/$user/.local/share/nats/nsc/keys/creds/Synadia Communications Inc./jammin_music"
}

### httpclient Provider

Provider oci ref: `wasmcloud.azurecr.io/httpclient:0.6.0`
default link def

<https://github.com/wasmCloud/capability-providers/tree/main/httpclient>

### httpserver Provider

Provider oci ref: `## httpclient Provider

Provider oci ref: `wasmcloud.azurecr.io/httpserver:0.16.0`
default link def

<https://github.com/wasmCloud/capability-providers/tree/main/httpserver>
