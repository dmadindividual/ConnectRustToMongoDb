# MongoDB Client Example in Rust

This Rust application demonstrates how to connect to a MongoDB database and list all available database names. It uses the MongoDB Rust driver to establish the connection and execute the query.

## Features

- **MongoDB Connection**: Connect to a MongoDB database using a connection string.
- **Environment Variable**: Utilize an environment variable for the MongoDB connection string, ensuring sensitive information is not hardcoded.
- **Database Listing**: Fetch and display the names of all databases on the connected MongoDB server.
- **Asynchronous Execution**: Implement asynchronous operations using the Tokio runtime, improving performance and responsiveness.

## Prerequisites

- **Rust**: Ensure Rust is installed on your machine. You can download it from [rust-lang.org](https://www.rust-lang.org/).
- **MongoDB**: Have access to a MongoDB server, either a MongoDB Atlas cluster or a local MongoDB instance.
- **Connection String**: Obtain your MongoDB connection string, which includes your username, password, and cluster URL.

## Getting Started

### Setting Up

1. **Clone the Repository**:
    ```bash
    git clone https://github.com/yourusername/mongodb-client-example.git
    cd mongodb-client-example
    ```

2. **Set the Environment Variable**:
   Set the `MONGO_URL` environment variable with your MongoDB connection string.

   **PowerShell**:
   ```powershell
   $env:MONGO_URL = "mongodb+srv://your_username:your_password@your_cluster_url/myDatabase"
   ```

   **Command Prompt**:
   ```cmd
   set MONGO_URL=mongodb+srv://your_username:your_password@your_cluster_url/myDatabase
   ```

### Running the Application

Execute the application using Cargo:

```bash
cargo run
```

## Summary

This application is a simple yet effective example of how to connect to a MongoDB database using Rust. It highlights best practices for managing sensitive information through environment variables and demonstrates the use of asynchronous programming with Tokio for efficient database operations.