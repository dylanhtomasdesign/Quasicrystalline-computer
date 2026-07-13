# Architettura Hardware Eterogenea di Senemosìa

## Panoramica

Senemosìa Punto Zero implementa un'architettura di calcolo eterogeneo che sfrutta tre paradigmi hardware fisici:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    KERNEL CORE (Rust no_std)                            │
│                 Coordinate 6D Quasicristalline                            │
└─────────────────────────────────────────────────────────────────────────┘
           │                         │                         │
           ▼                         ▼                         ▼
┌──────────────────┐   ┌──────────────────┐   ┌──────────────────┐
│   DOMINIO QPU    │   │  DOMINIO NPU     │   │  DOMINIO ANALOG  │
│   (Calcolo       │   │  (Neuromorfico   │   │  (Circuiti       │
│   Quantistico)   │   │   Memristori)    │   │   Analogici)     │
├──────────────────┤   ├──────────────────┤   ├──────────────────┤
│ Grover O(√N)     │   │ STDP Learning    │   │ KCL/KVL Solver   │
│ OpenQASM 3.0     │   │ Crossbar Array   │   │ Zero CPU Cost    │
│ PCIe DMA         │   │ In-Memory Comp   │   │ Latenza Zero     │
└──────────────────┘   └──────────────────┘   └──────────────────┘
```

---

## 🧪 Dominio Quantistico (QPU)

### Struttura Hardware

```text
┌────────────────────────────────────────────────────────┐
│                    QPU INTERFACE                       │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐           │
│  │  IBM Q   │  │ Rigetti  │  │  IonQ    │           │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘           │
│       └─────────────┼─────────────┘                  │
│                     ▼                                 │
│            ┌────────────────┐                         │
│            │  PCIe DMA Bus  │                         │
│            │  0x4000_0000   │                         │
│            └────────┬───────┘                         │
│                     ▼                                 │
│            ┌────────────────┐                         │
│            │ QPURegisters   │                         │
│            │ MMIO Control   │                         │
│            └────────────────┘                         │
└────────────────────────────────────────────────────────┘
```

### Registri MMIO

| Indirizzo | Funzione | Descrizione |
|-----------|----------|-------------|
| 0x4000_0000 | QPU_CTRL_REG | Registro di controllo |
| 0x4000_0004 | QPU_STATUS_REG | Stato QPU (IDLE/RUNNING/DONE) |
| 0x4000_0008 | QPU_CMD_REG | Comandi (RESET/START/LOAD/EXEC) |
| 0x4000_0010 | QPU_DMA_ADDR_REG | Indirizzo DMA per QASM |
| 0x4000_0018 | QPU_RESULT_REG | Risultati misura |

### Algoritmo di Grover

```rust
// Ricerca O(√N) per prossimità geometrica in SNFS
let grover = GroverSearch::new(8, target_index);  // 8 qubit
let coords = grover.target_coordinates();  // Coordinate 6D del target
```

### OpenQASM 3.0

```qasm
OPENQASM 3.0;
include "stdgates.inc";
qreg q[8];
creg c[8];

// Superposizione
h q[0]; h q[1]; h q[2]; h q[3]; h q[4]; h q[5]; h q[6]; h q[7];

// Oracle + Diffusion (iterazioni √N)
```

---

## 🧠 Dominio Neuromorfico (NPU)

### Crossbar Memristivo

```text
┌────────────────────────────────────────────────────────┐
│              MEMRISTOR CROSSBAR ARRAY                   │
│                                                        │
│  Row0 ──┬──[M]──┬──[M]──┬──[M]──┬── ... ──┬──[M]──┐   │
│         │        │        │        │         │        │   │
│  Row1 ──┼──[M]──┬──[M]──┬──[M]──┬── ... ──┼──[M]──┤   │
│         │        │        │        │         │        │   │
│  Row2 ──┼──[M]──┬──[M]──┬──[M]──┬── ... ──┼──[M]──┤   │
│         │        │        │        │         │        │   │
│         ...      │        │        │         │        │   │
│         │        │        │        │         │        │   │
│  RowN ──┴────────┴────────┴────────┴── ... ──┴────────┘   │
│              │        │        │                    │     │
│           Col0     Col1     Col2    ...           ColN   │
│                                                        │
│  G[i,j] = Conductance[i,j] = Peso Sinaptico            │
└────────────────────────────────────────────────────────┘
```

### STDP (Spike-Timing Dependent Plasticity)

```rust
// Apprendimento attraverso timing degli spike
pub fn applica_impulso_stdp(&mut self, synapse_id: usize, delta_t: i32) {
    if delta_t > 0 {
        // LTP: Pre-synaptic spike prima → Aumenta conduttanza
        self.conductances[synapse_id].ltp(delta_t);
    } else {
        // LTD: Post-synaptic spike prima → Diminuisce conduttanza
        self.conductances[synapse_id].ltd(delta_t);
    }
}
```

### In-Memory Computing

```rust
// Vector-Matrix Multiplication senza spostare dati
let outputs = memristor_array.compute_vmm(input_voltages);
```

---

## 🎛️ Dominio Analogico

### Risolutore Spettrale Analogico

```text
┌────────────────────────────────────────────────────────┐
│           ANALOG SPECTRAL SOLVER                        │
│                                                        │
│   DAC (Laplacian)              ADC (Fiedler Vector)   │
│   ┌─────────────┐              ┌─────────────┐        │
│   │ G[0] ──────┼──┐      ┌───│──── V[0]    │        │
│   │ G[1] ──────┼──┤      │   │──── V[1]    │        │
│   │ G[2] ──────┼──┤ KCL  ├───│──── V[2]    │        │
│   │ ...   ─────┼──┤ KVL  │   │ ...         │        │
│   │ G[31]──────┴──┘      └───│──── V[31]   │        │
│   └─────────────┘              └─────────────┘        │
│                                                        │
│   L · v = λ · v  (risolto in tempo reale)            │
└────────────────────────────────────────────────────────┘
```

### Flusso di Calcolo

```rust
// 1. Scrivi matrice Laplaciana come conduttanze DAC
dac.ricalibra_scheduler_analogico(&laplacian_matrix);

// 2. Circuito analogico risolve automaticamente via Kirchhoff
//    Lx = λx → vettore di Fiedler = priorità scheduling

// 3. Leggi risultato dall'ADC (tempo di convergenza: µs)
let fiedler = adc.leggi_vettore_fiedler();

// 4. Estrai priorità per scheduler
let priorities = solver.estrai_priorita_scheduler(&fiedler);
```

---

## 🔄 Bridge Geometrico

```
┌─────────────────────────────────────────────────────────────────────┐
│                    BRIDGE GEOMETRICO                                 │
│                                                                     │
│  QPU qubits ────────────────────────► Coordinate 6D                 │
│  │ │ │ │ │ │ ──→ [α.re·φ, β.re, α.im, β.im, φ·0.618, 0]           │
│                                                                     │
│  NPU spikes ─────────────────────────► Sistema Nodale                │
│  │ │ │ │ │ │ ──→ [v_avg, spike_rate, max_weight, time, n·φ⁻¹]      │
│                                                                     │
│  Analog voltages ────────────────────► Vettore di Fiedler            │
│  │ │ │ │ │ │ ──→ [v₀·φ, v₁·φ⁻¹, v₂·(φ-1), n_comp·0.1, ...]       │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 📊 Comparazione Complessità

| Operazione | CPU Classica | Hardware Senemosìa |
|------------|--------------|---------------------|
| DB Search | O(N) | O(√N) via QPU |
| Spectral Analysis | O(N³) Lanczos | O(1) via Analog |
| Matrix Mult | O(N³) | O(1) via Memristor |
| Pattern Learning | Iterativo | STDP Hardware |

---

## 🚀 Inizializzazione Kernel

```rust
pub extern "C" fn _start() -> ! {
    // 1. QPU - Grover per ricerca DB
    let mut qpu_driver = QPUDriver::new();
    let mut grover = GroverSearch::new(8, 42);

    // 2. NPU - Memristor crossbar
    let mut memristor_array = MemristorArray::new(64, 64, 1e-3);

    // 3. Analog - Spectral solver
    let mut analog_solver = AnalogSpectralSolver::new();

    loop { scheduler.schedule_next(); }
}
```

MIT License - Senemosìa Cooperative
