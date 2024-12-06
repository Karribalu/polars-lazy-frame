# Polars Web Application

This is a Rust-based web application that integrates Actix Web for building APIs and Polars for data processing. It also interacts with a PostgreSQL database for data storage and retrieval.

## Features

- **RESTful API**: Endpoints to fetch and display user data.
- **PostgreSQL Integration**: Manages a `users` table with sample data.
- **Polars Integration**: Utilizes Polars for efficient data handling and processing.

## Getting Started

### Prerequisites

- Rust (latest stable version recommended)
- PostgreSQL
- `dotenv` file with the following variable:
  ```env
  DATABASE_URL=postgres://username:password@localhost/dbname
  ```
  

### Installation
- Clone the repo
```env
git clone https://github.com/karribalu/polars-lazy-frame.git
cd polars-web-app  
```
- Install dependencies:
```env
cargo build
```
- Run the application:
```env
cargo run
```

The application will start at http://localhost:8102

Project Structure API Endpoints
- GET `/api/hey` - Returns a "Hello world!" message.
- GET `/api/users` - Fetches all users from the database and prints the LazyFrame in the logs


### Project Structure

- `src/main.rs`: Application entry point.
- `src/handlers.rs`: Defines API routes and handlers.
- `src/models.rs`: Data model definitions.
- `src/db.rs`: Database interaction logic.