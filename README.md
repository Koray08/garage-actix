# Project Setup

## Frontend
cd frontend
serve .

## BackEnd
cd backend
echo "DATABASE_URL=sqlite:data/database.db" > .env
mkdir -p data && touch data/database.db
cargo sqlx migrate run
cargo run