# Pretty Decent Party Game Platform

### Required prerequisites

1. [Node.js](https://nodejs.org/en/download)  
2. [pnpm CLI](https://pnpm.io/installation) (project is monorepo and uses `pnpm` workspace)  
3. [Rust toolchain](https://rust-lang.org/tools/install/)  
4. [TypeShare CLI](https://1password.github.io/typeshare/installation.html) (`typeshare`)  

### Generating data types

All data type generation happens automatically on install:
```bash
pnpm install
```

This single command:
- Installs all Node.js dependencies  
- Generates TypeScript types from Rust structs (TypeShare)  

To run clients:
```bash
# from root
pnpm run dev
```

To run server:
```bash
cd server
cargo run
```

To see commands and what they use, check scripts in `./package.json`:
```json
"scripts": {
	"dev": "pnpm --filter host --filter controller dev",
	"dev:host": "pnpm --filter host dev",
	"dev:controller": "pnpm --filter controller dev",
	"typeshare": "cd server && typeshare . --lang=typescript --output-file=../packages/typeshare/types/index.ts",
	"postinstall": "pnpm typeshare"
}
```

### Project structure

```txt
.
├── docs/                # Documentation
│
├── apps                 # Web client applications
│   ├── controller/      # Controller client (TypeScript/React)
│   └── host/            # Host client (TypeScript/React)
│
├── server               # Rust server workspace
│   ├── Cargo.lock
│   ├── Cargo.toml       # Workspace configuration
│   └── crates/          # Main binary - 'server' crate
│
├── packages/            # Shared packages for clients
│
├── package.json         # Root package.json (workspace scripts)
├── pnpm-lock.yaml
├── pnpm-workspace.yaml  # pnpm workspace configuration
│
├── .gitignore
└── README.md
```

