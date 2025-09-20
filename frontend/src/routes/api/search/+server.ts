// src/routes/api/data/+server.js
import { json } from '@sveltejs/kit';

export async function GET() {
	// Forward the request to your Rust backend
	// const rustApiUrl = 'http://localhost:8080/data';
	// const response = await fetch(rustApiUrl);

	// if (!response.ok) {
	// 	return new Response(null, {
	// 		status: response.status,
	// 		statusText: response.statusText
	// 	});
	// }

	// const data = await response.json();

	// Return the data from the SvelteKit API route
	return json({ results: ['a'] });
}
