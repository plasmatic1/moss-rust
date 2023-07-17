CREATE TABLE IF NOT EXISTS fingerprints (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    hash BIGINT UNSIGNED NOT NULL, -- hash of the fingerprint
    loc INTEGER NOT NULL, -- location of the fingerprint inside the original file
    lang VARCHAR(16) NOT NULL, -- language of the file (normalized file extension)
    path VARCHAR(64) NOT NULL, -- path of the file (relative to the fs)
    fs_id VARCHAR(64) NOT NULL -- ID of the filesystem
);