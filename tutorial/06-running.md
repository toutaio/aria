# Chapter 6: Running the Application

By the end of this chapter you will have the URL shortener running locally, accepting real HTTP requests, and shortening real URLs.

---

## 1. What Gets Wired Here

All the domain logic from chapters 3–5 is complete. What's missing is the L5 entry point that binds the HTTP layer to it. Two files are needed:

| File | What it does |
|------|--------------|
| `src/url/domain/expose.api.ts` | L5 handler — routes HTTP requests to the ARIA pipeline |
| `src/server.ts` | Node.js entry point — starts the HTTP server |

---

## 2. The L5 Handler — `src/url/domain/expose.api.ts`

This is the implementation of the `url.domain.expose.api` ARU. Its only job is to translate HTTP requests into typed domain calls and translate domain results back into HTTP responses. No business logic lives here.

```typescript
import type { IncomingMessage, ServerResponse } from "node:http";
import { orchestrateShorten } from "../pipeline/orchestrate.shorten";
import { executeShortCode } from "../store/execute.shortCode";
import type { OriginalUrl, ShortCode } from "../types";

async function readBody(req: IncomingMessage): Promise<string> {
  return new Promise((resolve, reject) => {
    let data = "";
    req.on("data", (chunk) => (data += chunk));
    req.on("end", () => resolve(data));
    req.on("error", reject);
  });
}

function json(res: ServerResponse, status: number, body: unknown) {
  const payload = JSON.stringify(body);
  res.writeHead(status, {
    "Content-Type": "application/json",
    "Content-Length": Buffer.byteLength(payload),
  });
  res.end(payload);
}

export async function handler(req: IncomingMessage, res: ServerResponse) {
  const url = req.url ?? "/";
  const method = req.method ?? "GET";

  // POST /shorten — create a short URL
  if (method === "POST" && url === "/shorten") {
    let body: { url?: string };
    try {
      body = JSON.parse(await readBody(req));
    } catch {
      return json(res, 400, { error: "Invalid JSON body" });
    }

    if (!body.url) return json(res, 400, { error: "Missing field: url" });

    const result = await orchestrateShorten(body.url as OriginalUrl);
    if (!result.ok) {
      const status = result.error.code === "DUPLICATE" ? 409 : 422;
      return json(res, status, { error: result.error.code, message: result.error.message });
    }

    return json(res, 201, {
      shortCode: result.value.shortCode,
      shortUrl: `http://localhost:3000/${result.value.shortCode}`,
      originalUrl: result.value.originalUrl,
      createdAt: result.value.createdAt,
    });
  }

  // GET /:shortCode — redirect to original URL
  const shortCodeMatch = url.match(/^\/([a-zA-Z0-9]{6})$/);
  if (method === "GET" && shortCodeMatch) {
    const shortCode = shortCodeMatch[1] as ShortCode;
    const result = await executeShortCode(shortCode);

    if (!result.ok) return json(res, 404, { error: "NOT_FOUND", message: "Short URL not found" });

    res.writeHead(302, { Location: result.value.originalUrl });
    return res.end();
  }

  // GET / — health check
  if (method === "GET" && url === "/") {
    return json(res, 200, { status: "ok", service: "url-shortener-aria" });
  }

  return json(res, 404, { error: "NOT_FOUND" });
}
```

Notice what is **not** here: no validation logic, no hash generation, no store calls. The handler calls `orchestrateShorten` (L4) and `executeShortCode` (L3) — both of which are typed, tested ARUs. The L5 handler is just translation.

---

## 3. The Server Entry Point — `src/server.ts`

```typescript
import { createServer } from "node:http";
import { handler } from "./url/domain/expose.api";

const PORT = Number(process.env.PORT ?? 3000);

const server = createServer(handler);

server.listen(PORT, () => {
  console.log(`\n🔗 url-shortener-aria running on http://localhost:${PORT}`);
  console.log(`   POST /shorten        — create a short URL`);
  console.log(`   GET  /:shortCode     — redirect to original URL`);
  console.log(`   GET  /               — health check\n`);
});
```

No framework dependency. Node's built-in `http` module is enough for a tutorial-scale service.

---

## 4. Start the Server

```bash
npm start
```

```
🔗 url-shortener-aria running on http://localhost:3000
   POST /shorten        — create a short URL
   GET  /:shortCode     — redirect to original URL
   GET  /               — health check
```

For development with auto-restart on file changes:

```bash
npm run dev
```

---

## 5. Try It

Open a second terminal and run these commands while the server is running.

### Health check

```bash
curl http://localhost:3000/
```

```json
{ "status": "ok", "service": "url-shortener-aria" }
```

### Shorten a URL

```bash
curl -X POST http://localhost:3000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://github.com/features/copilot"}'
```

```json
{
  "shortCode": "VhtAeA",
  "shortUrl": "http://localhost:3000/VhtAeA",
  "originalUrl": "https://github.com/features/copilot",
  "createdAt": "2026-03-15T13:16:02.961Z"
}
```

### Follow the redirect

```bash
curl -i http://localhost:3000/VhtAeA
```

```
HTTP/1.1 302 Found
Location: https://github.com/features/copilot
```

### Duplicate URL (409)

```bash
curl -X POST http://localhost:3000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://github.com/features/copilot"}'
```

```json
{ "error": "DUPLICATE", "message": "Short code VhtAeA already exists" }
```

### Unknown short code (404)

```bash
curl http://localhost:3000/xxxxxx
```

```json
{ "error": "NOT_FOUND", "message": "Short URL not found" }
```

---

## 6. Available Scripts

| Command | What it does |
|---------|--------------|
| `npm start` | Start the server (port 3000) |
| `npm run dev` | Start with file-watch auto-restart |
| `npm test` | Run all 22 tests |
| `npm run test:watch` | Run tests in watch mode |
| `npm run bundle:dev` | Build the semantic graph bundle |
| `npm run promote` | Interactively promote manifests to STABLE |
| `aria-build check ./src` | Validate all manifests |
| `aria-build impact <id>` | Show dependents of an ARU |

---

## 7. What's in Memory

The store is in-memory — all shortened URLs are lost when the server restarts. This is intentional for the tutorial: it keeps the setup to zero dependencies. In a real deployment you would replace `src/url/store/in-memory.store.ts` with a Redis or PostgreSQL adapter, and only the two L3 organisms (`persist.link` and `execute.shortCode`) need to change. Every other ARU is unaffected — which is exactly the point of the layered architecture.

---

**[← AI Collaboration](05-project-ai-collab.md)** | **[Conclusion →](07-conclusion.md)**
