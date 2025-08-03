# EdgeFlow SDK for JavaScript/TypeScript

JavaScript/TypeScript client for the EdgeFlow AI Gateway. This SDK provides a simple way to interact with the EdgeFlow API for LLM completions and vector operations.

## Installation

```bash
npm install edgeflow-sdk
```

## Usage

### Basic Completion Request

```typescript
import { EdgeFlowClient } from 'edgeflow-sdk';

// Initialize the client
const client = new EdgeFlowClient({
  baseUrl: 'https://your-edgeflow-instance.com',
  apiKey: 'your-api-key',
  defaultProvider: 'openai',
  defaultModel: 'gpt-4'
});

// Make a completion request
async function getCompletion() {
  const response = await client.completion({
    messages: [
      { role: 'system', content: 'You are a helpful assistant.' },
      { role: 'user', content: 'What is the capital of France?' }
    ]
  });
  
  console.log(response.message.content);
}

getCompletion();
```

### Streaming Completion

```typescript
import { EdgeFlowClient } from 'edgeflow-sdk';

const client = new EdgeFlowClient({
  baseUrl: 'https://your-edgeflow-instance.com',
  apiKey: 'your-api-key'
});

// Stream a completion
client.streamCompletion(
  {
    provider: 'anthropic',
    model: 'claude-3-opus',
    messages: [
      { role: 'user', content: 'Write a short poem about AI' }
    ]
  },
  (chunk) => {
    // Process each chunk as it arrives
    if (chunk.delta.content) {
      process.stdout.write(chunk.delta.content);
    }
  },
  (error) => console.error('Error:', error),
  () => console.log('\nStream completed')
);
```

### Vector Database Operations

```typescript
import { EdgeFlowClient } from 'edgeflow-sdk';

const client = new EdgeFlowClient({
  baseUrl: 'https://your-edgeflow-instance.com',
  apiKey: 'your-api-key'
});

// Insert vectors
async function storeVectors() {
  await client.upsertVectors({
    namespace: 'products',
    vectors: [
      {
        id: 'product-1',
        values: [0.1, 0.2, 0.3, 0.4],
        metadata: { name: 'Product 1', category: 'electronics' }
      },
      {
        id: 'product-2',
        values: [0.2, 0.3, 0.4, 0.5],
        metadata: { name: 'Product 2', category: 'clothing' }
      }
    ]
  });
}

// Search vectors
async function searchVectors() {
  const results = await client.searchVectors({
    namespace: 'products',
    query_vector: [0.15, 0.25, 0.35, 0.45],
    top_k: 5,
    filter: { category: 'electronics' }
  });
  
  console.log(results);
}
```

## Command Line Interface (CLI)

The SDK includes a powerful CLI that allows you to interact with the EdgeFlow AI Gateway directly from your terminal.

### Installation

```bash
# Install globally
npm install -g edgeflow-sdk

# Or use with npx
npx edgeflow-sdk --help
```

### Configuration

```bash
# Set environment variables
export EDGEFLOW_API_URL="https://your-edgeflow-instance.com"
export EDGEFLOW_API_KEY="your-api-key"
```

### Example Commands

```bash
# Get help
edgeflow --help

# Send a completion request
edgeflow completion "What is the capital of France?"

# Stream a response
edgeflow stream "Write a poem about AI"

# Interactive chat
edgeflow chat

# Vector operations
edgeflow vector:upsert --namespace products --id product-123 0.1 0.2 0.3 0.4
edgeflow vector:search --namespace products 0.1 0.2 0.3 0.4
edgeflow vector:delete --namespace products product-123
```

For more detailed CLI examples, see [CLI Examples](./examples/cli-examples.md).

## API Reference

### EdgeFlowClient

The main client class for interacting with the EdgeFlow API.

#### Constructor

```typescript
new EdgeFlowClient(config: EdgeFlowClientConfig)
```

Configuration options:

- `baseUrl` (required): URL of your EdgeFlow instance
- `apiKey`: API key for authentication
- `defaultProvider`: Default LLM provider to use
- `defaultModel`: Default model to use
- `timeout`: Request timeout in milliseconds (default: 30000)
- `headers`: Additional headers to include in requests

#### Methods

- `completion(request: CompletionRequest): Promise<CompletionResponse>`
- `streamCompletion(request: CompletionRequest, onChunk, onError?, onDone?): Promise<void>`
- `upsertVectors(request: VectorUpsertRequest): Promise<void>`
- `searchVectors(request: VectorSearchRequest): Promise<VectorSearchResponse>`
- `deleteVectors(request: VectorDeleteRequest): Promise<void>`

## License

Apache 2.0 