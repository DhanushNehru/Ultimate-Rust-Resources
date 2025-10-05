# Rocket SSE Chat

A simple real-time chat server built with **[Rocket](https://rocket.rs/)** in Rust, using **Server-Sent Events (SSE)** for live updates.  
Clients can send messages via HTTP POST requests, and all connected clients receive updates in real time.

---

## Features

- **Real-time messaging** with SSE
- **Publish/subscribe** using `tokio::sync::broadcast`
- Simple **REST API**
- Serves a static frontend from `/frontend`
- Lightweight and minimal dependencies

---

## Project Structure

├── src/
│ └── main.rs # Rocket server implementation
├── frontend/ # Static frontend files
│ ├── index.html
│ └── app.js
├── Cargo.toml
└── README.md


---

## API Endpoints

### POST `/message`

Send a message to the chat.

**Request Body** (Form Data):

| Field    | Type   | Description                             |
|----------|--------|-----------------------------------------|
| room     | String | Room name (max length 30)              |
| username | String | User name (max length 30)              |
| message  | String | The chat message                        |

### Frontend structure

frontend/
├── index.html
└── handle.js    # vanilla javascript for handling user-side operations
└── style.css   # styling css
└── settings.css # general settings

