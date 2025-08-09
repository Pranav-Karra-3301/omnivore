# Politeness & robots.txt

Omnivore includes a politeness engine:

- `respect_robots_txt`: enable robots.txt compliance
- `default_delay_ms`: base delay between requests
- `max_requests_per_second`: throttle ceiling
- `backoff_multiplier`: exponential backoff on errors

Tune per your target domains to balance speed and courtesy.
