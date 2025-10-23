=#!/bin/bash
set -e

# Define the location of the .env file (change if needed)
ENV_FILE="./auth-service/.env"

# Check if the .env file exists
if [[ ! -f "$ENV_FILE" ]]; then
  echo "❌ Error: .env file not found at $ENV_FILE"
  exit 1
fi

echo "✅ Loading environment variables from $ENV_FILE"

# Export variables from .env file
while IFS='=' read -r key value; do
  # Skip comments and blank lines
  [[ -z "$key" || "$key" =~ ^# ]] && continue
  # Remove possible surrounding quotes
  value=$(echo "$value" | sed -e 's/^"//' -e 's/"$//')
  export "$key"="$value"
done < "$ENV_FILE"

echo "✅ Environment variables loaded."

# Run docker-compose commands
docker compose build
docker compose up
