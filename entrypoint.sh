#!/bin/sh
set -e

CERT_DIR=/app/certs
KEY_FILE=${CERT_DIR}/localhost-key.pem
CRT_FILE=${CERT_DIR}/localhost.pem

if [ ! -f "$KEY_FILE" ] || [ ! -f "$CRT_FILE" ]; then
  echo "🔐 Génération de certificats TLS auto-signés..."
  mkdir -p "$CERT_DIR"
  openssl req -x509 -newkey rsa:4096 -days 365 -nodes \
    -keyout "$KEY_FILE" \
    -out "$CRT_FILE" \
    -subj "/CN=localhost"
fi

exec /app/blog-api
