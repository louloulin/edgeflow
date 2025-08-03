// Basic completion example
const { EdgeFlowClient } = require('edgeflow-sdk');

// Initialize the client
const client = new EdgeFlowClient({
  baseUrl: process.env.EDGEFLOW_API_URL || 'http://localhost:8000',
  apiKey: process.env.EDGEFLOW_API_KEY,
  defaultProvider: 'openai',
  defaultModel: 'gpt-3.5-turbo'
});

async function main() {
  try {
    console.log('Sending completion request...');
    
    const response = await client.completion({
      messages: [
        { role: 'system', content: 'You are a helpful assistant.' },
        { role: 'user', content: 'Explain what an AI gateway is in one sentence.' }
      ],
      temperature: 0.7,
      max_tokens: 100
    });
    
    console.log('\nResponse:');
    console.log(`Model: ${response.model}`);
    console.log(`Provider: ${response.provider}`);
    console.log(`Content: ${response.message.content}`);
    console.log(`Tokens: ${response.usage.total_tokens}`);
  } catch (error) {
    console.error('Error:', error.message);
  }
}

main(); 