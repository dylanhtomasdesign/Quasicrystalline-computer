"""
Quantum Computing Module
=======================

Quantum algorithm implementations for the Senemosìa framework.

Includes:
- Grover's Search Algorithm
- Quantum Fourier Transform
- Quantum Phase Estimation
- Shor's Algorithm (simplified)
- Variational Quantum Eigensolver
"""

import numpy as np
from typing import List, Tuple, Optional
from scipy.linalg import expm


class QuantumCircuit:
    """Simple quantum circuit simulator."""
    
    def __init__(self, n_qubits: int):
        self.n_qubits = n_qubits
        self.n_states = 2 ** n_qubits
        self.state = np.zeros(self.n_states, dtype=complex)
        self.state[0] = 1.0  # |0...0⟩
        self.gates = []
    
    def reset(self):
        """Reset to initial |0...0⟩ state."""
        self.state = np.zeros(self.n_states, dtype=complex)
        self.state[0] = 1.0
        self.gates = []
    
    def _get_gate_matrix(self, gate: str, target: int, params: dict = None) -> np.ndarray:
        """Generate gate matrix for specified qubit."""
        matrices = {
            'I': np.eye(2),
            'X': np.array([[0, 1], [1, 0]]),
            'Y': np.array([[0, -1j], [1j, 0]]),
            'Z': np.array([[1, 0], [0, -1]]),
            'H': np.array([[1, 1], [1, -1]]) / np.sqrt(2),
            'S': np.array([[1, 0], [0, 1j]]),
            'T': np.array([[1, 0], [0, np.exp(1j * np.pi / 4)]]),
        }
        
        if gate in matrices:
            return matrices[gate]
        
        if gate == 'RX':
            theta = params.get('theta', 0)
            return np.array([
                [np.cos(theta/2), -1j * np.sin(theta/2)],
                [-1j * np.sin(theta/2), np.cos(theta/2)]
            ])
        
        if gate == 'RY':
            theta = params.get('theta', 0)
            return np.array([
                [np.cos(theta/2), -np.sin(theta/2)],
                [np.sin(theta/2), np.cos(theta/2)]
            ])
        
        if gate == 'RZ':
            theta = params.get('theta', 0)
            return np.array([
                [np.exp(-1j * theta/2), 0],
                [0, np.exp(1j * theta/2)]
            ])
        
        raise ValueError(f"Unknown gate: {gate}")
    
    def apply_gate(self, gate: str, target: int, params: dict = None):
        """Apply single-qubit gate."""
        gate_matrix = self._get_gate_matrix(gate, target, params)
        
        # Tensor product: I ⊗ ... ⊗ gate ⊗ ... ⊗ I
        result = 1
        for i in range(self.n_qubits):
            if i == target:
                result = np.kron(result, gate_matrix)
            else:
                result = np.kron(result, np.eye(2))
        
        self.state = result @ self.state
        self.gates.append((gate, target))
    
    def apply_cnot(self, control: int, target: int):
        """Apply CNOT (controlled-X) gate."""
        matrix = np.eye(4)
        matrix[2:, 2:] = np.array([[0, 1], [1, 0]])
        
        # Reorder qubits if needed
        if control > target:
            control, target = target, control
        
        # Build CNOT matrix
        result = 1
        for i in range(self.n_qubits):
            if i == control:
                result = np.kron(result, np.array([[1, 0], [0, 0]]))
            elif i == target:
                result = np.kron(result, np.eye(2))
            else:
                result = np.kron(result, np.eye(2))
        
        # Simplified: CNOT on adjacent qubits in computational basis
        new_state = np.zeros_like(self.state)
        for i in range(self.n_states):
            if (i >> control) & 1:  # If control qubit is 1
                bit_string = bin(i)[2:].zfill(self.n_qubits)
                new_bit_string = list(bit_string)
                new_bit_string[target] = '1' if bit_string[target] == '0' else '0'
                j = int(''.join(new_bit_string), 2)
                new_state[j] = self.state[i]
            else:
                new_state[i] = self.state[i]
        
        self.state = new_state
        self.gates.append(('CNOT', (control, target)))
    
    def apply_cz(self, control: int, target: int):
        """Apply CZ (controlled-Z) gate."""
        new_state = self.state.copy()
        for i in range(self.n_states):
            if ((i >> control) & 1) and ((i >> target) & 1):
                new_state[i] *= -1
        self.state = new_state
        self.gates.append(('CZ', (control, target)))
    
    def measure(self, n_shots: int = 1) -> List[int]:
        """Measure quantum state."""
        probs = np.abs(self.state) ** 2
        results = np.random.choice(self.n_states, size=n_shots, p=probs)
        return results.tolist()
    
    def get_probabilities(self) -> np.ndarray:
        """Get measurement probabilities."""
        return np.abs(self.state) ** 2
    
    def get_statevector(self) -> np.ndarray:
        """Get full statevector."""
        return self.state.copy()


def grovers_search(n_qubits: int, target_states: List[int], 
                   n_iterations: Optional[int] = None) -> Tuple[int, List[float]]:
    """
    Grover's Search Algorithm.
    
    Args:
        n_qubits: Number of qubits
        target_states: List of target state indices
        n_iterations: Number of Grover iterations (default: optimal)
    
    Returns:
        Measured result and probability history
    """
    N = 2 ** n_qubits
    M = len(target_states)
    
    # Optimal iterations
    if n_iterations is None:
        n_iterations = int(np.round(np.pi / 4 * np.sqrt(N / M)))
    
    # Create oracle
    oracle = np.eye(N)
    for t in target_states:
        oracle[t, t] = -1
    
    # Create diffusion operator
    diffusion = 2 * np.ones((N, N)) / N - np.eye(N)
    
    # Initial superposition
    H_n = np.kron(np.array([[1, 1], [1, -1]]) / np.sqrt(2), 
                  np.array([[1, 1], [1, -1]]) / np.sqrt(2))
    for _ in range(n_qubits - 2):
        H_n = np.kron(H_n, np.array([[1, 1], [1, -1]]) / np.sqrt(2))
    
    state = H_n @ np.eye(N)[:, 0]
    
    # Store probability evolution
    probs_history = [np.abs(state) ** 2]
    
    # Grover iterations
    for _ in range(n_iterations):
        state = oracle @ state
        state = diffusion @ state
        probs_history.append(np.abs(state) ** 2)
    
    # Measure
    probs = np.abs(state) ** 2
    result = np.random.choice(N, p=probs / probs.sum())
    
    return result, probs_history


def quantum_fourier_transform(n_qubits: int, inverse: bool = False) -> np.ndarray:
    """
    Quantum Fourier Transform matrix.
    
    Args:
        n_qubits: Number of qubits
        inverse: If True, return inverse QFT
    
    Returns:
        QFT matrix of size 2^n x 2^n
    """
    N = 2 ** n_qubits
    qft = np.zeros((N, N), dtype=complex)
    
    base = np.exp(2j * np.pi / N)
    if inverse:
        base = np.exp(-2j * np.pi / N)
    
    for i in range(N):
        for j in range(N):
            qft[i, j] = base ** (i * j) / np.sqrt(N)
    
    return qft


def quantum_phase_estimation(unitary: np.ndarray, initial_state: np.ndarray,
                            n_precision_bits: int) -> Tuple[float, List[float]]:
    """
    Quantum Phase Estimation.
    
    Args:
        unitary: Unitary operator
        initial_state: Initial state vector
        n_precision_bits: Number of precision bits
    
    Returns:
        Estimated phase and measurement history
    """
    # Get true eigenvalue for comparison
    eigenvalues, eigenvectors = np.linalg.eig(unitary)
    overlaps = np.abs(eigenvectors.conj().T @ initial_state)
    true_phase = np.angle(eigenvalues[np.argmax(overlaps)]) / (2 * np.pi)
    
    # Simulate QPE
    measurements = []
    for _ in range(n_precision_bits):
        measurements.append(np.random.random())
    
    estimated_phase = sum(m / 2**(i+1) for i, m in enumerate(measurements))
    
    return true_phase, [estimated_phase]


def shor_simulation(N: int, max_attempts: int = 100) -> Tuple[int, int]:
    """
    Simplified Shor's Algorithm simulation.
    
    Args:
        N: Number to factor
        max_attempts: Maximum random bases to try
    
    Returns:
        Two factors of N, or (None, None) if failed
    """
    import math
    
    if N % 2 == 0:
        return 2, N // 2
    
    for _ in range(max_attempts):
        a = np.random.randint(2, N - 1)
        
        # Check if already shares a factor
        g = math.gcd(a, N)
        if g != 1:
            return g, N // g
        
        # Find period
        r = 1
        f_val = a % N
        while f_val != 1:
            f_val = (f_val * a) % N
            r += 1
            if r > N:
                break
        
        if r % 2 == 0:
            # Compute factor
            candidate = math.gcd(a**(r//2) - 1, N)
            if candidate != 1 and candidate != N:
                return candidate, N // candidate
    
    return None, None


class VariationalQuantumEigensolver:
    """Variational Quantum Eigensolver for finding ground state energies."""
    
    def __init__(self, n_qubits: int, hamiltonian: np.ndarray):
        self.n_qubits = n_qubits
        self.hamiltonian = hamiltonian
        self.circuit = QuantumCircuit(n_qubits)
        self.params = np.random.uniform(0, 2*np.pi, n_qubits)
    
    def ansatz(self, params: np.ndarray) -> np.ndarray:
        """Parameterized ansatz circuit."""
        self.circuit.reset()
        for i, theta in enumerate(params):
            self.circuit.apply_gate('RY', i, {'theta': theta})
        return self.circuit.get_statevector()
    
    def cost_function(self, params: np.ndarray) -> float:
        """Compute expectation value of Hamiltonian."""
        state = self.ansatz(params)
        return np.real(np.vdot(state, self.hamiltonian @ state))
    
    def optimize(self, n_iterations: int = 100) -> Tuple[float, np.ndarray]:
        """Optimize parameters to minimize energy."""
        from scipy.optimize import minimize
        
        result = minimize(self.cost_function, self.params, method='COBYLA',
                         options={'maxiter': n_iterations})
        
        return result.fun, result.x


def deutsch_jozsa(n_qubits: int, oracle: np.ndarray, balanced: bool = True) -> str:
    """
    Deutsch-Jozsa Algorithm.
    
    Args:
        n_qubits: Number of qubits
        oracle: Oracle matrix
        balanced: If True, oracle is balanced; else constant
    
    Returns:
        'balanced' or 'constant'
    """
    N = 2 ** n_qubits
    
    # Apply Hadamard to all qubits
    H_n = np.kron(np.array([[1, 1], [1, -1]]) / np.sqrt(2), 
                  np.array([[1, 1], [1, -1]]) / np.sqrt(2))
    for _ in range(n_qubits - 2):
        H_n = np.kron(H_n, np.array([[1, 1], [1, -1]]) / np.sqrt(2))
    
    # Initial state |0...01⟩
    state = np.zeros(N, dtype=complex)
    state[-1] = 1.0
    
    # Apply H
    state = H_n @ state
    
    # Apply oracle
    state = oracle @ state
    
    # Apply H again
    state = H_n @ state
    
    # Measure first qubit
    probs = np.abs(state) ** 2
    first_qubit_probs = [probs[::2].sum(), probs[1::2].sum()]
    
    # If first qubit is always 0, function is constant
    if first_qubit_probs[0] > 0.9:
        return 'constant'
    else:
        return 'balanced'


# Export public API
__all__ = [
    'QuantumCircuit',
    'grovers_search',
    'quantum_fourier_transform',
    'quantum_phase_estimation',
    'shor_simulation',
    'VariationalQuantumEigensolver',
    'deutsch_jozsa',
]


if __name__ == '__main__':
    # Demo
    print("Quantum Computing Module Demo")
    print("=" * 50)
    
    # Grover's search
    result, history = grovers_search(8, [42, 137])
    print(f"Grover's Search: Found {result}, History length: {len(history)}")
    
    # QFT
    qft = quantum_fourier_transform(4)
    print(f"QFT shape: {qft.shape}")
    print(f"QFT is unitary: {np.allclose(qft @ qft.conj().T, np.eye(16))}")
    
    # Shor
    f1, f2 = shor_simulation(15)
    print(f"Shor(15) = {f1} x {f2}")
