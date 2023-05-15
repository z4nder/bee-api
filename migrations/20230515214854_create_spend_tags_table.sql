-- Add migration script here
CREATE TABLE IF NOT EXISTS spends_tags
(
    spend_id BIGINT UNSIGNED NOT NULL,
    tag_id BIGINT UNSIGNED NOT NULL,

    FOREIGN KEY (spend_id) REFERENCES spends(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);