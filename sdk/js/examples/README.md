# EdgeFlow SDK Examples

This directory contains examples demonstrating how to use the EdgeFlow JavaScript SDK.

## Running the Examples

Before running the examples, set up the environment variables:

```bash
export EDGEFLOW_API_URL="https://your-edgeflow-instance.com"
export EDGEFLOW_API_KEY="your-api-key"
```

Or create a `.env` file in this directory with these variables.

### Basic Completion

A simple example showing how to send a completion request:

```bash
node basic-completion.js
```

### Streaming Completion

Example of using streaming for real-time responses:

```bash
node streaming-completion.js
```

### Vector Operations

Demonstrates vector database operations (upsert, search, delete):

```bash
node vector-operations.js
```

## Using with Local EdgeFlow Instance

If you're running a EdgeFlow instance locally, set:

```bash
export EDGEFLOW_API_URL="http://localhost:8000"
```

## Troubleshooting

If you encounter errors:

1. Verify that your EdgeFlow instance is running and accessible
2. Check that your API key is correct
3. Ensure that the requested model is available in your EdgeFlow configuration
4. Check the EdgeFlow logs for more detailed error information 