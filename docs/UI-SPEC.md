# Wadah UI/Frontend Specification v1.0

**Target Release:** v0.2.0  
**Last Updated:** November 1, 2025  
**Status:** Design Specification

---

## 🎨 Executive Summary

A modern, web-based UI for Wadah that makes AI agent management accessible to everyone - from developers to business users. Built with React/Next.js, the Wadah UI provides visual tools for creating, managing, deploying, and monitoring AI agents without requiring CLI expertise.

**Key Principles:**
- 🎯 **Simplicity First** - Complex operations made simple
- 🚀 **Speed** - Fast, responsive, minimal loading
- 🎨 **Beautiful** - Modern, clean design language
- ♿ **Accessible** - WCAG 2.1 AA compliance
- 📱 **Responsive** - Desktop, tablet, mobile

---

## 📋 Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [User Personas](#user-personas)
3. [Core Features](#core-features)
4. [Page Specifications](#page-specifications)
5. [Component Library](#component-library)
6. [Design System](#design-system)
7. [API Integration](#api-integration)
8. [State Management](#state-management)
9. [Security & Auth](#security--auth)
10. [Performance](#performance)
11. [Deployment](#deployment)
12. [Roadmap](#roadmap)

---

## 🏗️ Architecture Overview

### Tech Stack

```yaml
Frontend Framework: Next.js 14 (App Router)
UI Library: React 18
Styling: Tailwind CSS + shadcn/ui
State Management: Zustand + React Query
Real-time: WebSocket (Socket.io)
Charting: Recharts / Tremor
Code Editor: Monaco Editor
Terminal: Xterm.js
Build Tool: Turbopack
Testing: Vitest + Playwright
Type Safety: TypeScript 5.3+
```

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                    Wadah UI (Browser)                    │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │   Dashboard  │  │   Templates  │  │     Runs     │  │
│  │   (Home)     │  │   (Library)  │  │  (Monitor)   │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
│                                                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │    Editor    │  │   Settings   │  │   Registry   │  │
│  │  (Builder)   │  │   (Config)   │  │  (Packages)  │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
│                                                           │
├─────────────────────────────────────────────────────────┤
│              Next.js API Routes + tRPC                   │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ Wadah CLI    │  │   Docker     │  │   Database   │  │
│  │  (Rust)      │  │  (Runtime)   │  │  (PostgreSQL)│  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
│                                                           │
└─────────────────────────────────────────────────────────┘
```

### System Integration

```typescript
// Wadah UI communicates with:
interface WadahIntegrations {
  cli: {
    method: 'child_process' | 'http_api';
    endpoint: 'http://localhost:3030/api';
    commands: ['init', 'pack', 'run', 'trace', 'push', 'pull'];
  };
  
  docker: {
    method: 'dockerode';
    socket: '/var/run/docker.sock';
    operations: ['build', 'run', 'stop', 'logs'];
  };
  
  database: {
    type: 'postgresql';
    orm: 'prisma';
    stores: ['agents', 'runs', 'traces', 'users', 'teams'];
  };
  
  registry: {
    protocol: 'oci';
    endpoints: ['ghcr.io', 'docker.io', 'custom'];
    auth: 'oauth2' | 'token';
  };
}
```

---

## 👥 User Personas

### 1. **Developer Dave** 🧑‍💻
- **Goal:** Build and deploy AI agents quickly
- **Needs:** Code editor, debugging, CLI integration
- **Pain Points:** Complex YAML configs, debugging traces
- **Preferred Flow:** Code → Build → Test → Deploy

### 2. **Manager Maya** 👩‍💼
- **Goal:** Oversee agent performance and costs
- **Needs:** Dashboards, analytics, budget monitoring
- **Pain Points:** No visibility into agent behavior
- **Preferred Flow:** View → Analyze → Optimize → Report

### 3. **Ops Oliver** 👨‍🔧
- **Goal:** Deploy and monitor agents in production
- **Needs:** Deployment tools, logs, alerts
- **Pain Points:** Manual deployments, no monitoring
- **Preferred Flow:** Deploy → Monitor → Scale → Alert

### 4. **Business Brenda** 👩‍💼
- **Goal:** Use pre-built agents without coding
- **Needs:** Templates, simple config, drag-and-drop
- **Pain Points:** Too technical, no GUI
- **Preferred Flow:** Browse → Configure → Deploy → Use

---

## 🎯 Core Features

### 1. **Agent Builder** ⚙️
Visual editor for creating AI agents without writing YAML.

**Features:**
- Drag-and-drop interface
- Live preview
- Template gallery
- YAML editor (Monaco)
- Validation with inline errors
- One-click deployment

**Priority:** P0 (v0.2.0)

### 2. **Dashboard** 📊
Overview of all agents, runs, and system health.

**Features:**
- Active agents list
- Recent runs timeline
- Cost tracking
- Performance metrics
- Quick actions
- Real-time updates

**Priority:** P0 (v0.2.0)

### 3. **Run Monitor** 🔍
Real-time monitoring of agent execution with traces.

**Features:**
- Live logs (WebSocket)
- Trace visualization
- Token usage tracking
- Cost per run
- Replay functionality
- Export traces (JSON/CSV)

**Priority:** P0 (v0.2.0)

### 4. **Template Library** 📚
Browse, preview, and clone agent templates.

**Features:**
- Template cards with previews
- Search and filters
- README previews
- One-click clone
- Community templates
- Template ratings

**Priority:** P1 (v0.2.0)

### 5. **Registry Manager** 📦
Push, pull, and manage agent packages.

**Features:**
- Package browser
- Version history
- Push/pull interface
- Registry configuration
- Access control
- Package details

**Priority:** P1 (v0.3.0)

### 6. **Settings & Config** ⚙️
System-wide configuration and preferences.

**Features:**
- API keys management
- Model providers config
- Security policies
- Team management
- Billing & usage
- Integrations

**Priority:** P1 (v0.2.0)

---

## 📄 Page Specifications

### 🏠 Dashboard (`/`)

**Purpose:** Central hub for all Wadah activities

**Layout:**
```
┌─────────────────────────────────────────────────────────┐
│ Header: Logo | Search | Create Agent | Notifications   │
├─────────────────────────────────────────────────────────┤
│ Sidebar:                                                 │
│  - Dashboard (active)                                    │
│  - Agents                                                │
│  - Templates                                             │
│  - Runs                                                  │
│  - Registry                                              │
│  - Settings                                              │
├──────────────┬──────────────────────────────────────────┤
│              │  Welcome back, Dave! 👋                  │
│              │                                           │
│              │  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐│
│              │  │ 12   │  │ 3    │  │ $45  │  │ 99%  ││
│              │  │Agents│  │Active│  │Today │  │Up    ││
│              │  └──────┘  └──────┘  └──────┘  └──────┘│
│              │                                           │
│              │  Recent Runs                             │
│              │  ┌────────────────────────────────────┐ │
│              │  │ RAG Service   ✓ Success   2m ago  │ │
│              │  │ DevOps Bot    ⚡ Running   now    │ │
│              │  │ Support Agent ✓ Success   5m ago  │ │
│              │  └────────────────────────────────────┘ │
│              │                                           │
│              │  Quick Actions                           │
│              │  [New Agent] [Browse Templates]         │
│              │                                           │
└──────────────┴──────────────────────────────────────────┘
```

**Components:**
- `<StatsCard />` - Metric cards (agents, runs, cost, uptime)
- `<RecentRunsList />` - Timeline of recent executions
- `<QuickActions />` - Common action buttons
- `<SystemHealth />` - Docker, CLI, database status

**Data:**
```typescript
interface DashboardData {
  stats: {
    totalAgents: number;
    activeRuns: number;
    costToday: number;
    systemUptime: number;
  };
  
  recentRuns: Run[];
  activeAgents: Agent[];
  systemHealth: HealthStatus;
}
```

---

### 🤖 Agents Page (`/agents`)

**Purpose:** List and manage all AI agents

**Layout:**
```
┌─────────────────────────────────────────────────────────┐
│ My Agents                                [+ New Agent]  │
├─────────────────────────────────────────────────────────┤
│ 🔍 Search agents...  [Filter ▾] [Sort ▾] [View: Grid]  │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐    │
│  │ 🤖          │  │ 🔧          │  │ 💬          │    │
│  │ RAG Service │  │ DevOps Bot  │  │ Support AI  │    │
│  │ v0.1.0      │  │ v0.2.1      │  │ v1.0.0      │    │
│  │ ✓ Running   │  │ ⚡ Active    │  │ ⏸ Paused    │    │
│  │ 125 runs    │  │ 89 runs     │  │ 1.2k runs   │    │
│  └─────────────┘  └─────────────┘  └─────────────┘    │
│                                                           │
│  [Load More]                                             │
│                                                           │
└─────────────────────────────────────────────────────────┘
```

**Features:**
- **Search:** Full-text search by name, tags, description
- **Filters:** Status, security level, model, created date
- **Sort:** Name, created, runs, cost
- **Views:** Grid, list, table
- **Actions:** Edit, run, clone, delete, export

**Agent Card:**
```typescript
interface AgentCard {
  id: string;
  name: string;
  version: string;
  description: string;
  icon: string;
  status: 'running' | 'stopped' | 'error';
  stats: {
    totalRuns: number;
    lastRun: Date;
    avgCost: number;
  };
  tags: string[];
  security: 'minimal' | 'standard' | 'strict';
}
```

---

### ⚙️ Agent Builder (`/agents/new` or `/agents/:id/edit`)

**Purpose:** Visual editor for creating/editing agents

**Layout:**
```
┌─────────────────────────────────────────────────────────┐
│ Create New Agent                    [Save] [Deploy]     │
├──────────────┬──────────────────────────────────────────┤
│ Steps:       │                                           │
│ 1. Basic ✓   │  Basic Information                       │
│ 2. Model ←   │  ┌────────────────────────────────────┐ │
│ 3. Tools     │  │ Agent Name: my-rag-service         │ │
│ 4. Security  │  │ Description: Document Q&A agent    │ │
│ 5. Review    │  │ Version: 0.1.0                     │ │
│              │  │ Tags: [rag] [production]           │ │
│              │  └────────────────────────────────────┘ │
│              │                                           │
│              │  Model Configuration                     │
│              │  ┌────────────────────────────────────┐ │
│              │  │ Provider: [OpenAI ▾]               │ │
│              │  │ Model: [gpt-4o ▾]                  │ │
│              │  │ Temperature: [0.7] ━━━●━━━         │ │
│              │  │ Max Tokens: [2048]                 │ │
│              │  └────────────────────────────────────┘ │
│              │                                           │
│              │  [Previous] [Next: Tools →]              │
│              │                                           │
├──────────────┴──────────────────────────────────────────┤
│ Preview: wadah.yaml                                      │
│ ┌─────────────────────────────────────────────────────┐ │
│ │ apiVersion: wadah.ai/v0.1                           │ │
│ │ kind: Agent                                         │ │
│ │ metadata:                                           │ │
│ │   name: my-rag-service                              │ │
│ └─────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

**Steps:**

1. **Basic Info** - Name, description, version, tags
2. **Model** - Provider, model, parameters
3. **Tools** - Add tools from library or custom
4. **Security** - Security level, budgets, policies
5. **Review** - Preview YAML, validate, deploy

**Components:**
- `<StepWizard />` - Multi-step form
- `<ModelSelector />` - Provider and model picker
- `<ToolConfigurator />` - Tool management
- `<SecurityConfigurator />` - Policy builder
- `<YAMLPreview />` - Live YAML editor

**Modes:**
- **Visual Mode** - Form-based GUI (default)
- **YAML Mode** - Monaco editor with autocomplete
- **Split Mode** - Both side-by-side

---

### 🔍 Run Monitor (`/runs/:id`)

**Purpose:** Real-time monitoring and debugging of agent runs

**Layout:**
```
┌─────────────────────────────────────────────────────────┐
│ ← Back to Runs                                          │
│                                                           │
│ RAG Service #12345                    [Stop] [Replay]   │
│ ⚡ Running • Started 2m ago • $0.15                      │
├─────────────────────────────────────────────────────────┤
│                                                           │
│ ┌────────────┬────────────┬────────────┬────────────┐  │
│ │ Overview   │ Logs       │ Trace      │ Metrics    │  │
│ └────────────┴────────────┴────────────┴────────────┘  │
│                                                           │
│ Real-time Logs                          [Clear] [Export]│
│ ┌─────────────────────────────────────────────────────┐ │
│ │ [10:23:45] Starting agent execution...              │ │
│ │ [10:23:46] Loading model: gpt-4o                    │ │
│ │ [10:23:47] ✓ Model loaded successfully              │ │
│ │ [10:23:48] Processing prompt...                     │ │
│ │ [10:23:50] ⚡ Generating response (512 tokens)       │ │
│ │ [10:23:52] ✓ Response generated successfully        │ │
│ │ [10:23:52] Total cost: $0.15 • Tokens: 1,234       │ │
│ │ █                                                    │ │
│ └─────────────────────────────────────────────────────┘ │
│                                                           │
│ Performance                                              │
│ ┌────────────┬────────────┬────────────┬────────────┐  │
│ │ Latency    │ Tokens     │ Cost       │ Memory     │  │
│ │ 2.5s       │ 1,234      │ $0.15      │ 125 MB     │  │
│ └────────────┴────────────┴────────────┴────────────┘  │
│                                                           │
└─────────────────────────────────────────────────────────┘
```

**Tabs:**

1. **Overview** - Summary, status, quick stats
2. **Logs** - Real-time streaming logs
3. **Trace** - OpenAgentTrace visualization
4. **Metrics** - Performance charts

**Real-time Updates:**
```typescript
// WebSocket connection for live updates
interface RunUpdate {
  type: 'log' | 'metric' | 'status' | 'trace';
  timestamp: Date;
  data: {
    level: 'info' | 'warn' | 'error';
    message: string;
    metadata?: Record<string, any>;
  };
}

// Connect to WebSocket
const ws = new WebSocket(`ws://localhost:3030/runs/${runId}`);
ws.onmessage = (event) => {
  const update: RunUpdate = JSON.parse(event.data);
  // Update UI in real-time
};
```

**Features:**
- Live log streaming (auto-scroll)
- Syntax highlighting
- Search logs
- Filter by level (info/warn/error)
- Export logs (JSON/TXT)
- Replay trace deterministically

---

### 📚 Templates Page (`/templates`)

**Purpose:** Browse and use pre-built agent templates

**Layout:**
```
┌─────────────────────────────────────────────────────────┐
│ Agent Templates                         [Upload Custom] │
├─────────────────────────────────────────────────────────┤
│ 🔍 Search templates...  [Category ▾] [Security ▾]      │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  Categories: [All] [RAG] [DevOps] [Support] [Custom]   │
│                                                           │
│  ┌──────────────────┐  ┌──────────────────┐            │
│  │ 👋 Hello World   │  │ 🔗 LangChain RAG │            │
│  │ by Wadah Team    │  │ by Wadah Team    │            │
│  │ ⭐⭐⭐⭐⭐ (150)  │  │ ⭐⭐⭐⭐⭐ (89)   │            │
│  │                  │  │                  │            │
│  │ Quick start      │  │ Production RAG   │            │
│  │ template for     │  │ with full stack  │            │
│  │ learning         │  │ integration      │            │
│  │                  │  │                  │            │
│  │ [Use Template]   │  │ [Use Template]   │            │
│  └──────────────────┘  └──────────────────┘            │
│                                                           │
│  ┌──────────────────┐  ┌──────────────────┐            │
│  │ 🔧 DevOps Bot    │  │ 💬 Support Agent │            │
│  │ by Wadah Team    │  │ by Wadah Team    │            │
│  │ ⭐⭐⭐⭐☆ (67)    │  │ ⭐⭐⭐⭐⭐ (92)   │            │
│  │ [Use Template]   │  │ [Use Template]   │            │
│  └──────────────────┘  └──────────────────┘            │
│                                                           │
└─────────────────────────────────────────────────────────┘
```

**Features:**
- Template cards with preview
- Star ratings and downloads
- README preview (Markdown)
- One-click "Use Template"
- Filter by category, security, complexity
- Community templates (future)

**Template Card Data:**
```typescript
interface Template {
  id: string;
  name: string;
  description: string;
  author: string;
  icon: string;
  category: string[];
  security: 'minimal' | 'standard' | 'strict';
  complexity: 1 | 2 | 3 | 4 | 5; // Stars
  downloads: number;
  rating: number;
  stars: number;
  readme: string; // Markdown
  files: {
    'wadah.yaml': string;
    'prompts/system.txt': string;
    // ... more files
  };
}
```

**"Use Template" Flow:**
1. Click "Use Template"
2. Modal: "Name your agent"
3. Auto-fill form with template data
4. User can customize
5. Save or deploy immediately

---

### 📦 Registry Page (`/registry`)

**Purpose:** Manage agent packages in OCI registries

**Layout:**
```
┌─────────────────────────────────────────────────────────┐
│ Package Registry                   [+ Add Registry]     │
├─────────────────────────────────────────────────────────┤
│ 🔍 Search packages...  [Registry ▾] [Sort ▾]           │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  Connected Registries                                    │
│  ┌────────────────────────────────────────────────────┐ │
│  │ 📦 GitHub Container Registry (ghcr.io)      [✓]   │ │
│  │ 📦 Docker Hub (docker.io)                   [✓]   │ │
│  │ 📦 Custom Registry (registry.company.com)   [✓]   │ │
│  └────────────────────────────────────────────────────┘ │
│                                                           │
│  Recent Packages                                         │
│  ┌────────────────────────────────────────────────────┐ │
│  │ ghcr.io/devwadahai/rag-service:0.1.0               │ │
│  │ Pushed 2 hours ago • 15.2 MB • ✓ Verified         │ │
│  │ [Pull] [Delete]                                    │ │
│  ├────────────────────────────────────────────────────┤ │
│  │ ghcr.io/devwadahai/devops-bot:0.2.1                │ │
│  │ Pushed 1 day ago • 12.8 MB • ✓ Verified           │ │
│  │ [Pull] [Delete]                                    │ │
│  └────────────────────────────────────────────────────┘ │
│                                                           │
└─────────────────────────────────────────────────────────┘
```

**Features:**
- Add/remove registries
- Push packages (drag-and-drop .wpkg)
- Pull packages by reference
- Version history
- Package details (manifest, layers, size)
- Access control
- Verification badges

**Push/Pull UI:**
```
Push Package
┌────────────────────────────────────┐
│ Drop .wpkg file here or click      │
│                                    │
│     [📁 Browse Files]              │
│                                    │
│ Registry: [ghcr.io ▾]              │
│ Repository: [devwadahai/my-agent]  │
│ Tag: [0.1.0]                       │
│                                    │
│ [Cancel] [Push]                    │
└────────────────────────────────────┘
```

---

### ⚙️ Settings Page (`/settings`)

**Purpose:** System-wide configuration

**Layout:**
```
┌─────────────────────────────────────────────────────────┐
│ Settings                                                 │
├──────────────┬──────────────────────────────────────────┤
│ Sections:    │                                           │
│ - Profile    │  API Keys                                │
│ - API Keys ← │                                           │
│ - Models     │  ┌────────────────────────────────────┐ │
│ - Security   │  │ OpenAI API Key                     │ │
│ - Team       │  │ sk-proj-...********                │ │
│ - Billing    │  │ [Reveal] [Test] [Delete]           │ │
│ - Integrations│  └────────────────────────────────────┘ │
│              │                                           │
│              │  ┌────────────────────────────────────┐ │
│              │  │ Ollama Endpoint                    │ │
│              │  │ http://localhost:11434             │ │
│              │  │ [Test] [Delete]                    │ │
│              │  └────────────────────────────────────┘ │
│              │                                           │
│              │  [+ Add API Key]                         │
│              │                                           │
│              │  Environment Variables                   │
│              │  ┌────────────────────────────────────┐ │
│              │  │ QDRANT_URL=http://localhost:6333   │ │
│              │  │ TGI_ENDPOINT=http://localhost:8080 │ │
│              │  └────────────────────────────────────┘ │
│              │                                           │
└──────────────┴──────────────────────────────────────────┘
```

**Sections:**

1. **Profile** - User info, avatar, preferences
2. **API Keys** - Model provider credentials
3. **Models** - Default models, endpoints
4. **Security** - Default security policies
5. **Team** - Invite members, roles, permissions
6. **Billing** - Usage, costs, limits
7. **Integrations** - GitHub, Docker, databases

---

## 🧩 Component Library

### Core Components

```typescript
// Button variants
<Button variant="primary | secondary | outline | ghost | danger">
  Click me
</Button>

// Input with validation
<Input
  label="Agent Name"
  placeholder="my-agent"
  error="Name is required"
  icon={<SearchIcon />}
/>

// Select dropdown
<Select
  label="Model Provider"
  options={[
    { value: 'openai', label: 'OpenAI' },
    { value: 'ollama', label: 'Ollama' },
  ]}
  onChange={handleChange}
/>

// Card container
<Card
  title="RAG Service"
  description="Document Q&A agent"
  footer={<Button>Deploy</Button>}
>
  {children}
</Card>

// Stats card
<StatsCard
  label="Total Agents"
  value={12}
  change={+15}
  trend="up"
  icon={<AgentIcon />}
/>

// Badge/Status
<Badge variant="success | warning | error | info">
  Running
</Badge>

// Toast notifications
toast.success('Agent deployed successfully!');
toast.error('Failed to deploy agent');
toast.info('Building package...');

// Modal/Dialog
<Dialog
  open={open}
  onClose={onClose}
  title="Delete Agent"
  description="Are you sure?"
>
  <Button onClick={handleDelete}>Delete</Button>
</Dialog>

// Tabs
<Tabs defaultValue="overview">
  <TabsList>
    <TabsTrigger value="overview">Overview</TabsTrigger>
    <TabsTrigger value="logs">Logs</TabsTrigger>
  </TabsList>
  <TabsContent value="overview">...</TabsContent>
</Tabs>

// Code editor
<CodeEditor
  language="yaml"
  value={yamlContent}
  onChange={setYamlContent}
  theme="vs-dark"
/>

// Terminal
<Terminal
  logs={logs}
  onCommand={handleCommand}
  theme="dark"
/>

// Progress bar
<ProgressBar
  value={75}
  label="Building package..."
  showPercentage
/>

// Data table
<DataTable
  columns={columns}
  data={agents}
  onRowClick={handleRowClick}
  sortable
  filterable
/>

// Chart
<LineChart
  data={metricsData}
  xAxis="time"
  yAxis="cost"
  tooltip
/>
```

---

## 🎨 Design System

### Color Palette

```css
/* Primary Colors */
--primary-50: #EFF6FF;
--primary-100: #DBEAFE;
--primary-500: #3B82F6;
--primary-600: #2563EB;
--primary-700: #1D4ED8;

/* Success */
--success-500: #10B981;
--success-600: #059669;

/* Warning */
--warning-500: #F59E0B;
--warning-600: #D97706;

/* Error */
--error-500: #EF4444;
--error-600: #DC2626;

/* Neutral */
--gray-50: #F9FAFB;
--gray-100: #F3F4F6;
--gray-200: #E5E7EB;
--gray-500: #6B7280;
--gray-700: #374151;
--gray-900: #111827;

/* Brand */
--wadah-blue: #0EA5E9; /* Ocean wave */
--wadah-dark: #0C4A6E;
```

### Typography

```css
/* Font Family */
--font-sans: 'Inter', -apple-system, system-ui, sans-serif;
--font-mono: 'JetBrains Mono', 'Fira Code', monospace;

/* Font Sizes */
--text-xs: 0.75rem;    /* 12px */
--text-sm: 0.875rem;   /* 14px */
--text-base: 1rem;     /* 16px */
--text-lg: 1.125rem;   /* 18px */
--text-xl: 1.25rem;    /* 20px */
--text-2xl: 1.5rem;    /* 24px */
--text-3xl: 1.875rem;  /* 30px */
--text-4xl: 2.25rem;   /* 36px */

/* Font Weights */
--font-normal: 400;
--font-medium: 500;
--font-semibold: 600;
--font-bold: 700;
```

### Spacing

```css
/* 4px base unit */
--space-1: 0.25rem;   /* 4px */
--space-2: 0.5rem;    /* 8px */
--space-3: 0.75rem;   /* 12px */
--space-4: 1rem;      /* 16px */
--space-6: 1.5rem;    /* 24px */
--space-8: 2rem;      /* 32px */
--space-12: 3rem;     /* 48px */
--space-16: 4rem;     /* 64px */
```

### Border Radius

```css
--radius-sm: 0.25rem;  /* 4px */
--radius-md: 0.5rem;   /* 8px */
--radius-lg: 0.75rem;  /* 12px */
--radius-xl: 1rem;     /* 16px */
--radius-full: 9999px; /* Pill shape */
```

### Shadows

```css
--shadow-sm: 0 1px 2px 0 rgb(0 0 0 / 0.05);
--shadow-md: 0 4px 6px -1px rgb(0 0 0 / 0.1);
--shadow-lg: 0 10px 15px -3px rgb(0 0 0 / 0.1);
--shadow-xl: 0 20px 25px -5px rgb(0 0 0 / 0.1);
```

### Animations

```css
/* Transitions */
--transition-fast: 150ms ease-in-out;
--transition-base: 200ms ease-in-out;
--transition-slow: 300ms ease-in-out;

/* Keyframes */
@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes slideUp {
  from { transform: translateY(10px); opacity: 0; }
  to { transform: translateY(0); opacity: 1; }
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}
```

---

## 🔌 API Integration

### Backend API Structure

```typescript
// tRPC API Router
export const appRouter = router({
  // Agents
  agents: router({
    list: publicProcedure.query(() => getAgents()),
    get: publicProcedure.input(z.string()).query(({ input }) => getAgent(input)),
    create: publicProcedure.input(agentSchema).mutation(({ input }) => createAgent(input)),
    update: publicProcedure.input(agentSchema).mutation(({ input }) => updateAgent(input)),
    delete: publicProcedure.input(z.string()).mutation(({ input }) => deleteAgent(input)),
  }),
  
  // Runs
  runs: router({
    list: publicProcedure.query(() => getRuns()),
    get: publicProcedure.input(z.string()).query(({ input }) => getRun(input)),
    start: publicProcedure.input(runSchema).mutation(({ input }) => startRun(input)),
    stop: publicProcedure.input(z.string()).mutation(({ input }) => stopRun(input)),
    logs: publicProcedure.input(z.string()).subscription(({ input }) => subscribeLogs(input)),
  }),
  
  // Templates
  templates: router({
    list: publicProcedure.query(() => getTemplates()),
    get: publicProcedure.input(z.string()).query(({ input }) => getTemplate(input)),
  }),
  
  // Registry
  registry: router({
    list: publicProcedure.query(() => listPackages()),
    push: publicProcedure.input(packageSchema).mutation(({ input }) => pushPackage(input)),
    pull: publicProcedure.input(z.string()).mutation(({ input }) => pullPackage(input)),
  }),
  
  // System
  system: router({
    health: publicProcedure.query(() => getSystemHealth()),
    stats: publicProcedure.query(() => getSystemStats()),
  }),
});
```

### CLI Integration

```typescript
// Execute Wadah CLI commands from UI
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

export async function wadahCLI(command: string, args: string[]): Promise<string> {
  const fullCommand = `wadah ${command} ${args.join(' ')}`;
  
  try {
    const { stdout, stderr } = await execAsync(fullCommand);
    
    if (stderr) {
      throw new Error(stderr);
    }
    
    return stdout;
  } catch (error) {
    throw new Error(`CLI Error: ${error.message}`);
  }
}

// Examples:
await wadahCLI('init', ['my-agent', '--security', 'minimal']);
await wadahCLI('pack', ['--output', 'agent.wpkg']);
await wadahCLI('run', ['wadah.yaml', '--prompt', 'Hello!']);
```

### WebSocket for Real-time Updates

```typescript
// Server-side (Socket.io)
io.on('connection', (socket) => {
  socket.on('subscribe:run', (runId: string) => {
    // Subscribe to run logs
    const logStream = watchRunLogs(runId);
    
    logStream.on('data', (log) => {
      socket.emit('run:log', { runId, log });
    });
    
    logStream.on('end', () => {
      socket.emit('run:complete', { runId });
    });
  });
});

// Client-side (React)
useEffect(() => {
  const socket = io('http://localhost:3030');
  
  socket.emit('subscribe:run', runId);
  
  socket.on('run:log', ({ log }) => {
    setLogs((prev) => [...prev, log]);
  });
  
  return () => socket.disconnect();
}, [runId]);
```

---

## 🗄️ State Management

### Global State (Zustand)

```typescript
import create from 'zustand';

interface AppState {
  // User
  user: User | null;
  setUser: (user: User) => void;
  
  // Agents
  agents: Agent[];
  setAgents: (agents: Agent[]) => void;
  addAgent: (agent: Agent) => void;
  
  // UI State
  sidebarOpen: boolean;
  toggleSidebar: () => void;
  
  // Theme
  theme: 'light' | 'dark';
  setTheme: (theme: 'light' | 'dark') => void;
}

export const useAppStore = create<AppState>((set) => ({
  user: null,
  setUser: (user) => set({ user }),
  
  agents: [],
  setAgents: (agents) => set({ agents }),
  addAgent: (agent) => set((state) => ({
    agents: [...state.agents, agent],
  })),
  
  sidebarOpen: true,
  toggleSidebar: () => set((state) => ({
    sidebarOpen: !state.sidebarOpen,
  })),
  
  theme: 'light',
  setTheme: (theme) => set({ theme }),
}));
```

### Server State (React Query)

```typescript
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';

// Fetch agents
export function useAgents() {
  return useQuery({
    queryKey: ['agents'],
    queryFn: () => fetch('/api/agents').then((r) => r.json()),
  });
}

// Create agent
export function useCreateAgent() {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: (agent: Agent) => 
      fetch('/api/agents', {
        method: 'POST',
        body: JSON.stringify(agent),
      }),
    onSuccess: () => {
      queryClient.invalidateQueries(['agents']);
      toast.success('Agent created!');
    },
  });
}

// Get run logs (with polling)
export function useRunLogs(runId: string) {
  return useQuery({
    queryKey: ['runs', runId, 'logs'],
    queryFn: () => fetch(`/api/runs/${runId}/logs`).then((r) => r.json()),
    refetchInterval: 1000, // Poll every second
  });
}
```

---

## 🔐 Security & Auth

### Authentication Options

**Option 1: Simple (v0.2.0)**
- Local auth with username/password
- JWT tokens
- No multi-user (single-user mode)

**Option 2: Full Auth (v0.3.0)**
- OAuth 2.0 (GitHub, Google)
- RBAC (Role-Based Access Control)
- Team management
- API keys per user

### Authorization

```typescript
// Role definitions
enum Role {
  ADMIN = 'admin',     // Full access
  DEVELOPER = 'dev',   // Create, edit, deploy
  VIEWER = 'viewer',   // Read-only
}

// Permission checks
function canDeleteAgent(user: User, agent: Agent): boolean {
  return user.role === Role.ADMIN || agent.createdBy === user.id;
}

// Protected routes
<ProtectedRoute requiredRole={Role.DEVELOPER}>
  <AgentBuilder />
</ProtectedRoute>
```

### API Security

```typescript
// Rate limiting
import rateLimit from 'express-rate-limit';

const limiter = rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 100, // 100 requests per window
});

app.use('/api/', limiter);

// CORS
app.use(cors({
  origin: process.env.ALLOWED_ORIGINS?.split(','),
  credentials: true,
}));

// Helmet for security headers
app.use(helmet());
```

---

## ⚡ Performance

### Optimization Strategies

1. **Code Splitting**
```typescript
// Lazy load heavy components
const AgentBuilder = lazy(() => import('./components/AgentBuilder'));
const RunMonitor = lazy(() => import('./components/RunMonitor'));
```

2. **Image Optimization**
```typescript
import Image from 'next/image';

<Image
  src="/agent-icon.png"
  width={64}
  height={64}
  alt="Agent"
  loading="lazy"
/>
```

3. **Caching**
```typescript
// React Query cache
queryClient.setDefaultOptions({
  queries: {
    staleTime: 5 * 60 * 1000, // 5 minutes
    cacheTime: 10 * 60 * 1000, // 10 minutes
  },
});

// API response caching
app.get('/api/templates', cacheMiddleware('5m'), getTemplates);
```

4. **Virtual Scrolling**
```typescript
// For large lists
import { useVirtualizer } from '@tanstack/react-virtual';

const virtualizer = useVirtualizer({
  count: agents.length,
  getScrollElement: () => parentRef.current,
  estimateSize: () => 80,
});
```

5. **Debouncing**
```typescript
// Search input
const debouncedSearch = useDebouncedValue(searchQuery, 300);
```

### Performance Targets

| Metric | Target | Notes |
|--------|--------|-------|
| First Contentful Paint | < 1.5s | Critical |
| Time to Interactive | < 3s | Important |
| Bundle Size (initial) | < 200KB | Gzipped |
| Lighthouse Score | > 90 | All categories |
| API Response Time | < 500ms | P95 |
| WebSocket Latency | < 100ms | P95 |

---

## 🚀 Deployment

### Development

```bash
# Install dependencies
npm install

# Start dev server
npm run dev

# Build for production
npm run build

# Start production server
npm start
```

### Production (Docker)

```dockerfile
# Dockerfile for Wadah UI
FROM node:20-alpine AS builder

WORKDIR /app

COPY package*.json ./
RUN npm ci

COPY . .
RUN npm run build

FROM node:20-alpine AS runner

WORKDIR /app

ENV NODE_ENV production

COPY --from=builder /app/.next/standalone ./
COPY --from=builder /app/.next/static ./.next/static
COPY --from=builder /app/public ./public

EXPOSE 3000

CMD ["node", "server.js"]
```

```bash
# Build and run
docker build -t wadah-ui:latest .
docker run -p 3000:3000 \
  -e DATABASE_URL="postgresql://..." \
  -e WADAH_CLI_PATH="/usr/local/bin/wadah" \
  wadah-ui:latest
```

### Environment Variables

```bash
# .env.production
DATABASE_URL="postgresql://user:pass@localhost:5432/wadah"
WADAH_CLI_PATH="/usr/local/bin/wadah"
DOCKER_SOCKET="/var/run/docker.sock"
NEXTAUTH_SECRET="your-secret-key"
NEXTAUTH_URL="https://wadah.company.com"
```

### Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: wadah-ui
spec:
  replicas: 2
  selector:
    matchLabels:
      app: wadah-ui
  template:
    metadata:
      labels:
        app: wadah-ui
    spec:
      containers:
      - name: wadah-ui
        image: ghcr.io/devwadahai/wadah-ui:0.2.0
        ports:
        - containerPort: 3000
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: wadah-secrets
              key: database-url
        volumeMounts:
        - name: docker-socket
          mountPath: /var/run/docker.sock
      volumes:
      - name: docker-socket
        hostPath:
          path: /var/run/docker.sock
---
apiVersion: v1
kind: Service
metadata:
  name: wadah-ui
spec:
  type: LoadBalancer
  ports:
  - port: 80
    targetPort: 3000
  selector:
    app: wadah-ui
```

---

## 🗺️ Roadmap

### v0.2.0 (Q4 2025) - MVP
**Priority: P0**

- [ ] Dashboard with stats
- [ ] Agents list and details
- [ ] Agent Builder (visual + YAML)
- [ ] Run Monitor with live logs
- [ ] Template library
- [ ] Basic settings (API keys)
- [ ] Local authentication
- [ ] Docker integration
- [ ] PostgreSQL database

**Timeline:** 8-10 weeks  
**Team:** 2 frontend, 1 backend, 1 designer

### v0.3.0 (Q1 2026) - Enhanced
**Priority: P1**

- [ ] Registry management (push/pull)
- [ ] Advanced run analytics
- [ ] Team collaboration
- [ ] OAuth integration
- [ ] RBAC
- [ ] Cost optimization dashboard
- [ ] Alert system
- [ ] Mobile responsive
- [ ] Dark mode

**Timeline:** 6-8 weeks  
**Team:** 2 frontend, 1 backend

### v0.4.0 (Q2 2026) - Enterprise
**Priority: P2**

- [ ] Multi-tenant architecture
- [ ] SSO/SAML
- [ ] Audit logs
- [ ] Advanced security policies
- [ ] Custom integrations
- [ ] White-label options
- [ ] API documentation UI
- [ ] Workflow automation
- [ ] Advanced monitoring

**Timeline:** 8-12 weeks  
**Team:** 3 frontend, 2 backend, 1 DevOps

### Future (2026+)
**Priority: P3**

- [ ] Mobile app (React Native)
- [ ] VS Code extension
- [ ] AI assistant for building agents
- [ ] Marketplace for templates
- [ ] Community features
- [ ] Plugin ecosystem
- [ ] GraphQL API
- [ ] Real-time collaboration

---

## 📐 Technical Specifications

### Project Structure

```
wadah-ui/
├── app/                      # Next.js 14 App Router
│   ├── (auth)/              # Auth routes
│   │   ├── login/
│   │   └── register/
│   ├── (dashboard)/         # Main app routes
│   │   ├── page.tsx         # Dashboard
│   │   ├── agents/
│   │   │   ├── page.tsx     # Agents list
│   │   │   ├── [id]/        # Agent details
│   │   │   └── new/         # Agent builder
│   │   ├── runs/
│   │   │   ├── page.tsx     # Runs list
│   │   │   └── [id]/        # Run monitor
│   │   ├── templates/
│   │   ├── registry/
│   │   └── settings/
│   ├── api/                 # API routes
│   │   ├── agents/
│   │   ├── runs/
│   │   └── trpc/[trpc]/     # tRPC handler
│   └── layout.tsx           # Root layout
├── components/              # React components
│   ├── ui/                  # shadcn/ui components
│   ├── dashboard/
│   ├── agents/
│   └── shared/
├── lib/                     # Utilities
│   ├── api.ts              # API client
│   ├── cli.ts              # CLI integration
│   ├── db.ts               # Database (Prisma)
│   └── utils.ts
├── hooks/                   # Custom hooks
├── stores/                  # Zustand stores
├── styles/                  # Global styles
├── public/                  # Static assets
├── prisma/                  # Database schema
│   └── schema.prisma
├── tests/                   # Tests
│   ├── unit/
│   ├── integration/
│   └── e2e/
├── package.json
├── tsconfig.json
├── tailwind.config.ts
└── next.config.js
```

### Database Schema (Prisma)

```prisma
// prisma/schema.prisma

model User {
  id        String   @id @default(cuid())
  email     String   @unique
  name      String?
  password  String
  role      Role     @default(DEVELOPER)
  agents    Agent[]
  runs      Run[]
  createdAt DateTime @default(now())
  updatedAt DateTime @updatedAt
}

model Agent {
  id          String   @id @default(cuid())
  name        String
  version     String
  description String?
  spec        Json     // Full wadah.yaml
  status      Status   @default(STOPPED)
  user        User     @relation(fields: [userId], references: [id])
  userId      String
  runs        Run[]
  createdAt   DateTime @default(now())
  updatedAt   DateTime @updatedAt
  
  @@unique([name, version])
}

model Run {
  id        String   @id @default(cuid())
  agent     Agent    @relation(fields: [agentId], references: [id])
  agentId   String
  user      User     @relation(fields: [userId], references: [id])
  userId    String
  status    RunStatus @default(PENDING)
  prompt    String?
  logs      Json[]
  trace     Json?
  metrics   Json?
  startedAt DateTime @default(now())
  endedAt   DateTime?
  cost      Float    @default(0)
  tokens    Int      @default(0)
}

model Template {
  id          String   @id @default(cuid())
  name        String   @unique
  description String
  category    String[]
  spec        Json
  downloads   Int      @default(0)
  rating      Float    @default(0)
  createdAt   DateTime @default(now())
}

enum Role {
  ADMIN
  DEVELOPER
  VIEWER
}

enum Status {
  RUNNING
  STOPPED
  ERROR
}

enum RunStatus {
  PENDING
  RUNNING
  SUCCESS
  FAILED
  CANCELLED
}
```

### API Endpoints

```typescript
// REST API (fallback for CLI)
GET    /api/agents                  // List agents
POST   /api/agents                  // Create agent
GET    /api/agents/:id              // Get agent
PUT    /api/agents/:id              // Update agent
DELETE /api/agents/:id              // Delete agent

GET    /api/runs                    // List runs
POST   /api/runs                    // Start run
GET    /api/runs/:id                // Get run
GET    /api/runs/:id/logs           // Get logs (SSE)
POST   /api/runs/:id/stop           // Stop run

GET    /api/templates               // List templates
GET    /api/templates/:id           // Get template

POST   /api/registry/push           // Push package
POST   /api/registry/pull           // Pull package

GET    /api/system/health           // Health check
GET    /api/system/stats            // System stats

// WebSocket
WS     /api/ws                      // Real-time updates
```

---

## 🎯 Success Metrics

### User Metrics
- **Adoption:** 100+ active users in first month
- **Engagement:** 70%+ weekly active users
- **Retention:** 80%+ 30-day retention
- **NPS:** > 50

### Performance Metrics
- **Load Time:** < 2s first load
- **Uptime:** 99.9%
- **Error Rate:** < 1%
- **API Latency:** < 500ms P95

### Business Metrics
- **Agent Creation:** 500+ agents created
- **Runs:** 10,000+ agent runs
- **Templates Used:** 80%+ use templates
- **Cost Savings:** 30% reduction vs manual workflows

---

## 📚 Documentation

### For Developers
- [ ] Contributing guide
- [ ] Component storybook
- [ ] API documentation
- [ ] Database schema docs
- [ ] Deployment guide

### For Users
- [ ] User guide
- [ ] Video tutorials
- [ ] FAQ
- [ ] Troubleshooting
- [ ] Best practices

---

## 🎉 Conclusion

This UI specification provides a comprehensive blueprint for building Wadah's web interface. The design prioritizes:

1. **Simplicity** - Easy for anyone to use
2. **Power** - Full feature parity with CLI
3. **Speed** - Fast, responsive experience
4. **Beauty** - Modern, clean design
5. **Scalability** - Ready for growth

**Next Steps:**
1. Review and approve spec
2. Create design mockups (Figma)
3. Set up Next.js project
4. Build component library
5. Implement core features
6. Beta testing
7. Launch v0.2.0!

---

**Wadah UI Specification v1.0**  
**Created:** November 1, 2025  
**Target Release:** v0.2.0 (Q4 2025)  
**Status:** Ready for Review ✅

*"Make AI agents accessible to everyone."* 🌊

