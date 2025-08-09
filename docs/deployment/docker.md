# Docker

## Build
```bash
docker build -t omnivore:latest .
```

## Run
```bash
docker run -it --rm -p 3000:3000 omnivore:latest
```

## Compose
If using `docker-compose.yml` in the repo:
```bash
docker-compose up -d
```
