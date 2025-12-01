# One shot

cargo run -- --verbose --project **PROJECT_NAME** --zone **ZONE** --name **NAME** --auth /path/to/key.json

# Loop ( every 300 seconds / 5 minutes )

cargo run -- --poll-frequency 300 --verbose --project **PROJECT_NAME** --zone **ZONE** --name **NAME** --auth /path/to/key.json
