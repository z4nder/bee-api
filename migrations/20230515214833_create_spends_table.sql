-- Add migration script here
CREATE TABLE IF NOT EXISTS spends
(
    id    BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    name  VARCHAR(255) NOT NULL,
    date TIMESTAMP NOT NULL,
    value  DECIMAL(5,2) NOT NULL,

    created_by BIGINT UNSIGNED NOT NULL,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    FOREIGN KEY (created_by) REFERENCES users(id)
);