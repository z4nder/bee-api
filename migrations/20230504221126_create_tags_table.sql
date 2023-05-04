CREATE TABLE IF NOT EXISTS tags
(
    id    BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    name  VARCHAR(255) NOT NULL,
    color VARCHAR(255) NOT NULL,

    created_by BIGINT UNSIGNED NOT NULL,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    FOREIGN KEY (created_by) REFERENCES users(id)
);