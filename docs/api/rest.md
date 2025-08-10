# REST API Reference

The Omnivore REST API provides programmatic access to the crawler functionality.

## Base Configuration

- **Default URL**: `http://localhost:3000`
- **Content-Type**: `application/json`
- **CORS**: Enabled for all origins

## Starting the API Server

```bash
omnivore-api

# With custom port
omnivore-api --port 8080

# With custom config
omnivore-api --config config.toml
```

## Endpoints

### `GET /` - API Information

Returns API metadata and available endpoints.

#### Response
```json
{
  "name": "Omnivore API",
  "version": "0.1.0",
  "endpoints": [
    "/",
    "/health",
    "/api/crawl",
    "/api/stats",
    "/graphql"
  ]
}
```

### `GET /health` - Health Check

Check if the API server is running.

#### Response
- **Status**: `200 OK`
- **Body**: `"OK"`

#### Example
```bash
curl http://localhost:3000/health
```

### `POST /api/crawl` - Start Crawl

Initiate a new web crawl.

#### Request Body
```json
{
  "url": "https://example.com",
  "max_depth": 5,
  "max_workers": 10
}
```

#### Parameters
- `url` (required): Starting URL to crawl
- `max_depth` (optional): Maximum crawl depth (default: 5, max: 20)
- `max_workers` (optional): Number of concurrent workers (default: 10, max: 100)

#### Response
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "started",
  "message": "Crawl started for URL: https://example.com"
}
```

#### Example
```bash
curl -X POST http://localhost:3000/api/crawl \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com", "max_depth": 3}'
```

### `GET /api/stats` - Get Statistics

Retrieve statistics from the last crawl.

#### Response (Success)
```json
{
  "status": "completed",
  "stats": {
    "urls_visited": 150,
    "urls_queued": 0,
    "success_count": 145,
    "error_count": 5,
    "start_time": "2024-01-15T10:30:00Z",
    "end_time": "2024-01-15T10:35:30Z",
    "duration_secs": 330,
    "pages_per_second": 0.45,
    "average_response_time_ms": 250,
    "total_bytes_downloaded": 15728640
  }
}
```

#### Response (No Data)
```json
{
  "status": "no_data",
  "message": "No crawl statistics available"
}
```

#### Example
```bash
curl http://localhost:3000/api/stats
```

## Error Responses

All endpoints may return error responses in the following format:

```json
{
  "error": "Error message",
  "status": 400
}
```

### Common Error Codes
- `400 Bad Request`: Invalid request parameters
- `404 Not Found`: Endpoint not found
- `500 Internal Server Error`: Server error during processing

## Rate Limiting

Currently, the API does not implement rate limiting. For production use, consider:
- Adding a reverse proxy (nginx, Caddy) with rate limiting
- Implementing application-level rate limiting
- Using an API gateway

## Authentication

⚠️ **Not Implemented**: The API currently has no authentication. All endpoints are publicly accessible.

For production deployment:
1. Implement JWT or API key authentication
2. Use HTTPS/TLS
3. Restrict CORS origins
4. Add request validation

## Limitations

### Current Limitations
1. **No persistent sessions**: Crawl state is not preserved across API restarts
2. **Single crawl at a time**: Cannot run multiple crawls concurrently
3. **No crawl management**: Cannot pause, resume, or cancel crawls
4. **Limited statistics**: Only basic metrics are tracked
5. **No authentication**: All endpoints are public

### Not Implemented
- Crawl status endpoint (`GET /api/crawl/{id}`)
- Cancel crawl endpoint (`DELETE /api/crawl/{id}`)
- List crawls endpoint (`GET /api/crawls`)
- Configuration endpoint (`GET/POST /api/config`)
- WebSocket support for real-time updates

## Usage Examples

### Basic Crawl Workflow

1. **Start a crawl**:
```bash
CRAWL_ID=$(curl -s -X POST http://localhost:3000/api/crawl \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com"}' \
  | jq -r .id)
```

2. **Check statistics**:
```bash
curl http://localhost:3000/api/stats | jq
```

### Python Example

```python
import requests
import time

# Start crawl
response = requests.post(
    "http://localhost:3000/api/crawl",
    json={
        "url": "https://example.com",
        "max_depth": 3,
        "max_workers": 5
    }
)
crawl_id = response.json()["id"]

# Wait and check stats
time.sleep(30)
stats = requests.get("http://localhost:3000/api/stats").json()
print(f"Crawled {stats['stats']['urls_visited']} pages")
```

### JavaScript Example

```javascript
// Start crawl
fetch('http://localhost:3000/api/crawl', {
    method: 'POST',
    headers: {'Content-Type': 'application/json'},
    body: JSON.stringify({
        url: 'https://example.com',
        max_depth: 2
    })
})
.then(res => res.json())
.then(data => console.log('Crawl started:', data.id));

// Get stats
fetch('http://localhost:3000/api/stats')
    .then(res => res.json())
    .then(stats => console.log('Stats:', stats));
```

## Performance Considerations

1. **Concurrent Requests**: The API can handle multiple requests but only one crawl at a time
2. **Response Times**: Health check < 10ms, crawl start < 100ms
3. **Memory Usage**: Increases with crawl depth and worker count
4. **CPU Usage**: Scales with number of workers and parsing complexity

## Future Enhancements

Planned improvements for the REST API:
- WebSocket support for real-time crawl updates
- Batch crawl operations
- Crawl templates and presets
- Export formats (CSV, JSON, XML)
- Webhook notifications
- Metrics and monitoring endpoints