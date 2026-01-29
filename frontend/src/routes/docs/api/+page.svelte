<script lang="ts">
	import * as Card from '$lib/components/ui/card';
</script>

<div class="space-y-6">
	<div>
		<h1 class="text-3xl font-bold tracking-tight">API Reference</h1>
		<p class="mt-2 text-muted-foreground">Complete REST API documentation</p>
	</div>

	<Card.Root>
		<Card.Header>
			<Card.Title>Base URL</Card.Title>
		</Card.Header>
		<Card.Content>
			<code class="bg-muted px-2 py-1 rounded">http://localhost:3000/api</code>
			<p class="mt-2 text-sm text-muted-foreground">All endpoints require the <code>/api</code> prefix.</p>
		</Card.Content>
	</Card.Root>

	<Card.Root>
		<Card.Header>
			<Card.Title>Authentication</Card.Title>
		</Card.Header>
		<Card.Content class="prose prose-neutral dark:prose-invert max-w-none">
			<p>Most endpoints require authentication via Bearer token in the Authorization header:</p>
			<pre class="bg-muted p-4 rounded-lg overflow-x-auto"><code>Authorization: Bearer &lt;your-jwt-token&gt;</code></pre>

			<h3>POST /auth/register</h3>
			<p>Register a new user.</p>
			<pre class="bg-muted p-4 rounded-lg overflow-x-auto"><code>{`{
  "email": "user@example.com",
  "password": "password123",
  "name": "John Doe"
}`}</code></pre>

			<h3>POST /auth/login</h3>
			<p>Login and get JWT token.</p>
			<pre class="bg-muted p-4 rounded-lg overflow-x-auto"><code>{`{
  "email": "user@example.com",
  "password": "password123"
}`}</code></pre>
			<p><strong>Response:</strong></p>
			<pre class="bg-muted p-4 rounded-lg overflow-x-auto"><code>{`{
  "token": "eyJhbGciOiJIUzI1NiIs...",
  "user": { "id": "...", "email": "...", "name": "..." }
}`}</code></pre>

			<h3>GET /auth/me</h3>
			<p>Get current user info (requires auth).</p>
		</Card.Content>
	</Card.Root>

	<Card.Root>
		<Card.Header>
			<Card.Title>Containers</Card.Title>
		</Card.Header>
		<Card.Content class="prose prose-neutral dark:prose-invert max-w-none">
			<h3>GET /containers</h3>
			<p>List all containers. Use <code>?all=true</code> to include stopped containers.</p>

			<h3>POST /containers</h3>
			<p>Create a new container.</p>
			<pre class="bg-muted p-4 rounded-lg overflow-x-auto"><code>{`{
  "name": "my-container",
  "image": "nginx:latest",
  "env": ["KEY=value"]
}`}</code></pre>

			<h3>POST /containers/:id/start</h3>
			<p>Start a stopped container.</p>

			<h3>POST /containers/:id/stop</h3>
			<p>Stop a running container.</p>

			<h3>POST /containers/:id/restart</h3>
			<p>Restart a container.</p>

			<h3>DELETE /containers/:id</h3>
			<p>Remove a container.</p>

			<h3>GET /containers/:id/logs</h3>
			<p>Get container logs. Use <code>?tail=100</code> to limit lines.</p>

			<h3>GET /containers/:id/stats</h3>
			<p>Get container resource stats (CPU, memory, network).</p>

			<h3>GET /containers/:id/logs/stream</h3>
			<p>Stream container logs via SSE (Server-Sent Events).</p>

			<h3>GET /containers/:id/stats/stream</h3>
			<p>Stream container stats via SSE.</p>
		</Card.Content>
	</Card.Root>

	<Card.Root>
		<Card.Header>
			<Card.Title>Images</Card.Title>
		</Card.Header>
		<Card.Content class="prose prose-neutral dark:prose-invert max-w-none">
			<h3>GET /images</h3>
			<p>List all local images.</p>

			<h3>POST /images/pull</h3>
			<p>Pull an image from registry.</p>
			<pre class="bg-muted p-4 rounded-lg overflow-x-auto"><code>{`{
  "image": "nginx:latest"
}`}</code></pre>

			<h3>DELETE /images/:id</h3>
			<p>Remove an image.</p>
		</Card.Content>
	</Card.Root>

	<Card.Root>
		<Card.Header>
			<Card.Title>Stacks</Card.Title>
		</Card.Header>
		<Card.Content class="prose prose-neutral dark:prose-invert max-w-none">
			<h3>GET /stacks</h3>
			<p>List all stacks for the current user.</p>

			<h3>POST /stacks</h3>
			<p>Create a new stack from Docker Compose.</p>
			<pre class="bg-muted p-4 rounded-lg overflow-x-auto"><code>{`{
  "name": "my-stack",
  "compose_content": "version: '3.8'\\nservices:\\n  web:\\n    image: nginx:alpine"
}`}</code></pre>

			<h3>GET /stacks/:id</h3>
			<p>Get stack details.</p>

			<h3>DELETE /stacks/:id</h3>
			<p>Delete a stack and all its containers.</p>

			<h3>POST /stacks/:id/start</h3>
			<p>Start all containers in a stack.</p>

			<h3>POST /stacks/:id/stop</h3>
			<p>Stop all containers in a stack.</p>

			<h3>POST /stacks/:id/redeploy</h3>
			<p>Redeploy a stack (pull latest images and recreate containers).</p>

			<h3>PUT /stacks/:id/compose</h3>
			<p>Update compose content and redeploy.</p>

			<h3>POST /webhooks/deploy/:stack_id/:token</h3>
			<p>Trigger deployment via webhook (no auth required, token validation only).</p>
		</Card.Content>
	</Card.Root>

	<Card.Root>
		<Card.Header>
			<Card.Title>System</Card.Title>
		</Card.Header>
		<Card.Content class="prose prose-neutral dark:prose-invert max-w-none">
			<h3>GET /health</h3>
			<p>Health check endpoint (no auth required).</p>
			<pre class="bg-muted p-4 rounded-lg overflow-x-auto"><code>{`{
  "status": "ok",
  "version": "0.1.0"
}`}</code></pre>

			<h3>GET /system/stats</h3>
			<p>Get system resource stats (no auth required).</p>
			<pre class="bg-muted p-4 rounded-lg overflow-x-auto"><code>{`{
  "cpu_count": 4,
  "memory_total_kb": 8000000,
  "memory_available_kb": 4000000,
  "memory_used_percent": 50.0,
  "disk_total_bytes": 100000000000,
  "disk_available_bytes": 50000000000,
  "disk_used_percent": 50.0,
  "uptime_seconds": 86400,
  "load_average": { "one": 0.5, "five": 0.4, "fifteen": 0.3 }
}`}</code></pre>
		</Card.Content>
	</Card.Root>
</div>
