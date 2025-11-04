# Wadah - Cross-Project Documentation

**Comprehensive guide for working with the Wadah ecosystem**

---

## 📦 Repository Structure

Wadah is split into two main repositories:

### 1. **wadah-engine** (Backend/CLI)
**Repository:** https://github.com/devwadahai/wadah-engine  
**Language:** Rust  
**Purpose:** Core runtime, CLI, and agent execution

```
wadah-engine/
├── crates/
│   ├── cli/         # Command-line interface
│   ├── spec/        # WadahSpec implementation
│   ├── pack/        # .wpkg packaging
│   ├── runtime/     # Agent execution engine
│   ├── trace/       # OpenAgentTrace (OAT)
│   └── oci/         # Registry client
├── templates/       # Pre-built agent templates
├── docs/            # Documentation
└── scripts/         # Build and test scripts
```

### 2. **wadah-ui** (Frontend/Dashboard)
**Repository:** https://github.com/devwadahai/wadah-ui  
**Language:** TypeScript (React)  
**Purpose:** Web-based visual interface

```
wadah-ui/
├── client/
│   ├── src/
│   │   ├── pages/       # Dashboard, Agents, Runs, etc.
│   │   ├── components/  # React components
│   │   └── lib/         # Utilities
│   └── public/          # Static assets
├── server/              # Express backend
├── shared/              # Shared types
└── docs/                # UI documentation
```

---

## 🔄 How They Work Together

```
┌─────────────────────────────────────────────────────┐
│                    User                              │
│                      ↓                               │
├─────────────────────────────────────────────────────┤
│  Interface Layer                                     │
│  ┌──────────────┐          ┌──────────────┐        │
│  │  Wadah CLI   │          │  Wadah UI    │        │
│  │  (Terminal)  │          │  (Browser)   │        │
│  └──────┬───────┘          └──────┬───────┘        │
│         │                         │                 │
│         └─────────────┬───────────┘                 │
│                       ↓                              │
├─────────────────────────────────────────────────────┤
│  Core Engine (Rust)                                  │
│  ┌─────────────────────────────────────────────┐   │
│  │  Wadah Runtime                               │   │
│  │  • Agent execution                           │   │
│  │  • Model adapters (OpenAI, Ollama, TGI)     │   │
│  │  • Policy enforcement                        │   │
│  │  • Tracing & observability                  │   │
│  └─────────────────────────────────────────────┘   │
│                       ↓                              │
├─────────────────────────────────────────────────────┤
│  Infrastructure                                      │
│  • Docker/Podman                                     │
│  • Kubernetes                                        │
│  • OCI Registries (GHCR, Docker Hub)                │
│  • Databases (PostgreSQL, Qdrant)                   │
└─────────────────────────────────────────────────────┘
```

---

## 🚀 Getting Started

### Prerequisites

**For CLI (wadah-engine):**
- Rust 1.75+ (`rustup`)
- Cargo
- (Optional) Docker for containerization

**For UI (wadah-ui):**
- Node.js 20+
- npm or pnpm
- PostgreSQL (for database)

### Setup Both Projects

```bash
# Clone repositories
cd ~/Projects
git clone https://github.com/devwadahai/wadah-engine.git
git clone https://github.com/devwadahai/wadah-ui.git

# Build engine
cd wadah-engine
cargo build --release
cargo install --path crates/cli

# Set up UI
cd ../wadah-ui
npm install
cp .env.example .env.local
# Edit .env.local with your settings
npm run dev
```

---

## 🔗 Integration Points

### 1. **CLI → UI Communication**

The UI calls the CLI via:

**Option A: Direct Process Execution**
```typescript
// client/src/lib/cli.ts
import { exec } from 'child_process';

export async function wadahInit(name: string, security: string) {
  const { stdout } = await exec(`wadah init ${name} --security ${security}`);
  return stdout;
}
```

**Option B: HTTP API Wrapper**
```typescript
// server/routes.ts
app.post('/api/agents/init', async (req, res) => {
  const { name, security } = req.body;
  
  // Call wadah CLI
  const result = await execWadah('init', [name, '--security', security]);
  
  res.json({ success: true, result });
});
```

### 2. **Shared Data Formats**

Both projects use the same data structures:

**WadahSpec (wadah.yaml):**
```yaml
apiVersion: wadah.ai/v0.1
kind: Agent
metadata:
  name: my-agent
runtime:
  model:
    provider: openai
    model_id: gpt-4o
```

**Trace Format (OAT):**
```json
{
  "trace_id": "abc123",
  "spans": [
    {
      "span_id": "xyz789",
      "operation": "model.generate",
      "tokens": 150,
      "cost": 0.003
    }
  ]
}
```

### 3. **File System Integration**

```
~/wadah-workspace/
├── agents/              # Created by CLI or UI
│   ├── my-rag/
│   │   ├── wadah.yaml
│   │   ├── prompts/
│   │   └── data/
│   └── my-bot/
│       └── wadah.yaml
├── packages/            # .wpkg files
│   ├── my-rag-v0.1.0.wpkg
│   └── my-bot-v0.2.0.wpkg
└── traces/              # Execution traces
    ├── run-abc123.jsonl
    └── run-xyz789.jsonl
```

---

## 🛠️ Development Workflow

### Typical Development Flow

1. **Build Agent in UI**
   - Open browser → http://localhost:5000
   - Use visual builder to create agent
   - UI generates `wadah.yaml`

2. **Test Locally with CLI**
   ```bash
   cd agents/my-agent
   wadah run wadah.yaml --prompt "Test"
   ```

3. **Monitor in UI**
   - UI shows real-time logs
   - View traces and metrics
   - Debug issues visually

4. **Package and Deploy**
   ```bash
   # Via CLI
   wadah pack -o my-agent.wpkg
   wadah push ghcr.io/org/my-agent:v1
   
   # Or via UI
   # Click "Deploy" button
   ```

### Working on Both Projects

**Terminal 1 (Engine):**
```bash
cd ~/Projects/wadah-engine

# Watch for changes and rebuild
cargo watch -x 'build --release -p wadah-cli'
```

**Terminal 2 (UI):**
```bash
cd ~/Projects/wadah-ui

# Start dev server
npm run dev

# Runs on http://localhost:5000
```

**Terminal 3 (Testing):**
```bash
# Test CLI
wadah --version

# Test UI API
curl http://localhost:5000/api/agents
```

---

## 📝 Common Tasks

### Task 1: Add New CLI Command

1. **In wadah-engine:**
   ```rust
   // crates/cli/src/commands/my_command.rs
   pub async fn execute(/* args */) -> Result<()> {
       // Implementation
   }
   ```

2. **In wadah-ui:**
   ```typescript
   // client/src/lib/cli.ts
   export async function wadahMyCommand(/* args */) {
       return execWadah('my-command', [/* args */]);
   }
   ```

### Task 2: Add New UI Page

1. **Create page:**
   ```typescript
   // client/src/pages/MyPage.tsx
   export default function MyPage() {
       return <div>My Page</div>;
   }
   ```

2. **Add route:**
   ```typescript
   // client/src/App.tsx
   <Route path="/my-page" component={MyPage} />
   ```

3. **Call backend:**
   ```typescript
   // Uses wadah-engine CLI or API
   const data = await fetch('/api/my-endpoint');
   ```

### Task 3: Add New Template

1. **In wadah-engine:**
   ```bash
   cd templates
   mkdir my-template
   # Add wadah.yaml, prompts/, etc.
   ```

2. **In wadah-ui:**
   ```typescript
   // UI automatically discovers templates
   // via wadah CLI: `wadah templates list`
   ```

---

## 🔍 Debugging

### Debug CLI Issues

```bash
# Enable debug logging
export RUST_LOG=debug
wadah run wadah.yaml

# Check build
cargo build --verbose

# Run tests
cargo test --workspace
```

### Debug UI Issues

```bash
# Check server logs
npm run dev
# Server logs appear in terminal

# Check browser console
# Open DevTools → Console

# Test API directly
curl -X POST http://localhost:5000/api/agents/init \
  -H "Content-Type: application/json" \
  -d '{"name": "test", "security": "minimal"}'
```

### Debug Integration

```bash
# Verify CLI is accessible from UI
which wadah
# Should output: /Users/hsp/.cargo/bin/wadah

# Test CLI from UI directory
cd ~/Projects/wadah-ui
wadah --version
# Should work!

# Check environment variables
cat .env.local
# Should have: WADAH_CLI_PATH=/path/to/wadah
```

---

## 📊 Architecture Decisions

### Why Two Repos?

**Pros:**
- ✅ Different tech stacks (Rust vs TypeScript)
- ✅ Independent deployment
- ✅ Clearer separation of concerns
- ✅ Easier contribution (backend vs frontend devs)
- ✅ Different release cycles

**Cons:**
- ❌ More complex setup
- ❌ Need to keep in sync
- ❌ Separate CI/CD pipelines

**Alternative Considered:** Monorepo with Nx/Turborepo
- Decided against due to very different toolchains

### Communication Strategy

**Current:** CLI as Backend
- UI spawns CLI processes
- Simple, no API versioning issues
- CLI is the source of truth

**Future:** HTTP API Server
- wadahd HTTP server (optional)
- REST/WebSocket API
- Better for multi-user scenarios

---

## 🚢 Deployment

### Local Development

```bash
# Engine only
wadah run wadah.yaml

# UI only (connects to local CLI)
cd wadah-ui && npm run dev

# Both together
# Terminal 1: wadah server (future)
# Terminal 2: npm run dev
```

### Production

**Option 1: CLI Only**
```bash
# Install CLI globally
cargo install wadah-cli

# Use from command line
wadah run production.wpkg
```

**Option 2: UI + CLI**
```bash
# Deploy UI to Vercel/Netlify
npm run build

# UI calls CLI on server
# Set WADAH_CLI_PATH in environment
```

**Option 3: Docker**
```dockerfile
FROM rust:1.75 as builder
WORKDIR /build
COPY wadah-engine/ .
RUN cargo build --release

FROM node:20 as ui-builder
WORKDIR /build
COPY wadah-ui/ .
RUN npm install && npm run build

FROM debian:bookworm-slim
COPY --from=builder /build/target/release/wadah /usr/local/bin/
COPY --from=ui-builder /build/dist /app
CMD ["node", "/app/server.js"]
```

---

## 📚 Documentation Links

### wadah-engine Docs
- [README](https://github.com/devwadahai/wadah-engine/blob/main/README.md)
- [Quickstart](https://github.com/devwadahai/wadah-engine/blob/main/docs/Quickstart.md)
- [WadahSpec](https://github.com/devwadahai/wadah-engine/blob/main/docs/WadahSpec-v0.1.md)
- [Deployment](https://github.com/devwadahai/wadah-engine/blob/main/docs/Deployment.md)

### wadah-ui Docs
- [README](https://github.com/devwadahai/wadah-ui/blob/main/README.md) (to be created)
- [UI Spec](https://github.com/devwadahai/wadah-engine/blob/main/docs/UI-SPEC.md)
- [Component Library](https://github.com/devwadahai/wadah-ui/docs/components.md) (to be created)

---

## 🤝 Contributing

### To wadah-engine (Rust)
1. Fork the repo
2. Create feature branch
3. Make changes
4. Run tests: `cargo test --workspace`
5. Format: `cargo fmt --all`
6. Lint: `cargo clippy`
7. Submit PR

### To wadah-ui (TypeScript)
1. Fork the repo
2. Create feature branch
3. Make changes
4. Run tests: `npm test`
5. Format: `npm run format`
6. Lint: `npm run lint`
7. Submit PR

### Cross-repo Changes

If your change affects both repos:
1. Create PR in wadah-engine first
2. Reference engine PR in UI PR
3. Coordinate review/merge

---

## 🔐 Security

### API Keys
- **Engine:** Set via environment variables
- **UI:** Stored in database, encrypted
- **Never commit:** Use `.env` / `.env.local`

### Authentication
- **Engine:** None (local CLI)
- **UI:** User auth (Passport.js)
- **Future:** OAuth, SSO

---

## 🎯 Roadmap

### v0.2.0 (Q4 2025)
- ✅ wadah-engine: v0.1.0 complete
- 🚧 wadah-ui: Initial MVP
- 🚧 HTTP API server (optional)

### v0.3.0 (Q1 2026)
- Team collaboration features
- Advanced UI features
- Kubernetes operator

### v1.0.0 (Q2 2026)
- Production-ready both repos
- Enterprise features
- Full integration

---

## 💡 Tips

1. **Keep versions in sync:** Use same version number for related releases
2. **Test integration:** Always test UI with latest CLI build
3. **Document changes:** Update both READMEs when adding features
4. **Use branches:** `dev/feature-name` in both repos
5. **CI/CD:** Set up GitHub Actions in both repos

---

## 🆘 Getting Help

- **Engine Issues:** https://github.com/devwadahai/wadah-engine/issues
- **UI Issues:** https://github.com/devwadahai/wadah-ui/issues
- **Discussions:** https://github.com/devwadahai/wadah-engine/discussions
- **Email:** dev@wadah.ai (if available)

---

**Last Updated:** November 4, 2025  
**Engine Version:** v0.1.0  
**UI Version:** v0.2.0-alpha  
**Status:** Active Development

*"Two repos, one vision: making AI agents accessible to everyone."* 🌊

