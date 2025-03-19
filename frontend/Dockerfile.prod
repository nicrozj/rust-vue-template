FROM node:lts-alpine AS builder
WORKDIR /app

COPY ./frontend/package*.json ./
RUN npm install

COPY ./frontend ./
RUN npm run build

FROM caddy:alpine
COPY --from=builder /app/dist /usr/share/caddy
COPY ./Caddyfile.prod /etc/caddy/Caddyfile
