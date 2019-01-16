CREATE TABLE true_orbs (
    id SERIAL PRIMARY KEY,
    proof VARCHAR NOT NULL,
    verified_values VARCHAR NOT NULL,
    signature VARCHAR NOT NULL,
    public_key VARCHAR NOT NULL,
    common_reference_string VARCHAR NOT NULL
)