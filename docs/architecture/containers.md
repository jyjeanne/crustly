# Containers (C4 L2)

Hand-authored from `docs/ARCHITECTURE.md` section 1 ("High-Level Architecture"),
which lays out five layers: User Interface, Application, Provider, Service, and
Database. Update this alongside that section if the layering changes.

```mermaid
C4Container
    title Crustly - Containers

    Person(developer, "Developer")

    System_Boundary(crustly, "Crustly") {
        Container(ui, "User Interface", "Ratatui / Clap", "TUI (interactive chat, plan mode, file picker, tool approval) and CLI (single commands, batch, config)")
        Container(app, "Application Layer", "Rust", "AgentService (conversation + tool loop + cost tracking), PromptAnalyzer (intent/tool-hint detection)")
        Container(provider, "Provider Layer", "Rust traits", "Provider trait + backends (Anthropic, OpenAI, Bedrock, Azure, Ollama, ...), Tool Registry (20+ tools)")
        Container(service, "Service Layer", "Rust", "SessionService, MessageService, PlanService")
        ContainerDb(db, "Database Layer", "SQLite / SQLx", "Sessions, Messages, Plans, PlanTasks, Files, ToolExecutions")
    }

    System_Ext(llm, "LLM Provider APIs")
    System_Ext(shell, "Shell / Filesystem / Git")

    Rel(developer, ui, "Uses")
    Rel(ui, app, "Dispatches messages/commands")
    Rel(app, provider, "Sends requests, invokes tools")
    Rel(provider, llm, "HTTP(S)")
    Rel(provider, shell, "Tool execution")
    Rel(app, service, "Reads/writes sessions, messages, plans")
    Rel(service, db, "SQL")
```

## Notes

- This mirrors `docs/ARCHITECTURE.md` §1 exactly; treat that document as the
  narrative source and this diagram as its GitHub-renderable summary.
- Cross-check against `docs/graph/GRAPH_REPORT.md`'s community list
  periodically - if a community there doesn't map cleanly onto one of these
  five containers, either this diagram or the actual module boundaries have
  drifted.
