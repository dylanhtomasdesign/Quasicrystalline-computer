"""
Analog/Hybrid Computing Module
==============================

Continuous dynamical systems for the Senemosìa framework.

Includes:
- ODE solvers (Euler, RK4, adaptive)
- Harmonic oscillator networks
- Gradient descent dynamics
- Hopfield networks
- Physical optimization (simulated annealing)
"""

import numpy as np
from typing import Callable, Tuple, Optional, List
from scipy.integrate import solve_ivp, odeint
from scipy.optimize import minimize


class ContinuousDynamicalSystem:
    """
    Base class for continuous dynamical systems.
    """
    
    def __init__(self, n_dimensions: int):
        self.n_dims = n_dimensions
        self.state = np.zeros(n_dimensions)
    
    def dynamics(self, t: float, state: np.ndarray) -> np.ndarray:
        """Override this method to define system dynamics."""
        raise NotImplementedError
    
    def simulate(self, t_span: Tuple[float, float], 
                 y0: Optional[np.ndarray] = None,
                 method: str = 'RK45') -> Tuple[np.ndarray, np.ndarray]:
        """
        Simulate the dynamical system.
        
        Args:
            t_span: (t0, tf) time span
            y0: Initial state (defaults to zeros)
            method: Integration method ('RK45', 'RK23', 'Euler', etc.)
            
        Returns:
            (t, y) time and state arrays
        """
        if y0 is None:
            y0 = self.state.copy()
        
        t_eval = np.linspace(t_span[0], t_span[1], 1000)
        
        if method == 'Euler':
            return self._euler_simulation(t_span, y0, t_eval)
        elif method == 'RK4':
            return self._rk4_simulation(t_span, y0, t_eval)
        else:
            sol = solve_ivp(self.dynamics, t_span, y0, 
                          t_eval=t_eval, method=method)
            return sol.t, sol.y.T


class HarmonicOscillator(ContinuousDynamicalSystem):
    """
    Damped harmonic oscillator.
    
    d²x/dt² + 2ζω₀*dx/dt + ω₀²*x = 0
    """
    
    def __init__(self, omega0: float = 1.0, zeta: float = 0.1):
        super().__init__(n_dimensions=2)
        self.omega0 = omega0  # Natural frequency
        self.zeta = zeta       # Damping ratio
    
    def dynamics(self, t: float, state: np.ndarray) -> np.ndarray:
        x, v = state
        dxdt = v
        dvdt = -2 * self.zeta * self.omega0 * v - self.omega0**2 * x
        return np.array([dxdt, dvdt])


class CoupledOscillators(ContinuousDynamicalSystem):
    """
    Network of coupled harmonic oscillators.
    """
    
    def __init__(self, n_oscillators: int, k: float = 1.0, coupling: float = 0.5):
        super().__init__(n_dimensions=n_oscillators * 2)
        self.n_osc = n_oscillators
        self.k = k
        self.coupling = coupling
        
        # Coupling matrix
        self.coupling_matrix = np.zeros((n_oscillators, n_oscillators))
        for i in range(n_oscillators):
            for j in range(n_oscillators):
                if abs(i - j) == 1:
                    self.coupling_matrix[i, j] = coupling
    
    def dynamics(self, t: float, state: np.ndarray) -> np.ndarray:
        positions = state[:self.n_osc]
        velocities = state[self.n_osc:]
        
        accelerations = np.zeros(self.n_osc)
        
        for i in range(self.n_osc):
            # Self restoring force
            acc = -self.k * positions[i]
            
            # Coupling forces
            for j in range(self.n_osc):
                acc += self.coupling_matrix[i, j] * (positions[j] - positions[i])
            
            accelerations[i] = acc
        
        return np.concatenate([velocities, accelerations])


class LotkaVolterra(ContinuousDynamicalSystem):
    """
    Lotka-Volterra predator-prey equations.
    
    dx/dt = αx - βxy  (prey)
    dy/dt = δxy - γy  (predator)
    """
    
    def __init__(self, alpha: float = 1.5, beta: float = 1.0,
                 delta: float = 1.0, gamma: float = 1.0):
        super().__init__(n_dimensions=2)
        self.alpha = alpha
        self.beta = beta
        self.delta = delta
        self.gamma = gamma
    
    def dynamics(self, t: float, state: np.ndarray) -> np.ndarray:
        x, y = state
        dxdt = self.alpha * x - self.beta * x * y
        dydt = self.delta * x * y - self.gamma * y
        return np.array([dxdt, dydt])


class GradientDescentDynamics:
    """
    Gradient descent as a continuous dynamical system.
    
    dx/dt = -∇f(x)
    """
    
    def __init__(self, objective_func: Callable):
        self.objective = objective_func
        self.history = []
    
    def gradient(self, x: np.ndarray, eps: float = 1e-8) -> np.ndarray:
        """Numerical gradient."""
        grad = np.zeros_like(x)
        for i in range(len(x)):
            x_plus = x.copy()
            x_plus[i] += eps
            x_minus = x.copy()
            x_minus[i] -= eps
            grad[i] = (self.objective(x_plus) - self.objective(x_minus)) / (2 * eps)
        return grad
    
    def dynamics(self, t: float, state: np.ndarray, 
                 learning_rate: float = 0.1) -> np.ndarray:
        """Compute state dynamics."""
        grad = self.gradient(state)
        return -learning_rate * grad
    
    def optimize(self, x0: np.ndarray, t_span: Tuple[float, float],
                 learning_rate: float = 0.1, method: str = 'RK45') -> Tuple[np.ndarray, np.ndarray]:
        """
        Optimize using gradient descent dynamics.
        
        Returns:
            (t, states) trajectory
        """
        def wrapped_dynamics(t, state):
            return self.dynamics(t, state, learning_rate)
        
        t_eval = np.linspace(t_span[0], t_span[1], 1000)
        sol = solve_ivp(wrapped_dynamics, t_span, x0, t_eval=t_eval, method=method)
        
        # Compute objective values
        self.history = [self.objective(s) for s in sol.y.T]
        
        return sol.t, sol.y.T


class SimulatedAnnealingDynamics:
    """
    Simulated annealing as dynamical system.
    
    Temperature schedule determines exploration.
    """
    
    def __init__(self, objective_func: Callable, T0: float = 1.0,
                 cooling_rate: float = 0.995):
        self.objective = objective_func
        self.T = T0
        self.cooling_rate = cooling_rate
        self.history = []
    
    def step(self, x: np.ndarray, dt: float = 1.0) -> np.ndarray:
        """One step of simulated annealing."""
        # Compute gradient-like direction
        grad = np.zeros_like(x)
        eps = 1e-5
        for i in range(len(x)):
            x_plus = x.copy()
            x_plus[i] += eps
            grad[i] = (self.objective(x_plus) - self.objective(x)) / eps
        
        # Temperature-dependent noise
        noise = np.random.randn(len(x)) * np.sqrt(self.T)
        
        # Update
        x_new = x - 0.01 * grad + noise * np.sqrt(self.T)
        
        # Acceptance probability
        delta_E = self.objective(x_new) - self.objective(x)
        if delta_E > 0:
            prob = np.exp(-delta_E / self.T)
            if np.random.random() > prob:
                x_new = x  # Reject
        
        # Cool down
        self.T *= self.cooling_rate
        self.history.append(self.objective(x_new))
        
        return x_new
    
    def optimize(self, x0: np.ndarray, n_steps: int = 1000) -> Tuple[np.ndarray, List[float]]:
        """Run simulated annealing."""
        x = x0.copy()
        
        for _ in range(n_steps):
            x = self.step(x)
        
        return x, self.history


class HopfieldNetwork:
    """
    Hopfield Network for associative memory.
    
    Energy: E = -0.5 * Σᵢⱼ wᵢⱼ sᵢ sⱼ
    """
    
    def __init__(self, n_neurons: int):
        self.n = n_neurons
        self.W = np.zeros((n_neurons, n_neurons))
        self.threshold = 0
        self.states = None
    
    def store_patterns(self, patterns: List[np.ndarray]):
        """Store patterns using Hebbian learning."""
        for p in patterns:
            self.W += np.outer(p, p)
        self.W = self.W / self.n
        np.fill_diagonal(self.W, 0)
    
    def energy(self, state: np.ndarray) -> float:
        """Compute network energy."""
        return -0.5 * state @ self.W @ state + np.sum(state * self.threshold)
    
    def dynamics(self, state: np.ndarray) -> np.ndarray:
        """Asynchronous dynamics."""
        new_state = state.copy()
        i = np.random.randint(self.n)
        new_state[i] = np.sign(self.W[i] @ state - self.threshold)
        if new_state[i] == 0:
            new_state[i] = state[i]
        return new_state
    
    def recall(self, pattern: np.ndarray, max_iter: int = 100) -> Tuple[np.ndarray, List[float]]:
        """
        Recall stored pattern from noisy input.
        
        Returns:
            (recalled_pattern, energy_history)
        """
        state = pattern.copy()
        energies = [self.energy(state)]
        
        for _ in range(max_iter):
            new_state = self.dynamics(state)
            
            if np.array_equal(new_state, state):
                break
            
            state = new_state
            energies.append(self.energy(state))
        
        return state, energies
    
    def basin_of_attraction(self, pattern: np.ndarray, 
                            noise_levels: List[float]) -> np.ndarray:
        """
        Compute accuracy for different noise levels.
        """
        accuracies = []
        
        for noise in noise_levels:
            noisy = pattern.copy()
            flip_indices = np.random.choice(self.n, int(self.n * noise), replace=False)
            noisy[flip_indices] *= -1
            
            recalled, _ = self.recall(noisy)
            accuracy = np.mean(recalled == pattern)
            accuracies.append(accuracy)
        
        return np.array(accuracies)


def rastrigin(x: np.ndarray) -> float:
    """Rastrigin function - classic optimization benchmark."""
    A = 10
    return A * len(x) + np.sum(x**2 - A * np.cos(2 * np.pi * x))


def rosenbrock(x: np.ndarray) -> float:
    """Rosenbrock function."""
    return (1 - x[0])**2 + 100 * (x[1] - x[0]**2)**2


def ackley(x: np.ndarray) -> float:
    """Ackley function."""
    a, b, c = 20, 0.2, 2 * np.pi
    n = len(x)
    sum1 = np.sum(x**2)
    sum2 = np.sum(np.cos(c * x))
    return -a * np.exp(-b * np.sqrt(sum1/n)) - np.exp(sum2/n) + a + np.e


class ODESolver:
    """
    Collection of ODE solvers for analog computation.
    """
    
    @staticmethod
    def euler(f: Callable, y0: np.ndarray, t_span: Tuple, 
              n_steps: int) -> Tuple[np.ndarray, np.ndarray]:
        """Explicit Euler method."""
        t0, tf = t_span
        dt = (tf - t0) / n_steps
        
        y = np.zeros((n_steps + 1, len(y0)))
        y[0] = y0
        
        t = np.linspace(t0, tf, n_steps + 1)
        
        for i in range(n_steps):
            y[i+1] = y[i] + dt * f(t[i], y[i])
        
        return t, y
    
    @staticmethod
    def rk4(f: Callable, y0: np.ndarray, t_span: Tuple,
            n_steps: int) -> Tuple[np.ndarray, np.ndarray]:
        """Runge-Kutta 4th order method."""
        t0, tf = t_span
        dt = (tf - t0) / n_steps
        
        y = np.zeros((n_steps + 1, len(y0)))
        y[0] = y0
        
        t = np.linspace(t0, tf, n_steps + 1)
        
        for i in range(n_steps):
            k1 = f(t[i], y[i])
            k2 = f(t[i] + dt/2, y[i] + dt/2 * k1)
            k3 = f(t[i] + dt/2, y[i] + dt/2 * k2)
            k4 = f(t[i] + dt, y[i] + dt * k3)
            
            y[i+1] = y[i] + dt/6 * (k1 + 2*k2 + 2*k3 + k4)
        
        return t, y
    
    @staticmethod
    def adaptive_rk45(f: Callable, y0: np.ndarray, t_span: Tuple,
                      tol: float = 1e-6) -> Tuple[np.ndarray, np.ndarray]:
        """Adaptive Runge-Kutta-Fehlberg method."""
        sol = solve_ivp(f, t_span, y0, method='RK45', 
                       rtol=tol, atol=tol)
        return sol.t, sol.y.T


# Export public API
__all__ = [
    'ContinuousDynamicalSystem',
    'HarmonicOscillator',
    'CoupledOscillators',
    'LotkaVolterra',
    'GradientDescentDynamics',
    'SimulatedAnnealingDynamics',
    'HopfieldNetwork',
    'ODESolver',
    'rastrigin',
    'rosenbrock',
    'ackley',
]


if __name__ == '__main__':
    print("Analog/Hybrid Computing Module Demo")
    print("=" * 50)
    
    # Harmonic oscillator
    osc = HarmonicOscillator(omega0=2*np.pi, zeta=0.1)
    t, y = osc.simulate((0, 3), y0=np.array([1, 0]))
    print(f"Harmonic Oscillator: {len(t)} time points")
    
    # Lotka-Volterra
    lv = LotkaVolterra()
    t, y = lv.simulate((0, 50), y0=np.array([2, 1]))
    print(f"Lotka-Volterra: Prey final = {y[-1, 0]:.2f}, Predator final = {y[-1, 1]:.2f}")
    
    # Optimization
    opt = GradientDescentDynamics(rastrigin)
    x0 = np.array([3.0, 4.0])
    t, trajectory = opt.optimize(x0, (0, 10))
    print(f"Gradient Descent: Final value = {opt.history[-1]:.4f}")
    
    # Hopfield network
    np.random.seed(42)
    n = 100
    patterns = [np.random.choice([-1, 1], size=n) for _ in range(5)]
    
    hopfield = HopfieldNetwork(n)
    hopfield.store_patterns(patterns)
    
    test = patterns[0].copy()
    test[np.random.choice(n, 20, replace=False)] *= -1
    
    recalled, _ = hopfield.recall(test)
    accuracy = np.mean(recalled == patterns[0])
    print(f"Hopfield Network: Recall accuracy = {accuracy*100:.1f}%")
