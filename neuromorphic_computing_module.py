"""
Neuromorphic Computing Module
=============================

Spiking Neural Network implementations for the Senemosìa framework.

Includes:
- Leaky Integrate-and-Fire (LIF) neurons
- Izhikevich neuron model
- STDP learning rule
- Simple spiking network simulation
"""

import numpy as np
from typing import List, Tuple, Optional
import matplotlib.pyplot as plt


class LIFNeuron:
    """
    Leaky Integrate-and-Fire (LIF) Neuron Model.
    
    Dynamics:
        τ * dv/dt = -(v - v_rest) + R * I(t)
        
    When v >= v_thresh: spike and reset
    """
    
    def __init__(self, tau: float = 20.0, v_rest: float = -70.0,
                 v_thresh: float = -55.0, v_reset: float = -75.0,
                 r: float = 10.0, dt: float = 1.0):
        self.tau = tau        # Membrane time constant (ms)
        self.v_rest = v_rest  # Resting potential (mV)
        self.v_thresh = v_thresh  # Spike threshold (mV)
        self.v_reset = v_reset    # Reset potential (mV)
        self.r = r           # Membrane resistance (MΩ)
        self.dt = dt         # Time step (ms)
        
        self.v = v_rest      # Current membrane potential
        self.spike_history = []
        self.v_history = []
        self.last_spike_time = -1000
    
    def step(self, i_input: float) -> float:
        """
        Simulate one time step.
        
        Args:
            i_input: Input current (mV)
            
        Returns:
            Current membrane potential
        """
        # Leaky integration
        dv = (-(self.v - self.v_rest) / self.tau + i_input / self.tau) * self.dt
        self.v += dv
        
        # Check for spike
        if self.v >= self.v_thresh:
            self.spike_history.append(1)
            self.v = self.v_reset
            spike = 1
        else:
            self.spike_history.append(0)
            spike = 0
        
        self.v_history.append(self.v)
        return spike
    
    def reset(self):
        """Reset neuron state."""
        self.v = self.v_rest
        self.spike_history = []
        self.v_history = []
        self.last_spike_time = -1000
    
    def get_spike_times(self) -> List[int]:
        """Return spike times."""
        return [i for i, s in enumerate(self.spike_history) if s == 1]


class IzhikevichNeuron:
    """
    Izhikevich Neuron Model.
    
    A simple, biologically plausible model that can exhibit many spiking patterns.
    
    Dynamics:
        dv/dt = 0.04*v² + 5*v + 140 - u + I
        du/dt = a*(b*v - u)
        
    When v >= 30: v = c, u = u + d
    """
    
    # Neuron type presets
    REGIMES = {
        'RS': {'a': 0.02, 'b': 0.2, 'c': -65, 'd': 2},    # Regular Spiking
        'FS': {'a': 0.1, 'b': 0.2, 'c': -65, 'd': 2},    # Fast Spiking
        'IB': {'a': 0.02, 'b': 0.2, 'c': -55, 'd': 4},    # Intrinsically Bursting
        'CH': {'a': 0.02, 'b': 0.2, 'c': -50, 'd': 2},    # Chattering
        'LTS': {'a': 0.02, 'b': 0.25, 'c': -65, 'd': 2},  # Low-threshold Spiking
        'RZ': {'a': 0.02, 'b': 0.25, 'c': -65, 'd': 2},   # Resonator
    }
    
    def __init__(self, a: float = 0.02, b: float = 0.2, 
                 c: float = -65, d: float = 2):
        self.a = a  # Recovery time constant
        self.b = b  # Recovery sensitivity  
        self.c = c  # Reset voltage
        self.d = d  # Recovery reset
        
        self.v = c  # Initial voltage
        self.u = b * c  # Initial recovery variable
        
        self.spike_history = []
        self.v_history = []
    
    def step(self, i_input: float, dt: float = 1) -> float:
        """Simulate one time step."""
        for _ in range(int(dt)):
            v, u = self.v, self.u
            
            # Membrane potential update
            dv = 0.04 * v**2 + 5*v + 140 - u + i_input
            du = self.a * (self.b * v - u)
            
            v += dv
            u += du
            
            # Spike reset
            if v >= 30:
                self.spike_history.append(1)
                v = self.c
                u = u + self.d
            else:
                self.spike_history.append(0)
            
            self.v = v
            self.u = u
        
        self.v_history.append(self.v)
        return self.v
    
    def reset(self):
        """Reset neuron state."""
        self.v = self.c
        self.u = self.b * self.c
        self.spike_history = []
        self.v_history = []
    
    @classmethod
    def from_regime(cls, regime: str):
        """Create neuron from predefined regime."""
        params = cls.REGIMES.get(regime, cls.REGIMES['RS'])
        return cls(**params)


class STDPLearning:
    """
    Spike-Timing-Dependent Plasticity (STDP) Learning Rule.
    
    Weight change depends on timing difference Δt = t_post - t_pre
    
    Potentiation (Δt > 0): Δw = A_plus * exp(-Δt / tau_plus)
    Depression (Δt < 0): Δw = -A_minus * exp(Δt / tau_minus)
    """
    
    def __init__(self, a_plus: float = 0.01, a_minus: float = 0.012,
                 tau_plus: float = 20.0, tau_minus: float = 20.0,
                 w_max: float = 1.0, w_min: float = 0.0):
        self.a_plus = a_plus
        self.a_minus = a_minus
        self.tau_plus = tau_plus
        self.tau_minus = tau_minus
        self.w_max = w_max
        self.w_min = w_min
        
        self.w = 0.5  # Initial weight
    
    def update(self, delta_t: float) -> float:
        """
        Update synaptic weight based on spike timing.
        
        Args:
            delta_t: Time difference t_post - t_pre (ms)
            
        Returns:
            Updated weight
        """
        if delta_t >= 0:
            # Potentiation: pre before post
            dw = self.a_plus * np.exp(-delta_t / self.tau_plus)
        else:
            # Depression: post before pre
            dw = -self.a_minus * np.exp(delta_t / self.tau_minus)
        
        self.w = np.clip(self.w + dw, self.w_min, self.w_max)
        return self.w
    
    def reset(self):
        """Reset weight to initial value."""
        self.w = 0.5


def stdp_window(delta_t: float, a_plus: float = 0.01, 
                a_minus: float = 0.012, tau_plus: float = 20.0,
                tau_minus: float = 20.0) -> float:
    """
    Compute STDP weight change for given timing difference.
    
    Returns:
        Weight change Δw
    """
    if delta_t >= 0:
        return a_plus * np.exp(-delta_t / tau_plus)
    else:
        return -a_minus * np.exp(delta_t / tau_minus)


class SpikingNetwork:
    """
    Network of spiking neurons with synaptic connections.
    """
    
    def __init__(self, n_neurons: int, connection_probability: float = 0.1):
        self.n_neurons = n_neurons
        self.neurons = [LIFNeuron() for _ in range(n_neurons)]
        
        # Initialize synaptic weights
        self.W = np.random.randn(n_neurons, n_neurons) * connection_probability
        self.W = np.maximum(self.W, 0)  # Positive weights only
        np.fill_diagonal(self.W, 0)  # No self-connections
        
        # State
        self.state = np.zeros(n_neurons)
        self.spikes = np.zeros(n_neurons)
    
    def step(self, inputs: np.ndarray, dt: float = 1.0) -> np.ndarray:
        """
        Simulate one time step.
        
        Args:
            inputs: External input currents (n_neurons,)
            dt: Time step (ms)
            
        Returns:
            Spike array (n_neurons,)
        """
        # Compute recurrent input
        recurrent = self.W @ self.spikes
        
        # Total input
        total_input = inputs + recurrent
        
        # Update neurons
        spikes = np.zeros(self.n_neurons)
        for i, neuron in enumerate(self.neurons):
            spikes[i] = neuron.step(total_input[i], dt)
        
        self.spikes = spikes
        return spikes
    
    def apply_stdp(self, pre_idx: int, post_idx: int, delta_t: float):
        """Apply STDP update to synapse."""
        dw = stdp_window(delta_t)
        self.W[post_idx, pre_idx] = np.clip(
            self.W[post_idx, pre_idx] + dw, 0, 1
        )
    
    def get_firing_rates(self, window: int = 100) -> np.ndarray:
        """Compute firing rates for each neuron."""
        rates = np.zeros(self.n_neurons)
        for i, neuron in enumerate(self.neurons):
            n_spikes = sum(neuron.spike_history[-window:])
            rates[i] = n_spikes / (window * 0.001)  # spikes/sec
        return rates
    
    def reset(self):
        """Reset all neurons and state."""
        for neuron in self.neurons:
            neuron.reset()
        self.spikes = np.zeros(self.n_neurons)


class PoissonSpikeGenerator:
    """Generate Poisson-distributed spike trains."""
    
    def __init__(self, rate: float, seed: Optional[int] = None):
        """
        Args:
            rate: Firing rate (Hz)
            seed: Random seed for reproducibility
        """
        self.rate = rate
        if seed is not None:
            np.random.seed(seed)
    
    def generate(self, duration: float, dt: float = 1.0) -> np.ndarray:
        """
        Generate spike train.
        
        Args:
            duration: Duration in ms
            dt: Time step in ms
            
        Returns:
            Binary spike array
        """
        n_steps = int(duration / dt)
        probabilities = self.rate * dt / 1000  # Convert Hz to per-step prob
        return (np.random.rand(n_steps) < probabilities).astype(int)
    
    def generate_multiple(self, n_trains: int, duration: float, 
                         dt: float = 1.0) -> np.ndarray:
        """Generate multiple spike trains."""
        return np.array([self.generate(duration, dt) for _ in range(n_trains)])


def compute_neuronal_variance(spike_trains: np.ndarray, 
                               window_size: int = 100) -> float:
    """
    Compute variance of neuronal activity across spike trains.
    """
    n_trains, n_steps = spike_trains.shape
    
    # Bin spike trains
    n_windows = n_steps // window_size
    binned = spike_trains[:, :n_windows*window_size].reshape(n_trains, n_windows, window_size)
    counts = binned.sum(axis=2)
    
    # Compute variance
    return np.var(counts)


# Export public API
__all__ = [
    'LIFNeuron',
    'IzhikevichNeuron',
    'STDPLearning',
    'SpikingNetwork',
    'PoissonSpikeGenerator',
    'stdp_window',
    'compute_neuronal_variance',
]


if __name__ == '__main__':
    print("Neuromorphic Computing Module Demo")
    print("=" * 50)
    
    # LIF neuron
    neuron = LIFNeuron()
    for i in range(200):
        if 50 <= i < 150:
            neuron.step(2.5)
        else:
            neuron.step(0)
    
    n_spikes = sum(neuron.spike_history)
    print(f"LIF Neuron: {n_spikes} spikes in 200ms")
    
    # Izhikevich regimes
    print("\nIzhikevich Neuron Regimes:")
    for regime in ['RS', 'FS', 'IB', 'CH']:
        neuron = IzhikevichNeuron.from_regime(regime)
        for _ in range(100):
            neuron.step(10)
        n_spikes = sum(neuron.spike_history)
        print(f"  {regime}: {n_spikes} spikes")
    
    # STDP
    print("\nSTDP Weight Updates:")
    stdp = STDPLearning()
    for dt in [10, -10, 20, -20]:
        w = stdp.update(dt)
        print(f"  Δt = {dt:3d}ms: w = {w:.4f}")
