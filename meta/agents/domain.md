# Domain Docs

How the engineering skills should consume this repo's domain documentation when exploring the codebase.

## Layout

Single-context. All domain docs live under `meta/`:

```
/
├── CLAUDE.md                ← agent rules (auto-discovered)
└── meta/
    ├── CONTEXT.md           ← domain glossary (lazily created)
    ├── adr/                 ← architectural decisions (lazily created)
    └── agents/              ← per-skill config
```

`docs/` is the Astro Starlight project that builds the published site at
dublette.espadat.com — a build directory, not a place for agent or domain files.
Published pages go in `docs/src/content/docs/`; everything unpublished stays in
`meta/`. The site's shape is decided by ADR-0001 in `espadat-studio/docs-theme`,
not here: Starlight on a per-tool subdomain, themed by a shared git-pinned
package, light-only, with the sidebar hand-written so existing URLs survive.

## Before exploring, read these

- **`meta/CONTEXT.md`** if it exists — domain glossary.
- **`meta/adr/`** if it exists — read ADRs that touch the area you're about to work in.

If any of these files don't exist, **proceed silently**. Don't flag their absence; don't suggest creating them upfront. The producer skill (`/grill-with-docs`) creates them lazily when terms or decisions actually get resolved.

## Use the glossary's vocabulary

When your output names a domain concept (in an issue title, a refactor proposal, a hypothesis, a test name), use the term as defined in `meta/CONTEXT.md`. Don't drift to synonyms the glossary explicitly avoids.

If the concept you need isn't in the glossary yet, that's a signal — either you're inventing language the project doesn't use (reconsider) or there's a real gap (note it for `/grill-with-docs`).

## Flag ADR conflicts

If your output contradicts an existing ADR, surface it explicitly rather than silently overriding:

> _Contradicts ADR-0007 (event-sourced orders) — but worth reopening because…_
