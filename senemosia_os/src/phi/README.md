# Φ-Lang (Phi-Lang) - The Quasicrystalline Programming Language

Φ-Lang is a domain-specific programming language designed for the **Senemosìa Punto Zero** quasicrystalline operating system. It provides native support for hyperdimensional computing with 6D coordinates and golden ratio (φ) mathematics.

## 📜 Overview

Φ-Lang bridges human-readable application code with the kernel's geometric computing primitives:

```phi
usa core::libraries::dizionario;

// Inizializza il dizionario spaziale
mappa = DizionarioSpaziale.inizializza();

// Registra un nodo con coordinate 6D
mappa.associa("Produttore_OrtoBio", [0.618, 1.0, 0.382, 0.0, 1.0, 0.618]);

// Risolvi coordinate da stringa
coord = mappa.risolvi("Produttore_OrtoBio");
```

## 🏗️ Language Structure

```
src/phi/
├── mod.rs                    # Module exports
├── syscall_bindings.rs       # Kernel syscall interface
├── core/
│   ├── libraries/
│   │   └── dizionario.phi   # Spatial Dictionary
│   └── syscalls.phi         # System call interface
└── apps/
    └── gestione_cooperativa.phi  # Cooperative management example
```

## 🔑 Key Concepts

### 6D Hyperdimensional Coordinates
Every node in the system is identified by a 6D coordinate vector:
```phi
coord_6d: MatriceReale[6] = [x1, x2, x3, x4, x5, x6];
```

### Golden Ratio (φ)
The golden ratio φ ≈ 1.618 is used throughout:
- Memory partitioning (φ-alloc)
- Process scheduling weights
- Node frequency calculation
- Coordinate generation

### Spatial Dictionary
Maps human-readable strings to 6D coordinates:
```phi
DizionarioSpaziale.associa("NomeNodo", [0.618, 1.0, 0.382, 0.0, 1.0, 0.618]);
DizionarioSpaziale.risolvi("NomeNodo");  // Returns [0.618, 1.0, ...]
```

### Wave-Based IPC
Processes communicate through resonant wave channels:
```phi
SYSCALL(SYS_WAVE_IPC_SEND, coord_dest, "Messaggio");
```

## 📋 Standard Library

### `core::libraries::dizionario`
Dynamic registration and resolution of spatial nodes.

**Key Functions:**
- `DizionarioSpaziale.inizializza()` - Create new dictionary
- `DizionarioSpaziale.associa(chiave, coordinate)` - Register node
- `DizionarioSpaziale.risolvi(chiave)` - Get coordinates
- `DizionarioSpaziale.disattiva(chiave)` - Deactivate node
- `DizionarioSpaziale.lista_nodi_attivi()` - List all nodes

### `core::syscalls`
System call interface for kernel communication.

### `core::ipc`
Wave-based inter-process communication.

## 🚀 Kernel Syscall Numbers

| Syscall | Number | Description |
|---------|--------|-------------|
| `SYS_ALLOC_NODE` | 0x100 | Allocate geometric node |
| `SYS_GET_RESONANCE` | 0x101 | Get resonance value |
| `SYS_SNFS_REGISTER_NODE` | 0x200 | Register in spatial dictionary |
| `SYS_SNFS_RESOLVE_NODE` | 0x201 | Resolve key to coordinates |
| `SYS_SNFS_LIST_NODES` | 0x202 | List all registered nodes |
| `SYS_SNFS_DELETE_NODE` | 0x203 | Remove node from dictionary |
| `SYS_WAVE_IPC_SEND` | 0x300 | Send wave IPC message |
| `SYS_WAVE_IPC_RECEIVE` | 0x301 | Receive wave IPC message |

## 💻 Example Application

See `apps/gestione_cooperativa.phi` for a complete example of:
- Registering producers and riders
- Dynamic coordinate generation
- Wave-based notification routing

## 🔧 Status

Φ-Lang is currently defined as a specification. A full compiler/interpreter would parse `.phi` files and generate:
1. Syscall bindings for dictionary operations
2. Kernel memory allocations for node data
3. IPC channel setup for wave communication

## 📄 License

MIT License
