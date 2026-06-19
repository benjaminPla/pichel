# Pichel — TODO

- [ ] `api/Dockerfile`
- [ ] `web/Dockerfile`
- [ ] `web/nginx.prod.conf`
- [ ] `cargo sqlx prepare` → commit `.sqlx/` for offline builds
- [x] `docker-compose.prod.yml`
- [ ] `.env.example`
- [x] `.github/workflows/build.yml`
- [ ] GitHub secrets: `DOCKERHUB_USERNAME`, `DOCKERHUB_TOKEN`, `HETZNER_HOST`, `HETZNER_USER`, `HETZNER_SSH_KEY`
- [ ] `infra/pichel.service` → `systemctl enable` on server
- [ ] `infra/backup.sh` + rclone B2 config on server + cron
- [ ] Hetzner: create dirs, paste `.env`, start systemd service
- [ ] Run DB migrations on prod
- [ ] DNS → point to Hetzner IP
- [ ] certbot SSL → uncomment HTTPS blocks in `nginx.prod.conf`, rebuild web image

- [ ] Product image file upload endpoint (saves to `/app/uploads`)
- [ ] Admin UI: image upload field instead of URL string
