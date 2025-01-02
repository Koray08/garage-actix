# Project Setup

## Frontend
cd frontend
serve .

## BackEnd
cd backend
echo "DATABASE_URL=sqlite:data/database.db" > .env
touch data/database.db 
cargo sqlx migrate run
cargo run