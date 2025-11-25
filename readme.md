# Unicloud Rust

> A monorepo containing a under-developing full-stack application that provides services for managing cloud accounts securely. It features a robust Rust backend with a RESTful API and WebSocket capabilities, coupled with a modern Next.js frontend.


## ✨ Key Features

*   **User Authentication:** Supports Google-based authentication for secure access.
*   **Cloud Account Management:** Integration capabilities for managing various cloud storage accounts, such as Google Drive.
*   **User and Quota Management:** Handles user profiles and associated resource quotas.
*   **Database Migrations:** Automated database schema management using SeaORM.
*   **RESTful API:** Provides well-defined endpoints for core application functionalities.
*   **Real-time Communication:** A dedicated WebSocket service facilitates real-time interactions.
*   **Modern Web UI:** A responsive and interactive user interface built with Next.js.

## 🛠️ Tech Stack

*   **Backend:**
    *   Rust
    *   Axum (Web Framework)
    *   Tokio-Tungstenite (WebSocket Framework)
    *   SeaORM (ORM for database interaction and migrations)
    *   JSON Web Tokens (JWT) for authentication
    *   Heavy Oauth2.0 written from scratch 
*   **Frontend:**
    *   Next.js
    *   TypeScript
    *   Zustand (State Management)
    *   Tailwind CSS (Styling)

## 🚀 Installation

This project is a monorepo consisting of separate frontend and backend components. Each component has its own installation process.

### Backend

1.  Navigate to the backend directory:
    ```bash
    cd backend
    ```
2.  Build the backend services:
    ```bash
    cargo build
    ```
3.  Set up environment variables by copying `.env.example` in `backend` to `.env` and filling in the required values.

4.  To migrate database and generate entities follow the instructions in `backend/migrations/src/main.rs` and run commands from the `backend folder`.
5.  To run HTTP server
    ```bash
    cargo run -p api
    ```
    To run WebSocket server
    ```bash
    cargo run -p ws
    ```
### Frontend

1.  Navigate to the frontend directory:
    ```bash
    cd frontend
    ```
2.  Install dependencies using npm or bun:
    ```bash
    npm install
    # or
    bun install
    ```
3.  Set up environment variables by copying `.env.example` to `.env` and filling in the required values.

## Usage

### Running the Backend

From the `backend` directory:

1.  To run the API service:
    ```bash
    cd services/api
    cargo run
    ```
2.  To run the WebSocket service:
    ```bash
    cd services/ws
    cargo run
    ```

### Running the Frontend

From the `frontend` directory:

```bash
npm run dev
# or
bun run dev
```

After starting both the backend and frontend, access the application in your web browser, typically at `http://localhost:3000`.

## 🔧 How It Works

The application is architected as a monorepo, cleanly separating its frontend and backend concerns.

*   **Frontend (`frontend`):** A Next.js application built with TypeScript and utilizing Zustand for robust state management, particularly for authentication flows. Users interact with the application through various pages, including `login` and `home`, which make API requests to the backend for data and actions.
*   **Backend (`backend`):** Developed in Rust, the backend consists of several services and libraries working in concert:
    *   **API Service (`backend/services/api`):** This service, powered by the Axum web framework, exposes RESTful endpoints. It handles critical functionalities such as user authentication (e.g., `login_with_google`), integration with cloud providers (`add_google_drive`), and managing user-specific data. JWTs are employed for secure session management. Database operations are abstracted through a database connection utility (`db_connect`) and leverage an ORM (likely SeaORM).
    *   **WebSocket Service (`backend/services/ws`):** A dedicated Rust service responsible for managing real-time communication channels.
    *   **Entities (`backend/libs/entities`):** This library defines the core data models of the application (e.g., `cloud_account`, `users`, `quota`), which are used by the ORM for structured database interactions.
    *   **Migration (`backend/migration`):** Manages the evolution of the database schema using SeaORM's migration capabilities, ensuring data consistency across different versions of the application.
    *   **Common Library (`backend/libs/common`):** Provides shared code and utilities that can be used across various backend services, promoting code reuse and consistency.

The frontend communicates with the backend's API service to authenticate users, manage cloud accounts, and retrieve application data. The API, in turn, interacts with the database using the defined entities and handles the necessary business logic and authorization. Real-time features are facilitated by the separate WebSocket service, enhancing user experience with instant updates.