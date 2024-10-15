# Node API

# TODO
## build basic structure of project
## Functions
    * Module devision
    * Toml config
    * Log tracing
    * Rate Limiting
    * Circuit breaker
    * API Keys

## API Design
### API path prefix
    * https://api.chainmap.com/{version}/{chain-name}/testnet
    * https://api.chainmap.com/{version}/{chain-name}/mainet

## DB Design
### tables
-- 创建 tokens 表，用于存储 token 相关信息
CREATE TABLE tokens (
    id SERIAL PRIMARY KEY,                      -- 自动递增的主键，唯一标识每个 token
    user_id INTEGER NOT NULL,                   -- 关联用户的 ID，外键关联到用户表（如果有用户表）
    token VARCHAR(255) NOT NULL,                -- 存储实际的 token 值（如 JWT）
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,  -- token 创建时间，默认是当前时间
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,  -- token 的过期时间，必填
    revoked BOOLEAN DEFAULT FALSE               -- token 是否被撤销，默认为 false 表示有效
);
-- 如果有用户表，可以通过以下方式添加外键关联：
-- ALTER TABLE tokens
-- ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;
-- 该约束确保当用户被删除时，相关的 token 也被删除


-- 创建 rate_limits 表，用于存储与每个 token 相关的限流配置
CREATE TABLE rate_limits (
    id SERIAL PRIMARY KEY,                           -- 自动递增的主键，唯一标识每个限流配置
    token_id INTEGER NOT NULL,                       -- 关联的 token ID，外键关联到 tokens 表的 id
    max_requests_per_second INTEGER NOT NULL,        -- 每秒最大请求数 (QPS)，如 10 表示每秒最多 10 个请求
    total_request_number INTEGER NOT NULL,           -- 总请求数限制，限制 token 总共能发起的最大请求数
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,  -- 限流配置的创建时间，默认是当前时间
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,  -- 最后一次更新限流配置的时间
    FOREIGN KEY (token_id) REFERENCES tokens(id) ON DELETE CASCADE  -- 外键关联到 tokens 表，token 被删除时自动删除对应的限流配置
);
-- 如果需要更新限流配置，可以通过 UPDATE 语句更新 max_requests_per_second 或 total_request_number：
-- UPDATE rate_limits
-- SET max_requests_per_second = 20, total_request_number = 2000, updated_at = CURRENT_TIMESTAMP
-- WHERE token_id = 1;

