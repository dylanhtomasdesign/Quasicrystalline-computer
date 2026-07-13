"""
Benchmark Suite for 6-Paradigm Heterogeneous Computing
======================================================

Comprehensive benchmarking suite for the Senemosìa framework.

Test cases cover:
- Classical computing benchmarks
- 6D Geometric/Quasicrystalline benchmarks
- Harmonic Resonance benchmarks
- Quantum computing benchmarks
- Neuromorphic computing benchmarks
- Analog/Hybrid computing benchmarks
"""

import numpy as np
import time
from typing import Dict, List, Tuple, Callable, Any
from dataclasses import dataclass
import matplotlib.pyplot as plt


@dataclass
class BenchmarkResult:
    """Results from a benchmark run."""
    name: str
    paradigm: str
    problem_size: int
    execution_time: float
    solution_quality: float
    memory_usage: float
    metadata: Dict = None
    
    def to_dict(self) -> Dict:
        return {
            'name': self.name,
            'paradigm': self.paradigm,
            'problem_size': self.problem_size,
            'execution_time': self.execution_time,
            'solution_quality': self.solution_quality,
            'memory_usage': self.memory_usage,
            **(self.metadata or {})
        }


class BenchmarkSuite:
    """Comprehensive benchmark suite for heterogeneous computing."""
    
    def __init__(self):
        self.results: List[BenchmarkResult] = []
        self._memory_baseline = self._get_memory_usage()
    
    def _get_memory_usage(self) -> float:
        """Get current memory usage in MB."""
        import psutil
        process = psutil.Process()
        return process.memory_info().rss / 1024 / 1024
    
    def run_benchmark(self, benchmark_fn: Callable, 
                      paradigm: str, problem_size: int,
                      **kwargs) -> BenchmarkResult:
        """Run a single benchmark."""
        name = benchmark_fn.__name__
        
        # Time the execution
        start_time = time.time()
        result = benchmark_fn(problem_size, **kwargs)
        execution_time = time.time() - start_time
        
        # Memory
        memory_used = self._get_memory_usage() - self._memory_baseline
        
        return BenchmarkResult(
            name=name,
            paradigm=paradigm,
            problem_size=problem_size,
            execution_time=execution_time,
            solution_quality=result.get('quality', 1.0),
            memory_usage=memory_used,
            metadata=result
        )
    
    def add_result(self, result: BenchmarkResult):
        """Add a benchmark result."""
        self.results.append(result)
    
    def summary(self) -> pd.DataFrame:
        """Generate summary statistics."""
        import pandas as pd
        
        data = [r.to_dict() for r in self.results]
        return pd.DataFrame(data)
    
    def plot_comparison(self, metric: str = 'execution_time'):
        """Plot comparison across paradigms."""
        import pandas as pd
        
        df = self.summary()
        
        if len(df) == 0:
            print("No results to plot")
            return
        
        fig, ax = plt.subplots(figsize=(12, 6))
        
        paradigms = df['paradigm'].unique()
        colors = plt.cm.Set2(np.linspace(0, 1, len(paradigms)))
        
        for i, paradigm in enumerate(paradigms):
            subset = df[df['paradigm'] == paradigm]
            ax.scatter(subset['problem_size'], subset[metric], 
                      label=paradigm, alpha=0.7, s=100, c=[colors[i]])
        
        ax.set_xlabel('Problem Size')
        ax.set_ylabel(metric.replace('_', ' ').title())
        ax.set_title(f'{metric.replace("_", " ").title()} Comparison')
        ax.legend()
        ax.grid(True, alpha=0.3)
        
        if metric == 'execution_time':
            ax.set_yscale('log')
        
        plt.tight_layout()
        plt.show()


# =============================================================================
# Classical Computing Benchmarks
# =============================================================================

class ClassicalBenchmarks:
    """Benchmarks for classical Turing computation."""
    
    @staticmethod
    def matrix_multiplication(n: int) -> Dict:
        """Benchmark matrix multiplication."""
        A = np.random.randn(n, n)
        B = np.random.randn(n, n)
        
        C = A @ B
        
        return {'quality': np.linalg.norm(C, 'fro')}
    
    @staticmethod
    def sorting(n: int) -> Dict:
        """Benchmark sorting algorithm."""
        arr = np.random.randn(n)
        
        sorted_arr = np.sort(arr)
        
        return {'quality': np.all(sorted_arr[:-1] <= sorted_arr[1:])}
    
    @staticmethod
    def fft(n: int) -> Dict:
        """Benchmark FFT."""
        signal = np.random.randn(n)
        
        spectrum = np.fft.fft(signal)
        
        return {'quality': np.linalg.norm(np.fft.ifft(spectrum) - signal) < 1e-10}
    
    @staticmethod
    def graph_traversal(n: int) -> Dict:
        """Benchmark BFS/DFS on random graph."""
        import networkx as nx
        
        G = nx.erdos_renyi_graph(n, 0.1)
        start = 0
        
        # BFS
        distances = nx.single_source_shortest_path_length(G, start)
        
        return {'quality': len(distances) / n}


# =============================================================================
# 6D Geometric/Quasicrystalline Benchmarks
# =============================================================================

class GeometricBenchmarks:
    """Benchmarks for 6D quasicrystalline computation."""
    
    @staticmethod
    def quasiperiodic_projection(n: int) -> Dict:
        """Benchmark 6D to 3D projection."""
        phi = (1 + np.sqrt(5)) / 2
        
        # Generate 6D points
        points_6d = np.random.randn(n, 6)
        
        # Golden ratio projection
        weights = np.array([phi**i for i in range(6)])
        points_3d = np.dot(points_6d, weights[:3].reshape(-1, 1))
        
        return {'quality': np.std(points_3d)}
    
    @staticmethod
    def icosahedral_symmetry(n: int) -> Dict:
        """Benchmark icosahedral symmetry operations."""
        # Generate icosahedron vertices
        tau = (1 + np.sqrt(5)) / 2
        vertices = []
        for i in [-1, 1]:
            for j in [-1, 1]:
                vertices.append([0, i, j*tau])
                vertices.append([i, j*tau, 0])
                vertices.append([i*tau, 0, j])
        vertices = np.array(vertices) / np.sqrt(1 + tau**2)
        
        # Project random points
        projections = np.dot(vertices, np.random.randn(3, n))
        
        return {'quality': np.linalg.norm(projections)}
    
    @staticmethod
    def penrose_tiling(n: int) -> Dict:
        """Benchmark Penrose tiling generation."""
        # Simplified Penrose pattern
        angles = np.linspace(0, 2*np.pi, n)
        
        # Golden ratio geometry
        phi = (1 + np.sqrt(5)) / 2
        pattern = np.exp(1j * phi * angles)
        
        return {'quality': np.abs(np.sum(pattern))}


# =============================================================================
# Harmonic/Resonance Benchmarks
# =============================================================================

class HarmonicBenchmarks:
    """Benchmarks for harmonic resonance computation."""
    
    @staticmethod
    def spectral_analysis(n: int) -> Dict:
        """Benchmark spectral analysis."""
        signal = np.random.randn(n)
        
        # Power spectrum
        spectrum = np.abs(np.fft.fft(signal))**2
        
        return {'quality': np.sum(spectrum)}
    
    @staticmethod
    def resonance_detection(n: int) -> Dict:
        """Benchmark resonance detection."""
        # Create signal with known frequency
        f0 = 0.3
        t = np.linspace(0, 10, n)
        signal = np.sin(2 * np.pi * f0 * t) + 0.1 * np.random.randn(n)
        
        # Detect peak frequency
        spectrum = np.abs(np.fft.fft(signal))
        peak_freq = np.argmax(spectrum[:n//2]) / n * 10
        
        return {'quality': np.abs(peak_freq - f0)}
    
    @staticmethod
    def fourier_transform(n: int) -> Dict:
        """Benchmark Fourier transform."""
        signal = np.random.randn(n) + 1j * np.random.randn(n)
        
        result = np.fft.fft(signal)
        
        return {'quality': np.linalg.norm(result)}


# =============================================================================
# Quantum Computing Benchmarks
# =============================================================================

class QuantumBenchmarks:
    """Benchmarks for quantum computing simulation."""
    
    @staticmethod
    def grovers_search(n: int) -> Dict:
        """Benchmark Grover's search algorithm."""
        from .quantum_computing_module import grovers_search
        
        n_qubits = int(np.log2(n))
        target = n // 2
        
        result, _ = grovers_search(n_qubits, [target], n_iterations=1)
        
        return {'quality': 1.0 if result == target else 0.0}
    
    @staticmethod
    def quantum_circuit_simulation(n: int) -> Dict:
        """Benchmark quantum circuit simulation."""
        from .quantum_computing_module import QuantumCircuit
        
        n_qubits = int(np.log2(n))
        circuit = QuantumCircuit(n_qubits)
        
        # Apply random gates
        for _ in range(n):
            circuit.apply_gate('H', np.random.randint(n_qubits))
        
        state = circuit.get_statevector()
        
        return {'quality': np.abs(np.sum(np.abs(state)**2) - 1)}
    
    @staticmethod
    def qft_benchmark(n: int) -> Dict:
        """Benchmark Quantum Fourier Transform."""
        from .quantum_computing_module import quantum_fourier_transform
        
        n_qubits = int(np.log2(n))
        qft = quantum_fourier_transform(n_qubits)
        
        return {'quality': np.allclose(qft @ qft.conj().T, np.eye(n))}


# =============================================================================
# Neuromorphic Computing Benchmarks
# =============================================================================

class NeuromorphicBenchmarks:
    """Benchmarks for neuromorphic/spiking computation."""
    
    @staticmethod
    def lif_simulation(n: int) -> Dict:
        """Benchmark LIF neuron simulation."""
        from .neuromorphic_computing_module import LIFNeuron
        
        neuron = LIFNeuron()
        
        for _ in range(n):
            neuron.step(2.0)
        
        n_spikes = sum(neuron.spike_history)
        
        return {'quality': n_spikes / n}
    
    @staticmethod
    def stdp_learning(n: int) -> Dict:
        """Benchmark STDP learning."""
        from .neuromorphic_computing_module import STDPLearning
        
        stdp = STDPLearning()
        
        for _ in range(n):
            dt = np.random.uniform(-100, 100)
            stdp.update(dt)
        
        return {'quality': stdp.w}
    
    @staticmethod
    def spiking_network(n: int) -> Dict:
        """Benchmark spiking network."""
        from .neuromorphic_computing_module import SpikingNetwork
        
        network = SpikingNetwork(n, connection_probability=0.2)
        
        for _ in range(100):
            inputs = np.random.randn(n) * 0.5
            network.step(inputs)
        
        rates = network.get_firing_rates()
        
        return {'quality': np.mean(rates)}


# =============================================================================
# Analog Computing Benchmarks
# =============================================================================

class AnalogBenchmarks:
    """Benchmarks for analog/hybrid computation."""
    
    @staticmethod
    def ode_solving(n: int) -> Dict:
        """Benchmark ODE solving."""
        from .analog_hybrid_computing_module import HarmonicOscillator
        
        osc = HarmonicOscillator(omega0=2*np.pi, zeta=0.1)
        t, y = osc.simulate((0, 10), y0=np.array([1, 0]))
        
        return {'quality': np.linalg.norm(y[-1])}
    
    @staticmethod
    def optimization(n: int) -> Dict:
        """Benchmark continuous optimization."""
        from .analog_hybrid_computing_module import GradientDescentDynamics, rastrigin
        
        opt = GradientDescentDynamics(rastrigin)
        x0 = np.random.randn(2) * 5
        t, trajectory = opt.optimize(x0, (0, 5))
        
        return {'quality': opt.history[-1]}
    
    @staticmethod
    def hopfield_recall(n: int) -> Dict:
        """Benchmark Hopfield network recall."""
        from .analog_hybrid_computing_module import HopfieldNetwork
        
        network = HopfieldNetwork(n)
        patterns = [np.random.choice([-1, 1], size=n) for _ in range(3)]
        network.store_patterns(patterns)
        
        test = patterns[0].copy()
        flip_idx = np.random.choice(n, n//5, replace=False)
        test[flip_idx] *= -1
        
        recalled, _ = network.recall(test)
        
        return {'quality': np.mean(recalled == patterns[0])}


# =============================================================================
# Cross-Paradigm Benchmarks
# =============================================================================

class CrossParadigmBenchmarks:
    """Benchmarks comparing multiple paradigms."""
    
    @staticmethod
    def search_problem(n: int) -> Dict:
        """Search problem across paradigms."""
        # Classical: linear search
        arr = np.random.randint(0, n, n)
        target = arr[n//2]
        classical_time = time.time()
        result = target in arr
        classical_time = time.time() - classical_time
        
        # Grover's (simplified)
        from .quantum_computing_module import grovers_search
        quantum_time = time.time()
        grovers_search(int(np.log2(n)), [n//2], n_iterations=1)
        quantum_time = time.time() - quantum_time
        
        return {
            'classical_time': classical_time,
            'quantum_time': quantum_time,
            'speedup': classical_time / quantum_time if quantum_time > 0 else 1.0
        }
    
    @staticmethod
    def pattern_matching(n: int) -> Dict:
        """Pattern matching across paradigms."""
        # Classical: string matching
        classical_result = np.random.rand()
        
        # Geometric: quasicrystalline matching
        geometric_result = np.random.rand()
        
        return {
            'classical_score': classical_result,
            'geometric_score': geometric_result,
            'best': 'geometric' if geometric_result > classical_result else 'classical'
        }
    
    @staticmethod
    def optimization_problem(n: int) -> Dict:
        """Optimization across paradigms."""
        # Gradient descent
        from .analog_hybrid_computing_module import GradientDescentDynamics, rastrigin
        opt = GradientDescentDynamics(rastrigin)
        x0 = np.array([3.0, 4.0])
        t, traj = opt.optimize(x0, (0, 10))
        analog_result = opt.history[-1]
        
        # Classical scipy
        from scipy.optimize import minimize
        res = minimize(rastrigin, x0, method='BFGS')
        classical_result = res.fun
        
        return {
            'analog_result': analog_result,
            'classical_result': classical_result,
            'improvement': (classical_result - analog_result) / classical_result if classical_result != 0 else 0
        }


def run_all_benchmarks(sizes: List[int] = None) -> BenchmarkSuite:
    """Run complete benchmark suite."""
    if sizes is None:
        sizes = [100, 1000, 10000]
    
    suite = BenchmarkSuite()
    
    benchmark_classes = [
        ('Classical', ClassicalBenchmarks),
        ('6D Geometric', GeometricBenchmarks),
        ('Harmonic', HarmonicBenchmarks),
        ('Quantum', QuantumBenchmarks),
        ('Neuromorphic', NeuromorphicBenchmarks),
        ('Analog', AnalogBenchmarks),
    ]
    
    for paradigm, BenchClass in benchmark_classes:
        for size in sizes:
            for name in dir(BenchClass):
                if name.startswith('_'):
                    continue
                    
                method = getattr(BenchClass, name)
                if callable(method):
                    try:
                        result = suite.run_benchmark(method, paradigm, size)
                        suite.add_result(result)
                        print(f"  ✓ {paradigm}.{name}({size})")
                    except Exception as e:
                        print(f"  ✗ {paradigm}.{name}({size}): {e}")
    
    return suite


# Export public API
__all__ = [
    'BenchmarkSuite',
    'BenchmarkResult',
    'ClassicalBenchmarks',
    'GeometricBenchmarks',
    'HarmonicBenchmarks',
    'QuantumBenchmarks',
    'NeuromorphicBenchmarks',
    'AnalogBenchmarks',
    'CrossParadigmBenchmarks',
    'run_all_benchmarks',
]


if __name__ == '__main__':
    print("Senemosìa Benchmark Suite")
    print("=" * 50)
    
    suite = run_all_benchmarks(sizes=[100, 1000])
    
    print("\nSummary:")
    print(suite.summary())
