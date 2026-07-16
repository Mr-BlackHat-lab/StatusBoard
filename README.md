# StatusBoard

### `StatusBoard` (Rust Axum & Svelte)

A full-stack uptime monitoring application. It acts as a miniature observability platform that checks if designated websites are online and displays their status.

- **Core Features:**
- **Backend (Rust/Axum):** Uses an asynchronous runtime to ping a hardcoded list of URLs (e.g., Google, GitHub, your own portfolio) every 60 seconds. It exposes a REST API endpoint (e.g., `/api/status`) that serves this data as JSON.
- **Frontend (Svelte):** Fetches the JSON data on load and sets up a polling mechanism to refresh the UI automatically. Displays a dashboard with green "Operational" or red "Down" indicators for each service.

- **Key Crates/Tools to Use:** `axum`, `tokio`, `reqwest` (for making HTTP requests), `serde` (for JSON serialization).
- **What I Learned:** Asynchronous programming, REST API design, Cross-Origin Resource Sharing (CORS), JSON parsing, and wiring a frontend client to a backend server.

---
