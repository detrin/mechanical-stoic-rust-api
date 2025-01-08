# Mechanical Stoic Rust API
You can find the website [here](https://old.hermandaniel.com/#/mechanicalstoic).
## Motivation
This is my first attempt at creating a RESTful API using rust and axum framework. I am using this project to learn rust and axum. Plus who doesn't like to learn about the stoicism.

## How to run
1. Clone the repo.
2. Run `DATABASE_URL="postgresql://username:password@IP:port/mechanical_stoic" cargo run` in the root directory. The mechanical_stoic is database from postgres db, but it is not included in this repo. 
3. Open your browser and go to `http://localhost:3030/`
4. Run to get random quote
```shell
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{
    "resilience": 50,
    "acceptance": 20,
    "wisdom": 30
  }' \
  http://localhost:3030/api/v1/quotes
```

