FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && apt-get clean

COPY edgeflow /app/edgeflow

WORKDIR /app

EXPOSE 80 443

ENTRYPOINT ["/app/edgeflow"]
