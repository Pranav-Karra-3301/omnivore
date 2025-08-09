# REST API

Base URL: `http://localhost:3000`

## `GET /health`
Health check.

Response: `200 OK`, body `OK`

## `POST /api/crawl`
Start a crawl.

Request body:
```json
{
  "url": "https://example.com",
  "max_depth": 5,
  "max_workers": 10
}
```

Response body:
```json
{
  "id": "<uuid>",
  "status": "started",
  "message": "Crawl started for URL: https://example.com"
}
```

## `GET /api/stats`
Retrieve last crawl statistics.

Responses:
```json
{ "status": "completed", "stats": { /* CrawlStats */ } }
```
```json
{ "status": "no_data", "message": "No crawl statistics available" }
```

## Root
`GET /` returns a JSON descriptor including available endpoints and version.
