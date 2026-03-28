## Shared data autogeneration

We need a solution for sharing data structures between our TypeScript clients and Rust server. The requirements are:

- **Single source of truth**: One schema definition that generates both TypeScript and Rust types
- **Type safety**: Compile-time guarantees on both client and server
- **Performance**: Efficient for real-time game updates (player positions, game state, events)
- **WebSocket compatible**: Works with binary or text WebSocket messages
- **Developer experience**: Good tooling support, easy setup and update

Our game will send frequent state updates, so message size and serialization speed are critical.

### Possible solutions

**[Protocol Buffers (Protobuf)](https://protobuf.dev/)**  

Data structures are defined in .proto files, which act as the single source of truth. Rust and TypeScript types are generated from this schema. Messages are sent as compact binary data over WebSockets, making them smaller and faster than JSON. Also supports schema evolution and backward compatibility.

**[TypeShare (Rust-first approach)](https://github.com/1Password/typeshare)**  

Data structures are defined in Rust using the #[typeshare] annotation, and TypeScript types are generated from them. Messages are sent as JSON over WebSockets. This approach is simple and easy to use, but it only shares types.

## Decision

TypeShare is probably a good trade-off between how easy it is to set up and efficiency. We could come back to proto if performance becomes a bottleneck.

### Required prerequisites

Generic:
1. [Node.js](https://nodejs.org/en/download)  
2. [pnpm CLI](https://pnpm.io/installation) (project is monorepo and uses `pnpm` workspace)  
3. [Rust toolchain](https://rust-lang.org/tools/install/)  

```bash
brew install node
brew install pnpm
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Protobuf specific:

1. [Protobuf compiler](https://protobuf.dev/installation/) (`protoc`)

```bash
brew install protobuf
```

TypeShare specific:

1. [TypeShare CLI](https://1password.github.io/typeshare/installation.html) (`typeshare`)

```bash
cargo install typeshare-cli
```

### Generating data types

All data type generation happens automatically on install:
```bash
pnpm install
```

This single command:
- Installs all Node.js dependencies
- Generates TypeScript types from Proto files
- Generates Rust types from Proto files
- Generates TypeScript types from Rust structs (TypeShare)

To see commands and what they use, check scripts in `./package.json`:
```json
"scripts": {
	...
	"proto:ts": "pnpm --filter @party/proto run generate",
	"proto:rs": "cd server && cargo build -p proto-rs",
	"proto": "pnpm proto:ts && pnpm proto:rs",
	"typeshare": "cd server && typeshare . --lang=typescript --output-file=../packages/type-share/index.ts",
	"postinstall": "pnpm proto && pnpm typeshare"
},
```

### Project structure

This project uses a **monorepo** structure to keep all code in a single repository, making it easier to:
- Share types between frontend and backend
- Coordinate changes across client and server
- Maintain consistent tooling and dependencies
- Simplify CI/CD pipelines

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

### Protobuf vs TypeShare Comparison

|  | Protocol buffer | TypeShare | Comment |
| :-- | :---: | :--: | :-- |
| **Dev Setup Complexity** | Medium | Low | Proto requires `protoc` installation and build scripts. Proto files are standardized (Proto syntax). TypeShare is just a Rust crate and CLI tool. Protobuf is more complicated to set up. |
| **Message Size** | 33 bytes (binary) | 66 bytes (binary as MessagePack) | Size for data `{ "userName": "Martin", "favoriteNumber": 1337, "interests": ["daydreaming", "hacking"] }` |
| **Source of Truth** | `.proto` files | Rust structs | Proto is language-agnostic. TypeShare is Rust-first. |
| **Type Safety** | Compile-time | Compile-time | Both provide full type safety |
| **Schema Evolution** | Built-in | Manual | Proto has field numbering for backward compatibility |
| **Binary Format** | Yes | JSON/MessagePack | Proto is true binary. TypeShare uses text/binary serializers |
| **Human Readable** | Binary only | JSON is readable | Proto requires tools to inspect. JSON can be read directly |
| **Debugging** | Medium | Easy | JSON in browser console vs binary inspection tools |
| **Learning Curve** | Medium | Low | Proto syntax vs familiar Rust syntax |
| **WebSocket Support** | Binary | Text/Binary | Both work with WebSocket |
| **IDE Support** | Plugins available | Standard Rust | Both have good IDE support |
| **Versioning** | Built-in | Manual | Proto field numbers enable versioning |
| **Maintenance** | Medium | Low | Proto requires schema files. TypeShare works with existing Rust code |
| **Error Messages** | Good | Excellent | TypeShare uses native Rust errors |
| **Generated Files** | Not committed to repo | Committed to `packages/type-share/index.ts` | Proto files are regenerated on build. TypeShare output is committed for easier review and version control. |

### Generated files output

**Protobuf** (after running `pnpm install`):
Source: `proto/controller-types/controller_types_v1.proto`
Generated TypeScript: `packages/proto-ts/generated/controller-types/controller_types_v1.ts`
Generated Rust: `server/crates/proto-rs/src/generated/controller.types.v1.rs`

**TypeShare**:
Source: `server/crates/model/src/lib.rs`
Generated TypeScript: `packages/type-share/index.ts`