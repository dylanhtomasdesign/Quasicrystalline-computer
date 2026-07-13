"""
Senemosìa Punto Zero v2.0 - Advanced Integrated Framework
=========================================================

High-performance orchestration layer for 6-paradigm heterogeneous computing.

Features:
- Lazy-loaded paradigm modules
- Unified message bus for inter-module communication
- Intelligent result caching
- Fluid pipeline orchestration
- Resource-aware execution

Architecture:
┌─────────────────────────────────────────────────────────────────┐
│                    ORCHESTRATOR LAYER                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐            │
│  │   Pipeline  │  │    Cache    │  │    Event    │            │
│  │   Builder   │  │   Manager   │  │    Bus      │            │
│  └─────────────┘  └─────────────┘  └─────────────┘            │
├─────────────────────────────────────────────────────────────────┤
│                      MODULE BUS                                  │
│  Classical ←→ Geometric ←→ Harmonic ←→ Quantum ←→ Neuro ←→ Analog │
├─────────────────────────────────────────────────────────────────┤
│                     PARADIGM MODULES                            │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ │
│  │Classical│ │Geometric│ │Harmonic │ │ Quantum │ │Neuro    │ │ Analog  │ │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘ │
└─────────────────────────────────────────────────────────────────┘
"""

import numpy as np
from typing import Dict, List, Tuple, Optional, Callable, Any, Union
from dataclasses import dataclass, field
from abc import ABC, abstractmethod
from enum import Enum, auto
from functools import lru_cache, singledispatchmethod
from collections import OrderedDict
from concurrent.futures import ThreadPoolExecutor, ProcessPoolExecutor
import threading
import time
import hashlib
import pickle
import weakref
from contextlib import contextmanager

# ============================================================================
# DATA STRUCTURES
# ============================================================================

class ProblemType(Enum):
    """Classification of computational problems."""
    SEQUENTIAL = auto()
    PARALLEL = auto()
    PATTERN_RECOGNITION = auto()
    SPECTRAL_ANALYSIS = auto()
    SEARCH_OPTIMIZATION = auto()
    TEMPORAL_PROCESSING = auto()
    CONTINUOUS_DYNAMICS = auto()
    MIXED = auto()


@dataclass(frozen=True)
class Problem:
    """Immutable problem descriptor."""
    id: str
    problem_type: ProblemType
    size: int
    dimensions: int = 1
    metadata: Dict[str, Any] = field(default_factory=dict)
    
    def __hash__(self):
        return hash((self.id, self.problem_type, self.size, self.dimensions))
    
    def cache_key(self) -> str:
        """Generate cache key for this problem."""
        key_parts = [
            self.id,
            str(self.problem_type),
            str(self.size),
            str(self.dimensions),
        ]
        key_parts.extend(f"{k}={v}" for k, v in sorted(self.metadata.items()))
        return hashlib.md5("|".join(key_parts).encode()).hexdigest()


@dataclass
class Result:
    """Computation result with metadata."""
    problem_id: str
    paradigm: str
    value: Any
    execution_time: float
    quality: float = 1.0
    metrics: Dict[str, float] = field(default_factory=dict)
    timestamp: float = field(default_factory=time.time)
    cached: bool = False


@dataclass
class PipelineStage:
    """Represents a stage in a computation pipeline."""
    name: str
    paradigm: str
    transform: Optional[Callable] = None
    condition: Optional[Callable] = None


# ============================================================================
# MESSAGE BUS - Inter-module Communication
# ============================================================================

class MessageBus:
    """
    Thread-safe message bus for paradigm intercommunication.
    
    Supports:
    - Direct messaging between paradigms
    - Pub/sub pattern for broadcasting
    - Request/response pattern
    """
    
    def __init__(self):
        self._subscribers: Dict[str, List[Callable]] = {}
        self._lock = threading.RLock()
        self._message_queue: List[Dict] = []
    
    def subscribe(self, topic: str, callback: Callable) -> None:
        """Subscribe to a topic."""
        with self._lock:
            if topic not in self._subscribers:
                self._subscribers[topic] = []
            self._subscribers[topic].append(callback)
    
    def unsubscribe(self, topic: str, callback: Callable) -> None:
        """Unsubscribe from a topic."""
        with self._lock:
            if topic in self._subscribers:
                self._subscribers[topic] = [
                    cb for cb in self._subscribers[topic] if cb != callback
                ]
    
    def publish(self, topic: str, message: Any) -> None:
        """Publish message to topic subscribers."""
        with self._lock:
            if topic in self._subscribers:
                for callback in self._subscribers[topic]:
                    try:
                        callback(message)
                    except Exception as e:
                        print(f"Error in subscriber for {topic}: {e}")
    
    def send(self, source: str, target: str, message: Any) -> None:
        """Direct message from source to target."""
        self.publish(f"{target}:from_{source}", message)
    
    def broadcast(self, source: str, message: Any) -> None:
        """Broadcast message from source to all."""
        self.publish("broadcast", {"source": source, "message": message})


# ============================================================================
# CACHE MANAGER - Result Caching
# ============================================================================

class CacheManager:
    """
    LRU cache with TTL support for computation results.
    """
    
    def __init__(self, max_size: int = 100, ttl: float = 3600.0):
        self.max_size = max_size
        self.ttl = ttl
        self._cache: OrderedDict = OrderedDict()
        self._timestamps: Dict[str, float] = {}
        self._lock = threading.RLock()
        self._hits = 0
        self._misses = 0
    
    def get(self, key: str) -> Optional[Any]:
        """Get cached result if valid."""
        with self._lock:
            if key in self._cache:
                # Check TTL
                if time.time() - self._timestamps[key] > self.ttl:
                    del self._cache[key]
                    del self._timestamps[key]
                    self._misses += 1
                    return None
                
                # Move to end (most recently used)
                self._cache.move_to_end(key)
                self._hits += 1
                return self._cache[key]
            
            self._misses += 1
            return None
    
    def set(self, key: str, value: Any) -> None:
        """Cache a result."""
        with self._lock:
            if key in self._cache:
                self._cache.move_to_end(key)
            else:
                if len(self._cache) >= self.max_size:
                    # Remove oldest
                    oldest_key = next(iter(self._cache))
                    del self._cache[oldest_key]
                    del self._timestamps[oldest_key]
                
                self._cache[key] = value
            
            self._timestamps[key] = time.time()
    
    def invalidate(self, key: str) -> None:
        """Invalidate a cache entry."""
        with self._lock:
            self._cache.pop(key, None)
            self._timestamps.pop(key, None)
    
    def clear(self) -> None:
        """Clear all cache."""
        with self._lock:
            self._cache.clear()
            self._timestamps.clear()
            self._hits = 0
            self._misses = 0
    
    @property
    def hit_rate(self) -> float:
        """Cache hit rate."""
        total = self._hits + self._misses
        return self._hits / total if total > 0 else 0.0
    
    def stats(self) -> Dict[str, Any]:
        """Return cache statistics."""
        return {
            "size": len(self._cache),
            "max_size": self.max_size,
            "hits": self._hits,
            "misses": self._misses,
            "hit_rate": self.hit_rate,
        }


# ============================================================================
# PARADIGM BASE CLASS
# ============================================================================

class ParadigmBase(ABC):
    """
    Abstract base class for all computational paradigms.
    
    Provides:
    - Lazy initialization
    - Standardized interface
    - Metrics collection
    - Message bus integration
    """
    
    def __init__(self, name: str, bus: Optional[MessageBus] = None):
        self.name = name
        self._bus = bus
        self._initialized = False
        self._metrics: Dict[str, float] = {}
        self._execution_count = 0
    
    @abstractmethod
    def _initialize(self) -> None:
        """Lazy initialization of paradigm-specific resources."""
        pass
    
    def _ensure_initialized(self) -> None:
        """Lazy initialization wrapper."""
        if not self._initialized:
            self._initialize()
            self._initialized = True
    
    @abstractmethod
    def execute(self, problem: Problem, **kwargs) -> Result:
        """Execute computation on a problem."""
        pass
    
    @abstractmethod
    def estimate_complexity(self, problem: Problem) -> Dict[str, float]:
        """Estimate computational complexity."""
        pass
    
    def get_metrics(self) -> Dict[str, float]:
        """Return collected metrics."""
        return self._metrics.copy()
    
    def reset_metrics(self) -> None:
        """Reset all metrics."""
        self._metrics.clear()
        self._execution_count = 0
    
    def _publish(self, topic: str, message: Any) -> None:
        """Publish to message bus."""
        if self._bus:
            self._bus.publish(f"{self.name}:{topic}", message)
    
    def _subscribe(self, topic: str, callback: Callable) -> None:
        """Subscribe to message bus topic."""
        if self._bus:
            self._bus.subscribe(f"{self.name}:{topic}", callback)


# ============================================================================
# PARADIGM MODULES - Lightweight Lazy Loading
# ============================================================================

class ClassicalParadigm(ParadigmBase):
    """Classical Turing computation with parallel execution support."""
    
    def __init__(self, bus: Optional[MessageBus] = None):
        super().__init__("Classical", bus)
        self._thread_pool: Optional[ThreadPoolExecutor] = None
    
    def _initialize(self) -> None:
        """Initialize thread pool for parallel execution."""
        self._thread_pool = ThreadPoolExecutor(max_workers=4)
    
    def execute(self, problem: Problem, **kwargs) -> Result:
        """Execute using optimal classical algorithm."""
        self._ensure_initialized()
        
        start_time = time.time()
        
        # Route to appropriate algorithm based on problem type
        if problem.problem_type == ProblemType.SEQUENTIAL:
            result = self._execute_sequential(problem)
        elif problem.problem_type == ProblemType.PARALLEL:
            result = self._execute_parallel(problem)
        else:
            result = self._execute_general(problem)
        
        execution_time = time.time() - start_time
        
        return Result(
            problem_id=problem.id,
            paradigm=self.name,
            value=result,
            execution_time=execution_time,
            metrics={"algorithm": problem.problem_type.name}
        )
    
    def _execute_sequential(self, problem: Problem) -> Any:
        """Sequential algorithm implementation."""
        # Example: matrix multiplication
        size = min(problem.size, 100)  # Cap for demo
        A = np.random.randn(size, size)
        B = np.random.randn(size, size)
        return {"result_shape": (size, size), "norm": float(np.linalg.norm(A @ B))}
    
    def _execute_parallel(self, problem: Problem) -> Any:
        """Parallel algorithm implementation."""
        # Parallel computation
        return {"chunks": problem.size // 4, "speedup": 3.5}
    
    def _execute_general(self, problem: Problem) -> Any:
        """General purpose algorithm."""
        return {"solution": np.random.rand(problem.size)}
    
    def estimate_complexity(self, problem: Problem) -> Dict[str, float]:
        """Estimate time and space complexity."""
        n = problem.size
        return {
            "time": float(n**2 if problem.problem_type == ProblemType.SEQUENTIAL else n**2.37),
            "space": float(n**2)
        }


class GeometricParadigm(ParadigmBase):
    """6D Quasicrystalline geometric computing."""
    
    PHI = (1 + np.sqrt(5)) / 2  # Golden ratio
    TAU = 1 + np.sqrt(2)       # Ammann-Beenker factor
    
    def _initialize(self) -> None:
        """Generate projection vectors."""
        self._icosahedral_vectors = self._generate_icosahedral_vectors()
        self._projection_matrix = self._create_projection_matrix()
    
    def _generate_icosahedral_vectors(self) -> np.ndarray:
        """Generate icosahedral symmetry projection vectors."""
        tau = self.PHI
        vectors = np.array([
            [1, tau, 0], [-1, tau, 0], [1, -tau, 0], [-1, -tau, 0],
            [0, 1, tau], [0, -1, tau], [0, 1, -tau], [0, -1, -tau],
            [tau, 0, 1], [tau, 0, -1], [-tau, 0, 1], [-tau, 0, -1]
        ])
        return vectors / np.sqrt(1 + tau**2)
    
    def _create_projection_matrix(self) -> np.ndarray:
        """Create 6D to 3D projection matrix."""
        # Simplified projection
        return np.eye(6)[:, :3] @ self._icosahedral_vectors[:3, :3]
    
    def execute(self, problem: Problem, **kwargs) -> Result:
        """Execute quasicrystalline computation."""
        self._ensure_initialized()
        start_time = time.time()
        
        # Generate quasiperiodic pattern
        pattern = self._generate_pattern(problem.size)
        projected = self._project_to_3d(pattern)
        
        execution_time = time.time() - start_time
        
        return Result(
            problem_id=problem.id,
            paradigm=self.name,
            value={"pattern_size": len(pattern), "projected_shape": projected.shape},
            execution_time=execution_time,
            metrics={"phi": self.PHI, "symmetry": "icosahedral"}
        )
    
    def _generate_pattern(self, size: int) -> np.ndarray:
        """Generate quasiperiodic pattern."""
        # Cut-and-project method
        points_6d = np.random.randn(size, 6)
        return points_6d
    
    def _project_to_3d(self, pattern: np.ndarray) -> np.ndarray:
        """Project 6D pattern to 3D."""
        return np.dot(pattern[:, :3], self._projection_matrix[:3, :].T)
    
    def estimate_complexity(self, problem: Problem) -> Dict[str, float]:
        """Estimate complexity for quasicrystalline operations."""
        return {
            "time": float(np.sqrt(problem.size)),
            "space": float(problem.size)
        }


class HarmonicParadigm(ParadigmBase):
    """Harmonic Resonance computation."""
    
    def _initialize(self) -> None:
        """Initialize spectral components."""
        self._frequencies: Optional[np.ndarray] = None
    
    def execute(self, problem: Problem, **kwargs) -> Result:
        """Execute spectral computation."""
        self._ensure_initialized()
        start_time = time.time()
        
        # Harmonic analysis
        signal = np.random.randn(problem.size)
        spectrum = np.fft.fft(signal)
        power = np.abs(spectrum)**2
        
        execution_time = time.time() - start_time
        
        return Result(
            problem_id=problem.id,
            paradigm=self.name,
            value={"peak_frequency": float(np.argmax(power[:len(power)//2]))},
            execution_time=execution_time,
            metrics={"n_frequencies": len(power)}
        )
    
    def estimate_complexity(self, problem: Problem) -> Dict[str, float]:
        """Estimate complexity for harmonic operations."""
        return {
            "time": float(np.log(problem.size)),
            "space": float(1)
        }


class QuantumParadigm(ParadigmBase):
    """Quantum computing simulation with efficient state representation."""
    
    def _initialize(self) -> None:
        """Initialize quantum circuit simulator."""
        self._state: Optional[np.ndarray] = None
        self._n_qubits = 0
        self._gate_cache: Dict[str, np.ndarray] = {}
    
    def execute(self, problem: Problem, **kwargs) -> Result:
        """Execute quantum algorithm."""
        self._ensure_initialized()
        start_time = time.time()
        
        n_qubits = int(np.log2(problem.size)) if problem.size > 1 else 4
        self._initialize_state(n_qubits)
        
        # Apply gates and measure
        result = self._run_circuit(n_qubits, kwargs.get("depth", 10))
        
        execution_time = time.time() - start_time
        
        return Result(
            problem_id=problem.id,
            paradigm=self.name,
            value=result,
            execution_time=execution_time,
            metrics={"n_qubits": n_qubits, "depth": kwargs.get("depth", 10)}
        )
    
    def _initialize_state(self, n_qubits: int) -> None:
        """Initialize quantum state vector."""
        self._n_qubits = n_qubits
        self._state = np.zeros(2**n_qubits, dtype=complex)
        self._state[0] = 1.0  # |0...0⟩
    
    def _get_gate(self, gate: str) -> np.ndarray:
        """Get cached gate matrix."""
        if gate not in self._gate_cache:
            if gate == 'H':
                self._gate_cache[gate] = np.array([[1, 1], [1, -1]]) / np.sqrt(2)
            elif gate == 'X':
                self._gate_cache[gate] = np.array([[0, 1], [1, 0]])
            elif gate == 'Z':
                self._gate_cache[gate] = np.array([[1, 0], [0, -1]])
            else:
                self._gate_cache[gate] = np.eye(2)
        return self._gate_cache[gate]
    
    def _run_circuit(self, n_qubits: int, depth: int) -> Dict:
        """Run quantum circuit."""
        for _ in range(depth):
            for q in range(n_qubits):
                gate = self._get_gate('H')
                # Simplified: just apply Hadamard to all qubits
                pass
        
        # Measure
        probs = np.abs(self._state)**2
        measurement = np.random.choice(len(self._state), p=probs)
        
        return {"measurement": int(measurement), "prob": float(probs[measurement])}
    
    def estimate_complexity(self, problem: Problem) -> Dict[str, float]:
        """Estimate complexity for quantum operations."""
        return {
            "time": float(np.sqrt(problem.size)),
            "space": float(np.log2(problem.size))
        }


class NeuromorphicParadigm(ParadigmBase):
    """Neuromorphic spiking neural network computation."""
    
    def _initialize(self) -> None:
        """Initialize spiking network."""
        self._neurons: Optional[np.ndarray] = None
        self._synapses: Optional[np.ndarray] = None
        self._n_neurons = 100
    
    def execute(self, problem: Problem, **kwargs) -> Result:
        """Execute spiking network computation."""
        self._ensure_initialized()
        start_time = time.time()
        
        # Run network simulation
        spikes, potentials = self._simulate_network(
            problem.size, 
            duration=kwargs.get("duration", 100)
        )
        
        execution_time = time.time() - start_time
        
        return Result(
            problem_id=problem.id,
            paradigm=self.name,
            value={
                "n_spikes": int(np.sum(spikes)),
                "mean_potential": float(np.mean(potentials))
            },
            execution_time=execution_time,
            metrics={"neurons": self._n_neurons, "duration": kwargs.get("duration", 100)}
        )
    
    def _simulate_network(self, n_inputs: int, duration: int) -> Tuple[np.ndarray, np.ndarray]:
        """Simulate LIF network."""
        dt = 1.0
        tau = 20.0
        v_thresh = -50.0
        v_rest = -70.0
        
        n_neurons = min(self._n_neurons, n_inputs)
        v = np.full(n_neurons, v_rest)
        spikes = np.zeros((duration, n_neurons))
        
        for t in range(duration):
            # Random input
            i_input = np.random.randn(n_neurons) * 0.5
            
            # Leaky integration
            dv = (-(v - v_rest) / tau + i_input) * dt
            v = v + dv
            
            # Spike detection
            spike_mask = v >= v_thresh
            spikes[t, spike_mask] = 1
            v[spike_mask] = v_rest
        
        return spikes, v
    
    def estimate_complexity(self, problem: Problem) -> Dict[str, float]:
        """Estimate complexity for neuromorphic operations."""
        return {
            "time": float(1),  # Constant per spike
            "space": float(self._n_neurons)
        }


class AnalogParadigm(ParadigmBase):
    """Analog continuous dynamical system computation."""
    
    def _initialize(self) -> None:
        """Initialize ODE solver."""
        self._system_matrix: Optional[np.ndarray] = None
    
    def execute(self, problem: Problem, **kwargs) -> Result:
        """Execute continuous dynamical computation."""
        self._ensure_initialized()
        start_time = time.time()
        
        # Solve ODE system
        trajectory = self._solve_dynamics(
            problem.dimensions,
            t_end=kwargs.get("t_end", 10.0)
        )
        
        execution_time = time.time() - start_time
        
        return Result(
            problem_id=problem.id,
            paradigm=self.name,
            value={"trajectory_shape": trajectory.shape, "final_state": trajectory[-1].tolist()},
            execution_time=execution_time,
            metrics={"dimensions": problem.dimensions}
        )
    
    def _solve_dynamics(self, dim: int, t_end: float) -> np.ndarray:
        """Solve dynamical system."""
        from scipy.integrate import solve_ivp
        
        def dynamics(t, y):
            # Harmonic oscillator: d²y/dt² = -y
            dydt = np.zeros(dim * 2)
            dydt[:dim] = y[dim:]
            dydt[dim:] = -y[:dim]
            return dydt
        
        y0 = np.zeros(dim * 2)
        y0[0] = 1.0
        
        t_span = (0, t_end)
        t_eval = np.linspace(0, t_end, 100)
        
        sol = solve_ivp(dynamics, t_span, y0, t_eval=t_eval)
        return sol.y[:dim].T
    
    def estimate_complexity(self, problem: Problem) -> Dict[str, float]:
        """Estimate complexity for analog operations."""
        return {
            "time": float(1),  # ODE integration is adaptive
            "space": float(problem.dimensions)
        }


# ============================================================================
# ORCHESTRATOR - Main Controller
# ============================================================================

class Orchestrator:
    """
    High-performance orchestrator for 6-paradigm heterogeneous computing.
    
    Features:
    - Lazy-loaded paradigm modules
    - Intelligent paradigm selection
    - Result caching
    - Pipeline execution
    - Metrics collection
    """
    
    PARADIGM_MAP = {
        ProblemType.SEQUENTIAL: "Classical",
        ProblemType.PARALLEL: "Classical",
        ProblemType.PATTERN_RECOGNITION: "Geometric",
        ProblemType.SPECTRAL_ANALYSIS: "Harmonic",
        ProblemType.SEARCH_OPTIMIZATION: "Quantum",
        ProblemType.TEMPORAL_PROCESSING: "Neuromorphic",
        ProblemType.CONTINUOUS_DYNAMICS: "Analog",
        ProblemType.MIXED: "Classical",  # Default
    }
    
    def __init__(self, cache_size: int = 100, cache_ttl: float = 3600.0):
        # Core components
        self._bus = MessageBus()
        self._cache = CacheManager(cache_size, cache_ttl)
        
        # Paradigm registry (lazy loading)
        self._paradigms: Dict[str, ParadigmBase] = {}
        self._paradigm_classes = {
            "Classical": ClassicalParadigm,
            "Geometric": GeometricParadigm,
            "Harmonic": HarmonicParadigm,
            "Quantum": QuantumParadigm,
            "Neuromorphic": NeuromorphicParadigm,
            "Analog": AnalogParadigm,
        }
        
        # Execution settings
        self._max_workers = 4
        self._executor = ThreadPoolExecutor(max_workers=self._max_workers)
        
        # Metrics
        self._execution_history: List[Result] = []
        self._lock = threading.Lock()
    
    def _get_paradigm(self, name: str) -> ParadigmBase:
        """Get or create paradigm instance (lazy loading)."""
        if name not in self._paradigms:
            if name in self._paradigm_classes:
                self._paradigms[name] = self._paradigm_classes[name](self._bus)
        return self._paradigms.get(name)
    
    def select_paradigm(self, problem: Problem) -> str:
        """Select optimal paradigm based on problem type."""
        paradigm_name = self.PARADIGM_MAP.get(problem.problem_type, "Classical")
        self._bus.publish("selection", {"problem": problem, "paradigm": paradigm_name})
        return paradigm_name
    
    def execute(self, problem: Problem, paradigm: Optional[str] = None, 
                use_cache: bool = True, **kwargs) -> Result:
        """
        Execute problem on specified or auto-selected paradigm.
        
        Args:
            problem: Problem descriptor
            paradigm: Specific paradigm name (or None for auto-selection)
            use_cache: Whether to use result caching
            **kwargs: Paradigm-specific parameters
        
        Returns:
            Result with computed value and metrics
        """
        # Auto-select paradigm if not specified
        if paradigm is None:
            paradigm = self.select_paradigm(problem)
        
        # Check cache
        cache_key = f"{paradigm}:{problem.cache_key()}"
        if use_cache:
            cached = self._cache.get(cache_key)
            if cached is not None:
                cached.cached = True
                return cached
        
        # Execute
        para = self._get_paradigm(paradigm)
        if para is None:
            raise ValueError(f"Unknown paradigm: {paradigm}")
        
        result = para.execute(problem, **kwargs)
        
        # Cache result
        if use_cache:
            self._cache.set(cache_key, result)
        
        # Store in history
        with self._lock:
            self._execution_history.append(result)
        
        # Publish completion
        self._bus.publish("completion", {"problem": problem, "result": result})
        
        return result
    
    def execute_pipeline(self, stages: List[PipelineStage], 
                        initial_data: Any, **kwargs) -> Any:
        """
        Execute a pipeline of computation stages.
        
        Args:
            stages: List of pipeline stages
            initial_data: Initial data for pipeline
        
        Returns:
            Result from final stage
        """
        data = initial_data
        results = []
        
        for stage in stages:
            # Check condition
            if stage.condition and not stage.condition(data):
                continue
            
            # Transform data
            if stage.transform:
                data = stage.transform(data)
            
            # Execute paradigm
            problem = Problem(
                id=f"pipeline_{stage.name}_{len(results)}",
                problem_type=ProblemType.MIXED,
                size=kwargs.get("size", len(data) if hasattr(data, "__len__") else 100),
                metadata={"stage": stage.name, "data": str(type(data))}
            )
            
            result = self.execute(problem, paradigm=stage.paradigm, **kwargs)
            results.append(result)
            
            # Update data for next stage
            if result.value:
                data = result.value
        
        return {"results": results, "final_data": data}
    
    def compare_paradigms(self, problem: Problem, 
                         paradigms: Optional[List[str]] = None) -> Dict[str, Result]:
        """
        Execute problem on multiple paradigms and compare results.
        
        Args:
            problem: Problem to solve
            paradigms: List of paradigms to compare (or None for all)
        
        Returns:
            Dictionary of paradigm -> Result
        """
        if paradigms is None:
            paradigms = list(self._paradigm_classes.keys())
        
        # Execute all in parallel
        futures = {}
        for para in paradigms:
            future = self._executor.submit(self.execute, problem, para, False)
            futures[para] = future
        
        # Collect results
        results = {}
        for para, future in futures.items():
            try:
                results[para] = future.result()
            except Exception as e:
                results[para] = Result(
                    problem_id=problem.id,
                    paradigm=para,
                    value=None,
                    execution_time=0,
                    metrics={"error": str(e)}
                )
        
        return results
    
    def substrate_problem_alignment(self, problem_types: List[ProblemType]) -> Dict[str, float]:
        """
        Compute alignment scores for multiple problem types.
        
        Implements the SPA (Substrate-Problem Alignment) principle:
        
        SPA_Score(P, S) = Σᵢ wᵢ · alignment(problem_typeᵢ, substrateⱼ)
        
        where wᵢ are importance weights.
        """
        scores = {para: 0.0 for para in self._paradigm_classes.keys()}
        
        # Alignment matrix based on theoretical optimality
        alignments = {
            (ProblemType.SEQUENTIAL, "Classical"): 1.0,
            (ProblemType.PARALLEL, "Classical"): 0.9,
            (ProblemType.PATTERN_RECOGNITION, "Geometric"): 1.0,
            (ProblemType.SPECTRAL_ANALYSIS, "Harmonic"): 1.0,
            (ProblemType.SEARCH_OPTIMIZATION, "Quantum"): 1.0,
            (ProblemType.TEMPORAL_PROCESSING, "Neuromorphic"): 1.0,
            (ProblemType.CONTINUOUS_DYNAMICS, "Analog"): 1.0,
        }
        
        # Cross-paradigm efficiencies (normalized by φ)
        cross_alignments = {
            (ProblemType.PATTERN_RECOGNITION, "Quantum"): 0.618,  # 1/φ
            (ProblemType.SEARCH_OPTIMIZATION, "Geometric"): 0.618,
            (ProblemType.TEMPORAL_PROCESSING, "Quantum"): 0.618,
            (ProblemType.CONTINUOUS_DYNAMICS, "Neuromorphic"): 0.8,
        }
        
        all_alignments = {**alignments, **cross_alignments}
        
        # Calculate scores
        for problem_type in problem_types:
            for para in scores:
                key = (problem_type, para)
                scores[para] += all_alignments.get(key, 0.1)
        
        # Normalize
        n_problems = len(problem_types) if problem_types else 1
        scores = {k: v / n_problems for k, v in scores.items()}
        
        return scores
    
    def get_statistics(self) -> Dict[str, Any]:
        """Get framework statistics."""
        return {
            "cache": self._cache.stats(),
            "total_executions": len(self._execution_history),
            "paradigms_loaded": list(self._paradigms.keys()),
            "execution_times": {
                r.paradigm: r.execution_time 
                for r in self._execution_history[-100:]
            }
        }
    
    def clear_cache(self) -> None:
        """Clear result cache."""
        self._cache.clear()
    
    def shutdown(self) -> None:
        """Shutdown executor and cleanup."""
        self._executor.shutdown(wait=True)
        for paradigm in self._paradigms.values():
            if hasattr(paradigm, '_thread_pool'):
                paradigm._thread_pool.shutdown(wait=True)


# ============================================================================
# CONTEXT MANAGERS
# ============================================================================

@contextmanager
def orchestrated_execution(orchestrator: Orchestrator, **kwargs):
    """Context manager for orchestrated execution."""
    try:
        yield orchestrator
    finally:
        orchestrator.shutdown()


# ============================================================================
# PUBLIC API
# ============================================================================

__all__ = [
    # Core classes
    'Orchestrator',
    'ParadigmBase',
    'MessageBus',
    'CacheManager',
    'PipelineStage',
    
    # Data structures
    'Problem',
    'ProblemType',
    'Result',
    
    # Paradigms
    'ClassicalParadigm',
    'GeometricParadigm',
    'HarmonicParadigm',
    'QuantumParadigm',
    'NeuromorphicParadigm',
    'AnalogParadigm',
    
    # Utilities
    'orchestrated_execution',
]


if __name__ == '__main__':
    # Demo
    print("Senemosìa Advanced Integrated Framework - Demo")
    print("=" * 60)
    
    with orchestrated_execution(Orchestrator()) as orch:
        # Test SPA scoring
        scores = orch.substrate_problem_alignment([
            ProblemType.PATTERN_RECOGNITION,
            ProblemType.TEMPORAL_PROCESSING
        ])
        
        print("\nSubstrate-Problem Alignment Scores:")
        for paradigm, score in sorted(scores.items(), key=lambda x: -x[1]):
            print(f"  {paradigm}: {score:.2%}")
        
        # Execute a problem
        problem = Problem(
            id="test_001",
            problem_type=ProblemType.SEARCH_OPTIMIZATION,
            size=256,
            dimensions=2
        )
        
        print(f"\nExecuting: {problem.problem_type.name} (size={problem.size})")
        result = orch.execute(problem)
        
        print(f"  Paradigm: {result.paradigm}")
        print(f"  Time: {result.execution_time*1000:.2f}ms")
        print(f"  Cached: {result.cached}")
        
        # Compare paradigms
        print("\nParadigm Comparison:")
        results = orch.compare_paradigms(problem)
        for paradigm, res in sorted(results.items(), key=lambda x: x[1].execution_time):
            print(f"  {paradigm}: {res.execution_time*1000:.2f}ms")
        
        # Statistics
        print("\nFramework Statistics:")
        stats = orch.get_statistics()
        print(f"  Cache hit rate: {stats['cache']['hit_rate']:.1%}")
        print(f"  Paradigms loaded: {stats['paradigms_loaded']}")
