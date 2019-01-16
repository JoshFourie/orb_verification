# orb_verification
Orb Verification layer for OMNIA Protocol.

# Set-Up for Diesel

Initialise the env and run a test:
    1. sudo apt-get install libpqdev 
    2. sudo apt install postgresql postgresql-contrib
        * setup a password for postgresql
    3. echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
        * replace username and password with the relevant account set-up with postgres
    4. cargo install diesel_cli
    5. diesel setup
    6. cargo test