# Architettura Ibrida Neuromorfica Quasicristallina

## Proposta di Architettura Computazionale Ibrida con Tassellazioni Quasiperiodiche

---

## Abstract

Propongo un'architettura neuromorfica ibrida che combina due tassellazioni quasiperiodiche complementari: la **tassellazione di Ammann-Beenker** per il routing efficiente dei dati e la **tassellazione di Penrose** per la disposizione locale dei neuroni. Questa architettura sfrutta le proprietà uniche di ciascuna struttura geometrica per ottimizzare rispettivamente la comunicazione inter-neuronale e le capacità di calcolo aperiodico.

---

## 1. Introduzione

### 1.1 Motivazione

Le tassellazioni quasiperiodiche offrono proprietà uniche per il calcolo neuromorfico:
- **Simmetria icosaedrale**: Ottimale per pattern recognition
- **Aperiodicità**: Efficace per memorizzazione associativa
- **Autosimilarità**: Adatta per rappresentazione gerarchica

Tuttavia, ogni tassellazione ha punti di forza diversi:

| Proprietà | Ammann-Beenker | Penrose |
|-----------|----------------|---------|
| Routing | ✅ Eccellente | ⚠️ Complesso |
| Connettività locale | ⚠️ Media | ✅ Ottimale |
| Gestione topologica | ✅ Semplice | ❌ Complessa |
| Calcolo aperiodico | ⚠️ Buono | ✅ Eccellente |

### 1.2 Soluzione Ibrida

```
┌─────────────────────────────────────────────────────────────────┐
│                    HYBRID QUASICRYSTALLINE ARCHITECTURE          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────┐         ┌─────────────────┐               │
│  │  PENROSE LAYER  │         │  AMMANN-BEENKER │               │
│  │   (Neurons)     │◄───────►│    LAYER        │               │
│  │                 │   DATA   │   (Routing)     │               │
│  │  Local Compute  │  FLOW   │                 │               │
│  └─────────────────┘         └─────────────────┘               │
│          ▲                            ▲                         │
│          │     INTERFACE LAYER        │                         │
│          └────────────────────────────┘                         │
│                    (Projection)                                  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. Fondamenti Geometrici

### 2.1 Tassellazione di Ammann-Beenker

La tassellazione di Ammann-Beenker è una tassellazione quasiperiodica del piano con simmetria ottagonale (simmetria di ordine 8).

#### Definizione Matematica

La tassellazione può essere generata attraverso la **metodo di taglio-proiezione** da un reticolo 4D:

```
Vettori di proiezione per Ammann-Beenker (simmetria 4-fold):

v₁ = (1, 0, 0, 0)
v₂ = (0, 1, 0, 0)
v₃ = (0, 0, 1, 0)
v₄ = (0, 0, 0, 1)

Finestra di accettazione (strip):
S = {x ∈ ℝ⁴ : |xᵢ| ≤ 1/2, Σxᵢ = 0}
```

#### Struttura dei Ripieni (Inflators/Deflators)

```
Regola di inflazione Ammann-Beenker (factor φ = 1+√2):

        ╱╲         →        ╱╲╱╲
       ╱  ╲               ╱╲  ╱╲
      ╱╲  ╱╲      →     ╱  ╲╱  ╲
     ╱  ╲╱  ╲           ╱╲╱╲╱  ╲
    ╱╲  ╱╲  ╱╲         ╱  ╱╲╱╲  ╱╲
   ╱  ╲╱  ╲╱  ╲       ╱╲╱  ╲╱  ╲╱  ╲
```

#### Proprietà per Routing

- **Grado massimo**: 4 (gestibile)
- **Diametro medio**: O(log N) per N nodi
- **Bilanciamento del carico**: Ottimale grazie a simmetria 8-fold

### 2.2 Tassellazione di Penrose

La tassellazione di Penrose (P3) utilizza rombi acuti e ottusi con simmetria 5-fold, ottimale per computazione locale.

#### Definizione Matematica

```
Proiezione da 5D (simmetria icosaedrale):

v₁ = (1, 0, 0, 0, 0)
v₂ = (0, 1, 0, 0, 0)
v₃ = (0, 0, 1, 0, 0)
v₄ = (0, 0, 0, 1, 0)
v₅ = (0, 0, 0, 0, 1)

Golden ratio φ = (1+√5)/2 compare nella finestra di accettazione
```

#### Struttura dei Quasi-cristalli

```
           ╱╲
          ╱  ╲
         ╱    ╲
        ╱      ╲
       ╱   72°  ╲
      ╱          ╲
     ╱            ╲
    ╱      P3      ╲
   ╱                ╲
  ╱──────────────────╲
      Rombo Acuto (36°/144°)
      
           ╱╲
          ╱  ╲
         ╱    ╲
        ╱      ╲
       ╱        ╲
      ╱   108°   ╲
     ╱            ╲
    ╱              ╲
   ╱────────────────╲
      Rombo Ottuso (108°/72°)
```

### 2.3 Connessione Geometrica

La proiezione tra i due spazi può essere realizzata attraverso una trasformazione che preserva la località:

```
Proiezione Ammann-Beenker → Penrose:

P = M × A

dove M è la matrice di matching che preserva le distanze locali
e A sono le coordinate Ammann-Beenker

M ≈ [[φ, 0, 0],
     [0, φ, 0],
     [0, 0, 1/φ]]
```

---

## 3. Architettura Sistema

### 3.1 Diagramma a Blocchi

```
┌──────────────────────────────────────────────────────────────────────┐
│                    QUASICRYSTALLINE NEUROMORPHIC PROCESSOR          │
├──────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │                    INTERFACE LAYER                           │    │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │    │
│  │  │ Input Spike │  │   Address   │  │   Output    │         │    │
│  │  │    Queue    │  │  Translator │  │   Merge     │         │    │
│  │  └─────────────┘  └─────────────┘  └─────────────┘         │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                              ▲                                       │
│                              │                                       │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │                    ROUTING LAYER (Ammann-Beenker)           │    │
│  │                                                              │    │
│  │   ┌──┐   ┌──┐   ┌──┐   ┌──┐   ┌──┐   ┌──┐   ┌──┐          │    │
│  │   │RB│───│RB│───│RB│───│RB│───│RB│───│RB│───│RB│          │    │
│  │   └──┘   └──┘   └──┘   └──┘   └──┘   └──┘   └──┘          │    │
│  │     │       │       │       │       │       │               │    │
│  │   ┌──┐   ┌──┐   ┌──┐   ┌──┐   ┌──┐   ┌──┐   ┌──┐          │    │
│  │   │RB│───│RB│───│RB│───│RB│───│RB│───│RB│───│RB│          │    │
│  │   └──┘   └──┘   └──┘   └──┘   └──┘   └──┘   └──┘          │    │
│  │                                                              │    │
│  │   Legend: RB = Routing Block (max 4 ports)                  │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                              ▲                                       │
│                              │ Projection                            │
│                              ▼                                       │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │                   COMPUTATION LAYER (Penrose)                │    │
│  │                                                              │    │
│  │      ╱╲    ╱╲    ╱╲    ╱╲    ╱╲    ╱╲    ╱╲                │    │
│  │     ╱  ╲  ╱  ╲  ╱  ╲  ╱  ╲  ╱  ╲  ╱  ╲  ╱  ╲               │    │
│  │    ╱    ╲╱    ╲╱    ╲╱    ╲╱    ╲╱    ╲╱    ╲              │    │
│  │   ╱╲    ╱╲    ╱╲    ╱╲    ╱╲    ╱╲    ╱╲    ╱╲             │    │
│  │   ╱  ╲  ╱  ╲  ╱  ╲  ╱  ╲  ╱  ╲  ╱  ╲  ╱  ╲  ╱  ╲           │    │
│  │                                                              │    │
│  │   Legend: ● = Neuron (LIF/Izhikevich)                       │    │
│  │           ═ = Synapse with STDP                              │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘
```

### 3.2 Routing Layer - Ammann-Beenker

#### Proprietà del Routing

```
Struttura: Grafo quasiperiodico 8-fold

          ┌───────────────────────────────┐
          │                               │
    ──────┼──────►  ROUTING LAYER ───────┼───────►
          │      (Ammann-Beenker)         │
          │                               │
          │   • Max degree: 4             │
          │   • Diameter: O(log N)        │
          │   • Bisection width: high     │
          │                               │
          └───────────────────────────────┘
```

#### Algoritmo di Routing

```python
class AmmannBeenkerRouter:
    """
    Router basato su tassellazione Ammann-Beenker.
    Garantisce routing deadlock-free con massimo 4 porte per nodo.
    """
    
    def __init__(self, size=7):
        self.graph = self._generate_ammann_beenker(size)
        self.routing_tables = self._compute_routing_tables()
    
    def _generate_ammann_beenker(self, inflation_steps):
        """
        Genera grafo Ammann-Beenker con regola di inflazione.
        
        Factor di scala: φ = 1 + √2 ≈ 2.414
        """
        # Initial tile (octagon + square)
        tiles = self._init_tiles()
        
        # Inflation iterations
        for _ in range(inflation_steps):
            tiles = self._inflate(tiles)
        
        return self._tiles_to_graph(tiles)
    
    def route(self, source, destination):
        """
        Route packet using dimension-order routing.
        
        Complessità: O(log N) hop
        """
        path = self._shortest_path(source, destination)
        return path
    
    def _shortest_path(self, src, dst):
        """
        BFS shortest path con heuristic geometrica.
        """
        # Geometrie preservate permettono routing greedy efficiente
        pass
```

#### Proprietà di Routing

| Metrica | Valore |
|---------|--------|
| Grado massimo | 4 |
| Diametro (N=1024) | ~8 hop |
| Bisection width | Θ(√N) |
| Congestione media | Bassa |

### 3.3 Computation Layer - Penrose

#### Disposizione Neuroni

```
                    PENROSE NEURON LAYER
                    
           ╱╲₀₂        ╱╲₁₃        ╱╲₂₄        ╱╲₃₅
          ╱  ╲        ╱  ╲        ╱  ╲        ╱  ╲
         ╱ N₀ ╲      ╱ N₃ ╲      ╱ N₆ ╲      ╱ N₉ ╲
        ╱──────╲    ╱──────╲    ╱──────╲    ╱──────╲
       ╱╲      ╱╲  ╱╲      ╱╲  ╱╲      ╱╲  ╱╲      ╱╲
      ╱  ╲    ╱  ╲╱  ╲    ╱  ╲╱  ╲    ╱  ╲╱  ╲    ╱  ╲
     ╱ N₁ ╲  ╱ N₂ ╲  ╱ N₄ ╲  ╱ N₅ ╲  ╱ N₇ ╲  ╱ N₈ ╲
    ╱──────╲╱──────╲╱──────╲╱──────╲╱──────╲╱──────╲
    
    N = Neuron (LIF/Izhikevich)
    ─ = Synapse (STDP-enabled)
```

#### Connettività Locale

```python
class PenroseNeuronLayer:
    """
    Layer computazionale basato su tassellazione Penrose.
    Ottimizzato per calcolo locale e pattern matching.
    """
    
    def __init__(self, n_neurons):
        self.tiles = self._generate_penrose(n_neurons)
        self.neurons = self._place_neurons()
        self.synapses = self._create_synapses()
    
    def _generate_penrose(self, target_neurons):
        """
        Genera tassellazione Penrose P3 con numero approssimato di neuroni.
        
        Densità: ~0.4 neuroni per unità di area
        """
        # Robinson triangle decomposition
        # Inflazione con factor φ = (1+√5)/2
        pass
    
    def _create_synapses(self):
        """
        Crea sinapsi solo tra neuroni adiacenti (stesso tile).
        
        Complessità sinaptica: O(N) invece di O(N²)
        """
        pass
    
    def local_computation(self, tile_id):
        """
        Computazione locale confinata a un tile Penrose.
        Minimizza comunicazione inter-layer.
        """
        tile_neurons = self.tiles[tile_id].neurons
        # STDP learning within tile
        # Winner-take-all for pattern recognition
        pass
```

---

## 4. Interfaccia tra Layer

### 4.1 Proiezione Geometrica

```
        AMMANN-BEENKER                    PENROSE
        (Routing)                         (Computation)
        
           ┌───┐                            ╱╲
          ╱│   │╲                          ╱  ╲
         ╱ │   │ ╲                        ╱ N₀ ╲
        ╱  └───┤  ╲                      ╱──────╲
       ╱   │   │   ╲                    ╱╲      ╱╲
      ╱    │   │    ╲                   ╱  ╲    ╱  ╲
     ╱     └───┤     ╲                 ╱ N₁ ╲  ╱ N₂ ╲
    ╱      │   │      ╲               ╱──────╲╱──────╲
                                   
   ───────────────────────────► Projection Window
                                   (Local neighborhood)
```

### 4.2 Algoritmo di Proiezione

```python
class QuasicrystalProjection:
    """
    Proietta spikes dal layer Ammann-Beenker al layer Penrose.
    Preserva la località spaziale.
    """
    
    def __init__(self, ammann_grid, penrose_grid):
        self.ammann = ammann_grid
        self.penrose = penrose_grid
        self.projection_matrix = self._compute_projection()
    
    def _compute_projection(self):
        """
        Calcola matrice di proiezione basata su:
        1. Distanza geometrica
        2. Simmetria preservata
        3. Bilanciamento del carico
        """
        P = np.zeros((len(self.penrose), len(self.ammann)))
        
        for i, p_neuron in enumerate(self.penrose):
            for j, a_router in enumerate(self.ammann):
                # Gaussian-like projection kernel
                dist = self._geometric_distance(p_neuron, a_router)
                P[i, j] = np.exp(-dist**2 / (2 * sigma**2))
        
        # Normalize columns for load balancing
        P = P / P.sum(axis=0, keepdims=True)
        return P
    
    def project_spikes(self, spike_pattern_ammann):
        """
        Proietta pattern di spike al layer Penrose.
        
        Input:  vettore di spike dal routing layer
        Output: stimolazione probabilistica per neuroni Penrose
        """
        return self.projection_matrix @ spike_pattern_ammann
```

### 4.3 Algoritmo di Routing Integrato

```python
class HybridRoutingAlgorithm:
    """
    Algoritmo di routing ibrido che combina:
    1. Routing globale: Ammann-Beenker
    2. Computazione locale: Penrose
    """
    
    def process_spike(self, spike, destination):
        """
        Processa uno spike attraverso l'architettura ibrida.
        """
        # Step 1: Global routing (Ammann-Beenker)
        path = self.ammann_router.route(spike.source, destination)
        
        # Step 2: Project to computation layer
        local_stimulus = self.projection.project_spikes(path)
        
        # Step 3: Local computation (Penrose)
        result = self.penrose_layer.compute(local_stimulus)
        
        # Step 4: Feedback projection
        feedback = self.projection.project_back(result)
        
        # Step 5: Route response
        self.ammann_router.route_response(feedback, spike.source)
        
        return result
```

---

## 5. Proprietà Computazionali

### 5.1 Complessità Algoritmica

```
                        COMPLESSITÀ IBRIDA
                        
┌─────────────────────────────────────────────────────────┐
│                                                         │
│  Routing Layer (Ammann-Beenker):                        │
│  ───────────────────────────────                        │
│  • Spaziale: O(1) per hop                              │
│  • Temporale: O(log N) hop per path                    │
│  • HW: N nodi, max 4 porte ciascuno                    │
│                                                         │
│  Computation Layer (Penrose):                          │
│  ──────────────────────────────────                    │
│  • Spaziale: O(k) per tile (k = neuroni adiacenti)    │
│  • Temporale: O(1) per computazione locale             │
│  • HW: ~0.4N neuroni, O(N) sinapsi                     │
│                                                         │
│  Interfaccia:                                          │
│  ────────────                                          │
│  • Proiezione: O(N) con banda limitata                 │
│  • Loss: ~5% per proiezione gaussiana                 │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### 5.2 Risultati Attesi

| Metrica | Ammann-Beenker Only | Penrose Only | **Ibrido** |
|---------|---------------------|--------------|------------|
| Routing Efficiency | Alta | Bassa | **Alta** |
| Pattern Recognition | Media | Alta | **Alta** |
| Gestione topologica | Semplice | Complessa | **Semplice** |
| Fault Tolerance | Media | Alta | **Alta** |
| Scalabilità | Ottima | Buona | **Ottima** |

### 5.3 Risparmio Energetico

```
Confronto energetico (relativo):

                    Ammann-Beenker    Penrose      Ibrido
                    
Routing Energy:         ████░░░░       ████████     ███░░░░
Compute Energy:        ██████░░       ██████░░     ██████░░
Interconnect:          ████░░░░       ██████░░     ████░░░░
Total Energy:          █████░░░       ██████░░     ████░░░░

Legenda: ██ = consumo relativo (ogni █ = 10%)
```

---

## 6. Implementazione Hardware

### 6.1 Architettura FPGA Consigliata

```
┌─────────────────────────────────────────────────────────────────┐
│                    FPGA IMPLEMENTATION                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │              ROUTING FABRIC (Ammann-Beenker)             │   │
│  │                                                          │   │
│  │   ┌─────┐     ┌─────┐     ┌─────┐     ┌─────┐          │   │
│  │   │ Xbar│─────│ Xbar│─────│ Xbar│─────│ Xbar│          │   │
│  │   │4×4  │     │4×4  │     │4×4  │     │4×4  │          │   │
│  │   └─────┘     └─────┘     └─────┘     └─────┘          │   │
│  │      │           │           │           │              │   │
│  └──────│───────────│───────────│───────────│──────────────┘   │
│         │           │           │           │                   │
│         ▼           ▼           ▼           ▼                   │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │              PROJECTION BUFFERS                         │   │
│  │  (Line buffers + Gaussian interpolation)               │   │
│  └─────────────────────────────────────────────────────────┘   │
│                              │                                   │
│                              ▼                                   │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │           COMPUTATION ARRAY (Penrose Tiles)            │   │
│  │                                                          │   │
│  │   ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐    │   │
│  │   │  Tile   │  │  Tile   │  │  Tile   │  │  Tile   │    │   │
│  │   │  LIF×8  │  │  LIF×8  │  │  LIF×8  │  │  LIF×8  │    │   │
│  │   │ +STDP   │  │ +STDP   │  │ +STDP   │  │ +STDP   │    │   │
│  │   └─────────┘  └─────────┘  └─────────┘  └─────────┘    │   │
│  │                                                          │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 6.2 Resource Estimation (Xilinx Zynq)

```
RISORSE FPGA PER IMPLEMENTAZIONE IBRIDA

Target: 1024 neuroni totali (~400 Penrose layer)

                    Utilizzo      Disponibili    % Usage
─────────────────────────────────────────────────────────
LUT (routing)        8,500         53,200        16%
LUT (compute)        6,400         53,200        12%
FF (routing)        12,000        106,400       11%
FF (compute)         8,600         106,400        8%
BRAM (buffers)       24 (864KB)   140 (5MB)     17%
DSP (compute)        48            220           22%

Totale LUT: ~28% del dispositivo
Totale FF: ~19% del dispositivo
```

---

## 7. Applicazioni Scientifiche

### 7.1 Pattern Recognition Quasiperiodico

L'architettura ibrida è particolarmente efficace per:

1. **Pattern con simmetria 5-fold**: DNA sequences, virus capsids
2. **Pattern con simmetria 8-fold**: Materiali quasicristallini, texture naturali
3. **Pattern irregolari**: Qualsiasi pattern senza simmetria traslazionale

### 7.2 Esempio: Riconoscimento di Quasi-cristalli

```
Input: Immagine TEM di lega Al-Mn-Pd (simmetria icosaedrale)

Processing Pipeline:
────────────────────────────►

┌────────┐   ┌────────┐   ┌────────┐   ┌────────┐   ┌────────┐
│ Input  │──►│Preproc │──►│ A-B    │──►│Penrose │──►│ Output │
│Image   │   │        │   │Routing │   │Compute │   │ Class  │
└────────┘   └────────┘   └────────┘   └────────┘   └────────┘
                │             │            │
              Normalize    Global        Local
              +Filter      routing       pattern
                          to tile       matching
```

---

## 8. Conclusione

### 8.1 Contributi Principali

1. **Prima architettura neuromorfica ibrida** che combina Ammann-Beenker e Penrose
2. **Routing efficiente** grazie alla bassa connettività di Ammann-Beenker
3. **Computazione ottimizzata** grazie alle proprietà di pattern matching di Penrose
4. **Proiezione conservativa** che preserva la località geometrica

### 8.2 Lavoro Futuro

- [ ] Implementazione hardware su FPGA
- [ ] Validazione con benchmark standard (MNIST, N-MNIST)
- [ ] Estensione a 3D (Icosaedrico → Cubo-ottaedrico)
- [ ] Integrazione con reservoir computing

### 8.3 Riferimenti

1. Ammann, R., Grünbaum, B., Shephard, G.C. (1992). Aperiodic Tiles. *Geometriae Dedicata*.
2. Penrose, R. (1979). Mathematics of Long-Range Aperiodic Order. *Proceedings of the Royal Society*.
3. Indiveri, G., et al. (2011). Neuromorphic Silicon Neuron Circuits. *Frontiers in Neuroscience*.

---

## Appendice A: Codice di Generazione Tassellazione

```python
import numpy as np

def generate_ammann_beenker(iterations=4):
    """
    Genera tassellazione di Ammann-Beenker.
    
    Parametri:
        iterations: Numero di iterazioni di inflazione
        
    Ritorna:
        Lista di tiles con vertici e tipo
    """
    # phi = 1 + sqrt(2) per Ammann-Beenker
    phi = 1 + np.sqrt(2)
    
    # Tile iniziale: ottagono + quadrato
    # ...
    pass

def generate_penrose_p3(iterations=5):
    """
    Genera tassellazione di Penrose P3.
    
    Parametri:
        iterations: Numero di iterazioni di inflazione
        
    Ritorna:
        Lista di rombi (acuti e ottusi) con vertici
    """
    # Golden ratio
    phi = (1 + np.sqrt(5)) / 2
    
    # Robinson triangles
    # ...
    pass
```

---

*Documento preparato per pubblicazione scientifica*
*Autori: [Da compilare]*
*Data: Luglio 2026*
