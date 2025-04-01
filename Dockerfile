FROM rust:1.85 AS builder

WORKDIR /app

# Install trunk
RUN rustup target add wasm32-unknown-unknown && \
    cargo install trunk --version 0.21.9

# Copy source code
COPY . .

# Build the application
RUN trunk build --release

# Nginx stage
FROM nginx:1.27.4-alpine

# Copy the built assets
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx configuration
RUN echo 'server { \
    listen 80; \
    server_name localhost; \
    location / { \
    root /usr/share/nginx/html; \
    try_files $uri $uri/ /index.html; \
    } \
    }' > /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]