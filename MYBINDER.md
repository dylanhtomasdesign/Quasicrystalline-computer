# Launch This Project on MyBinder

[![Binder](https://mybinder.org/badge_logo.svg)](https://mybinder.org/v2/gh/dylanhtomasdesign/Quasicrystalline-computer/main?urlpath=lab)

## Quick Start

**Click the badge above or [this link](https://mybinder.org/v2/gh/dylanhtomasdesign/Quasicrystalline-computer/main?urlpath=lab) to launch an interactive Jupyter Lab environment in your browser.**

No installation required! Everything runs in the cloud.

Once launched, navigate to the `notebooks/` folder to access all available notebooks.

---

## What is MyBinder?

[MyBinder.org](https://mybinder.org) is a free service that turns a GitHub repository into an interactive computing environment. It builds a Docker container from your code and dependencies, then launches it in a browser-based Jupyter Lab interface.

### Benefits:
- **Zero Installation:** No need to install Python, Jupyter, or dependencies
- **Cloud-Based:** Runs in your browser
- **Free & Open Source:** Powered by Jupyter and repo2docker
- **Share Research:** Easily share reproducible computational research
- **Instant Access:** Launch interactive notebooks directly from GitHub

---

## How It Works

### 1. **MyBinder reads from your repository:**
   - `requirements.txt` - Python package dependencies
   - `runtime.txt` - Python version specification
   - `Dockerfile` - Custom Docker image (alternative to requirements.txt)
   - `environment.yml` - Conda environment specification (alternative)

### 2. **Builds a Docker container** with:
   - Base Python environment
   - All dependencies installed
   - Your code and notebooks

### 3. **Launches Jupyter Lab** with:
   - Pre-installed Python packages
   - Direct access to your notebooks
   - Full computing power (subject to limits)

---

## Notebooks Available

### 📓 `01_Introduction.ipynb` ⭐ START HERE

Interactive tutorial covering:
- The Golden Ratio (φ) and 6D Geometry
- Icosahedral Projection
- Substrate-Problem Alignment Principle
- Energy Efficiency Comparison
- Laplacian Spectral Scheduling
- Quantum Computing (Grover's Algorithm)
- Spiking Neural Networks
- Summary and Key Insights

**Runtime:** ~5-10 minutes

### 📊 Additional Notebooks

- `02_Quantum_Computing.ipynb` - Detailed quantum algorithm simulations
- `03_Neuromorphic_Computing.ipynb` - SNN training and benchmarks
- `04_Analog_Computing.ipynb` - ODE solving demonstrations
- `05_Hardware_Deployment.ipynb` - FPGA and deployment pipeline

---

## System Requirements

### Client Side (Your Computer)
- Web browser (Chrome, Firefox, Safari, Edge)
- Internet connection
- No special software needed

### Server Side (MyBinder)
- Default: 1-2 GB RAM per session
- Default: ~12 hour session timeout
- CPU: Shared (typically 1-2 cores)
- Storage: Temporary (cleared after session ends)

### Session Limits
- **Idle Timeout:** 10 minutes
- **Max Runtime:** ~12 hours
- **Max Users:** ~100 concurrent per repo
- **Max File Size:** 100 GB git repo size

---

## Keyboard Shortcuts (Jupyter Lab)

| Shortcut | Action |
|----------|--------|
| `Ctrl + Enter` | Execute cell |
| `Shift + Enter` | Execute cell and move to next |
| `Ctrl + /` | Toggle comment |
| `Tab` | Autocomplete |
| `Shift + Tab` | Show docstring |
| `Esc` | Exit edit mode |
| `A` | Insert cell above |
| `B` | Insert cell below |
| `D + D` | Delete cell |
| `M` | Convert to Markdown |
| `Y` | Convert to Code |

---

## Save Your Work

⚠️ **Important:** Sessions are temporary. To save your work:

### Option 1: Download Notebook
1. File → Download → `.ipynb` format
2. Save to your local computer

### Option 2: Export as PDF
1. File → Export → PDF
2. Save visualization of your work

### Option 3: Push to GitHub
1. Clone the repo locally
2. Modify and commit changes
3. Push back to GitHub

---

## Troubleshooting

### **"Build failed" error**
- Check `requirements.txt` syntax
- Ensure package names are spelled correctly
- Verify version compatibility
- Rebuild by clicking "Rebuild" on mybinder.org page

### **Slow to start**
- First launch builds Docker image (~2-5 minutes)
- Subsequent builds are faster (cached)
- Try again later if services are busy

### **Session keeps timing out**
- MyBinder idles after 10 minutes of inactivity
- Click on the notebook to keep it active
- Run long computations before idle timeout

### **Out of memory**
- Close other browser tabs
- Clear notebook outputs: Cell → All Output → Clear
- Run computations with smaller datasets

---

## Configuration Files

### `requirements.txt`
Specifies Python package dependencies (already included in repo).

### `runtime.txt`
Specifies Python version:
```
python-3.9
```

---

## Next Steps

1. **Launch on MyBinder:** Click the badge at the top
2. **Run the introduction notebook:** `01_Introduction.ipynb`
3. **Explore the research:** Follow along with interactive examples
4. **Modify and experiment:** Try changing parameters and running cells
5. **Download results:** Save notebooks and visualizations

---

## Learn More

- **MyBinder Documentation:** https://mybinder.readthedocs.io/
- **Jupyter Project:** https://jupyter.org/
- **Repo2Docker:** https://repo2docker.readthedocs.io/
- **This Research:** https://github.com/dylanhtomasdesign/Quasicrystalline-computer

---

**Ready to explore? Click the MyBinder badge above to get started!** 🚀
