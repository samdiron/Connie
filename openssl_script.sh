openssl req -x509 -nodes -newkey rsa:2048 -keyout ~/.config/connie/keys/key.pem -out ~/.config/connie/certificates/cert.pem -days 365 -subj "/CN=No-Domain Server" -extensions v3_req -config <(
  cat <<-EOF
  [req]
  distinguished_name = req_distinguished_name
  req_extensions = v3_req
  prompt = no
  [req_distinguished_name]
  CN = No-Domain Server
  stateOrProvinceName = N/A
  localityName = N/A
  organizationName = Self-signed certificate
  commonName = 192.168.7.13: Self-signed certificate
  [v3_req]
  subjectAltName = @alt_names
  [alt_names]
  IP.2 = 192.168.7.13
EOF
)
