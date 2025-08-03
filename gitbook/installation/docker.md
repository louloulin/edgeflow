# Docker

Similar to other proxies, EdgeFlow can be run as a Docker container. The following command will run the latest version of it:

```bash
docker run -d -p 80:80 -p 443:443 -v /path/to/config:/etc/edgeflow/ luizfonseca/edgeflow
```

If you are using `docker-compose.yml`  to manage your services, you can configure EdgeFlow as your main host-mode container without even creating a `edgeflow.hcl` file.

```yaml
version: '3.8'
services:
  edgeflow:
    environment:
      EDGEFLOW_LOGGING__LEVEL: "info"
      EDGEFLOW_WORKER_THREADS: 2

      # Enables EdgeFlow to fetch services/containers 
      # matching Smart labels 
      EDGEFLOW_DOCKER__ENABLED: "true"
      EDGEFLOW_DOCKER__MODE: container

      EDGEFLOW_LETS_ENCRYPT__ENABLED: "true"
      EDGEFLOW_LETS_ENCRYPT__STAGING: "true"
      EDGEFLOW_LETS_ENCRYPT__EMAIL: "contact@email.net"

      EDGEFLOW_PATHS__LETS_ENCRYPT: "/etc/edgeflow/certs"
    image: luizfonseca/edgeflow:latest
    networks:
      # Any service in the same network will be able to communicate with EdgeFlow
      - web 
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
      - /path/to/config:/etc/edgeflow/certs
```

And then you can expose any service by using `edgeflow.host` and `edgeflow.enable` labels. For example a simple `nginxdemos/hello` container/service:

```yaml
services:
  # ... (include EdgeFlow configuration)
  web:
    image: nginxdemos/hello

    networks:
      - public
      - shared
    deploy:
      replicas: 2
    labels:
      edgeflow.enabled: "true"
      edgeflow.host: "your-site.localhost"
      edgeflow.port: "80" # no need to publish host ports

      # If you are running locally
      edgeflow.ssl_certificate.self_signed_on_failure: "true"
```

