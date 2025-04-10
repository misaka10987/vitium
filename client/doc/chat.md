# Vitium Chat Server API Documentation

## Overview
This document provides a comprehensive guide to the chat server API implemented in `chat.rs`, designed for real-time messaging applications.

## Table of Contents
- [API Endpoints](#api-endpoints)
- [Message Format](#message-format)
- [Authentication](#authentication)
- [Real-time Mechanisms](#real-time-mechanisms)
- [Command System](#command-system)
- [Implementation Details](#implementation-details)

## API Endpoints

The chat server exposes three main REST endpoints:

### GET / - Fetch Messages in Real-time
Provides real-time updates for new messages using either SSE or long polling.

**Request:**
- No body required
- Header `Accept: text/event-stream` to use SSE (recommended)

**Response:**
- SSE stream of Message objects or
- Single Message object (long polling)

### POST / - Create Message
Sends a new message to the chat server.

**Request:**
- Requires authentication token
- JSON body with Message object

**Response:**
- 200 OK on success
- 403 Forbidden if token doesn't match sender

### GET /{time} - Message History
Retrieves all messages after the specified timestamp.

**Request:**
- Path parameter: time (millisecond timestamp)

**Response:**
- JSON array of Message objects

## Message Format

Messages use the following JSON structure:

**Fields:**
- `time`: Millisecond timestamp
- `sender`: Username of sender (empty for system messages)
- `content`: Message text content
- `html`: Boolean flag indicating if content contains HTML

## Authentication

- Uses JWT tokens through `axum_pass::Token`
- Token must match the sender field in messages
- Authentication is required for posting messages

## Real-time Mechanisms

The server supports two methods of delivering real-time updates:

### Server-Sent Events (SSE)
- Modern, efficient streaming protocol
- Automatically used when client includes `Accept: text/event-stream` header
- Provides continuous connection with automatic reconnection
- Events delivered as they occur
- Lower overhead than long polling

### Long Polling
- Fallback mechanism for broader compatibility
- Client makes HTTP request that server holds open until new data is available
- Client must reconnect after receiving each message
- Higher server resource usage

## Command System

Special messages starting with `/` are treated as commands:

- Regular users access standard commands via `s.cmd()`
- Operators have additional privileges via `s.op_cmd()`
- Command results are broadcast to all users as system messages
- Format: `{username} /command -- {result}`

## Implementation Details

The chat server is built with:

- **Axum**: Modern Rust web framework
- **SQLite**: Persistent message storage
- **Tokio watch channels**: Efficient message broadcasting
- **JWT**: Authentication mechanism

The architecture provides:
- Persistent message storage
- Efficient real-time notifications
- Backward compatibility
- Command processing for moderation