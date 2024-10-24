-- Add up migration script here
CREATE TABLE IF NOT EXISTS messages (
    id SERIAL NOT NULL PRIMARY KEY,
    message TEXT NOT NULL,
    send_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_automatic_updating
BEFORE UPDATE ON messages
FOR EACH ROW
EXECUTE FUNCTION update_automatic_updating_updated_at();