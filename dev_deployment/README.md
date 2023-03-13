# Setup

Can use an A record and CNAMES to point to tailscale IP.

Lets Encrypt:
<https://letsencrypt.org/getting-started/>

Cert Bot:
<https://certbot.eff.org/>

GCloud Provider:
<https://go-acme.github.io/lego/dns/gcloud/>

Docker Compose External Network Docs:
<https://docs.docker.com/compose/compose-file/#external>

Docker Compose with Caddy and Tailscale:
<https://www.reddit.com/r/Tailscale/comments/104y6nq/docker_tailscale_and_caddy_with_https_a_love_story/>

- `docker exec tailscaled tailscale --socket /tmp/tailscaled.sock cert <the server domain name>`

## Resources

- <https://www.reddit.com/r/Tailscale/comments/104y6nq/docker_tailscale_and_caddy_with_https_a_love_story/>
- <https://forum.tailscale.com/t/is-it-possible-to-run-tailscale-both-in-docker-containers-and-on-the-host/3042>
- <https://www.cloudflare.com/products/tunnel/>
- <https://www.reddit.com/r/Tailscale/comments/yuczp2/running_two_instances_of_tailscale_simultaneously/>
- <https://github.com/caddy-dns/googleclouddns>
