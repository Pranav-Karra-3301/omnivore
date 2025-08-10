-- Omnivore Database Initialization Script
-- This script sets up the initial database schema for Omnivore

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";

-- Create omnivore database if it doesn't exist
-- Note: This would typically be done outside the script since we're already connecting to the omnivore database

-- Sessions table for tracking crawl sessions
CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    start_url TEXT NOT NULL,
    max_depth INTEGER DEFAULT 5,
    max_workers INTEGER DEFAULT 10,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'running', 'completed', 'failed', 'paused')),
    settings JSONB DEFAULT '{}'::jsonb
);

-- Pages table for storing crawled pages
CREATE TABLE IF NOT EXISTS pages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id UUID REFERENCES sessions(id) ON DELETE CASCADE,
    url TEXT NOT NULL,
    title TEXT,
    content TEXT,
    html_content TEXT,
    status_code INTEGER,
    response_headers JSONB,
    content_type VARCHAR(255),
    content_length BIGINT,
    depth INTEGER NOT NULL DEFAULT 0,
    parent_url TEXT,
    discovered_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    crawled_at TIMESTAMP WITH TIME ZONE,
    metadata JSONB DEFAULT '{}'::jsonb,
    error_message TEXT
);

-- Links table for tracking discovered links
CREATE TABLE IF NOT EXISTS links (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id UUID REFERENCES sessions(id) ON DELETE CASCADE,
    source_page_id UUID REFERENCES pages(id) ON DELETE CASCADE,
    source_url TEXT NOT NULL,
    target_url TEXT NOT NULL,
    link_text TEXT,
    link_type VARCHAR(50) DEFAULT 'href',
    discovered_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    processed BOOLEAN DEFAULT FALSE
);

-- Entities table for knowledge graph
CREATE TABLE IF NOT EXISTS entities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id UUID REFERENCES sessions(id) ON DELETE CASCADE,
    page_id UUID REFERENCES pages(id) ON DELETE CASCADE,
    entity_type VARCHAR(100) NOT NULL,
    entity_value TEXT NOT NULL,
    confidence FLOAT DEFAULT 0.0,
    context TEXT,
    position_start INTEGER,
    position_end INTEGER,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Relationships table for entity relationships
CREATE TABLE IF NOT EXISTS relationships (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id UUID REFERENCES sessions(id) ON DELETE CASCADE,
    source_entity_id UUID REFERENCES entities(id) ON DELETE CASCADE,
    target_entity_id UUID REFERENCES entities(id) ON DELETE CASCADE,
    relationship_type VARCHAR(100) NOT NULL,
    confidence FLOAT DEFAULT 0.0,
    context TEXT,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Domain politeness tracking
CREATE TABLE IF NOT EXISTS domain_politeness (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    domain VARCHAR(255) UNIQUE NOT NULL,
    last_request_at TIMESTAMP WITH TIME ZONE,
    delay_ms INTEGER DEFAULT 1000,
    requests_count BIGINT DEFAULT 0,
    robots_txt TEXT,
    robots_fetched_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Crawl statistics
CREATE TABLE IF NOT EXISTS crawl_stats (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id UUID REFERENCES sessions(id) ON DELETE CASCADE,
    pages_discovered BIGINT DEFAULT 0,
    pages_crawled BIGINT DEFAULT 0,
    pages_failed BIGINT DEFAULT 0,
    entities_extracted BIGINT DEFAULT 0,
    relationships_found BIGINT DEFAULT 0,
    total_bytes BIGINT DEFAULT 0,
    average_response_time_ms FLOAT DEFAULT 0.0,
    recorded_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_pages_session_id ON pages(session_id);
CREATE INDEX IF NOT EXISTS idx_pages_url ON pages USING hash(url);
CREATE INDEX IF NOT EXISTS idx_pages_status_code ON pages(status_code);
CREATE INDEX IF NOT EXISTS idx_pages_depth ON pages(depth);
CREATE INDEX IF NOT EXISTS idx_pages_crawled_at ON pages(crawled_at);

CREATE INDEX IF NOT EXISTS idx_links_session_id ON links(session_id);
CREATE INDEX IF NOT EXISTS idx_links_source_page_id ON links(source_page_id);
CREATE INDEX IF NOT EXISTS idx_links_target_url ON links USING hash(target_url);
CREATE INDEX IF NOT EXISTS idx_links_processed ON links(processed);

CREATE INDEX IF NOT EXISTS idx_entities_session_id ON entities(session_id);
CREATE INDEX IF NOT EXISTS idx_entities_page_id ON entities(page_id);
CREATE INDEX IF NOT EXISTS idx_entities_type ON entities(entity_type);
CREATE INDEX IF NOT EXISTS idx_entities_value ON entities USING gin(entity_value gin_trgm_ops);

CREATE INDEX IF NOT EXISTS idx_relationships_session_id ON relationships(session_id);
CREATE INDEX IF NOT EXISTS idx_relationships_source ON relationships(source_entity_id);
CREATE INDEX IF NOT EXISTS idx_relationships_target ON relationships(target_entity_id);
CREATE INDEX IF NOT EXISTS idx_relationships_type ON relationships(relationship_type);

CREATE INDEX IF NOT EXISTS idx_domain_politeness_domain ON domain_politeness(domain);
CREATE INDEX IF NOT EXISTS idx_domain_politeness_last_request ON domain_politeness(last_request_at);

CREATE INDEX IF NOT EXISTS idx_crawl_stats_session_id ON crawl_stats(session_id);
CREATE INDEX IF NOT EXISTS idx_crawl_stats_recorded_at ON crawl_stats(recorded_at);

-- Updated at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply updated_at triggers
CREATE TRIGGER update_sessions_updated_at 
    BEFORE UPDATE ON sessions 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_domain_politeness_updated_at 
    BEFORE UPDATE ON domain_politeness 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Insert default session for development
INSERT INTO sessions (id, name, start_url, max_depth, max_workers, status) 
VALUES (
    uuid_generate_v4(),
    'Default Development Session',
    'https://example.com',
    5,
    10,
    'pending'
) ON CONFLICT DO NOTHING;

-- Create a view for session overview
CREATE OR REPLACE VIEW session_overview AS
SELECT 
    s.id,
    s.name,
    s.start_url,
    s.status,
    s.created_at,
    s.updated_at,
    COUNT(DISTINCT p.id) as total_pages,
    COUNT(DISTINCT CASE WHEN p.crawled_at IS NOT NULL THEN p.id END) as crawled_pages,
    COUNT(DISTINCT e.id) as total_entities,
    COUNT(DISTINCT r.id) as total_relationships
FROM sessions s
LEFT JOIN pages p ON s.id = p.session_id
LEFT JOIN entities e ON s.id = e.session_id
LEFT JOIN relationships r ON s.id = r.session_id
GROUP BY s.id, s.name, s.start_url, s.status, s.created_at, s.updated_at;

COMMIT;
