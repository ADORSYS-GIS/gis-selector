# Applicant Selection App

## Overview

The **Applicant Selection App** is a Rust-based application designed to automatically select applicants for our GIS training program based on specified criteria. It uses the Actix-web framework for the backend and Diesel ORM for database interactions.

## Features

- **REST API**: Exposes endpoints to interact with the application.
- **Applicant Selection**: Automatically selects applicants based on experience and education criteria.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [PostgreSQL](https://www.postgresql.org/download/) (database server)
- [libpq-dev](https://www.postgresql.org/docs/current/libpq.html) (PostgreSQL client library) - Install with `sudo apt-get install libpq-dev`

### Setup

1. **Clone the Repository**

   ```bash
   git clone https://github.com/your-username/applicant-selection-app.git
   cd applicant-selection-app
   ```
2. **Configure Environment**

- Create a `.env` file in the `backend` directory with the following 

  content:

   ```bash
   DATABASE_URL=postgres://user:password@localhost/applicant_db
   HOST=127.0.0.1
   PORT=8080
   ```
3. **Build and Run**
- Build the project:
   ```bash
   cargo build
   ```
- Run the application:
   ```bash
   cargo run
   ```
### API Endpoints
#### POST /select
- **Description**: Selects applicants based on the provided criteria.

- **Request Body**:
  ```json
  {
    "experience": 5,
    "education": "Bachelor's"
  }
  ```
- **Response**:
  ```json
      [
        {
          "name": "John Doe",
          "score": 85
        },
        {
          "name": "Jane Smith",
          "score": 90
        }
      ]
  ```

## License

This project is licensed under the Apache License. See the LICENSE file for details.

## Contributing

Feel free to submit issues or pull requests to improve the application.
