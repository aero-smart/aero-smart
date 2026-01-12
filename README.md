# AeroSmart — Desktop-Grade Intelligent Wind Tunnel System

## 1. Project Overview

AeroSmart is a desktop-grade intelligent ducted wind tunnel system designed for aeromodeling enthusiasts and early-stage prototype development teams. The project aims to provide a low-cost, repeatable, and engineering-grade validation tool positioned between CFD (Computational Fluid Dynamics) simulations and real flight testing, assisting in the screening and iterative optimization of design schemes.

The system utilizes a modular wind tunnel structure driven by dual ducted fans, capable of generating stable and controllable high-speed airflow within a small-scale test section. It achieves real-time monitoring of wind speed and airflow states through multi-source sensing methods, including Pitot tubes and acoustic sensors. Combined with PID closed-loop control and automated experimental workflows, users can conduct rapid, comparable testing and evaluation of different shapes or structural designs under consistent aerodynamic conditions. Building on this, the system introduces an AI analysis module that provides targeted suggestions for structural or profile improvements based on aerodynamic performance and experimental data, thereby enhancing design iteration efficiency.

* **Precisely Controllable Airflow Output**
* **Natural Language Intelligent Control**
* **Intelligent Optimization and Improvement Prompts**

---

## 2. Target Users and Use Cases

* **Aeromodeling Enthusiasts & Drone Players:** For validating wing and body simulations when large-scale wind tunnels are inaccessible.
    * AeroSmart provides a stable desktop environment for low-cost aerodynamic testing.
    * Controllable wind speeds allow for comparative analysis.
    * Natural Language Control lowers the barrier to entry, requiring no complex wind tunnel or system control experience.


* **Student Startup Projects:** Bridging the gap between CFD simulation and physical prototyping where a low-cost intermediate validation step is missing.
    * Acts as a workflow validation tool for rapid screening of design options.
    * Multi-source sensor data provides a quantitative basis for decisions.
    * AI analysis and optimization tips improve efficiency.


* **Educational Demonstrations:** CFD results can be abstract; real airflow is vital for teaching.
    * Desktop-sized equipment suitable for classroom environments.
    * Real-time visualization and data overlays turn abstract concepts into intuitive phenomena.
    * Adjustable airflow allows for quick demonstrations of different effects and models.

---

## 3. Technical Solution

### 3.1 Technical Specifications

| Module | Purpose | Specification |
| --- | --- | --- |
| **Intake & Filtering** | Generate straight airflow | 5 mm Aluminum Honeycomb + PLA-CF |
| **Contraction Layer (Nozzle)** | Accelerate airflow | 4:1 Area Ratio |
| **Test Section** | Observation & Data | 5" x 3" x 8"; 5 mm PC Panel |
| **Diffuser & Splitter** | Pressure recovery | Y-splitter with 7° expansion angle |
| **Power Source** | Generate airflow | Dual 70mm EDFs, Max 3.6kg/4.6kg thrust |

### 3.2 Hardware Solution

**3.2.1 Structural Materials**
The overall structure primarily uses 3D printing and laser cutting:

* **Intake Filter Section:** 3003 Aluminum Alloy honeycomb for durability; intake frame 3D printed with PLA-CF.
* **Contraction:** Integrated 3D printing using PETG-CF for structural strength.
* **Test Section:** Laser-cut PC panels; TPU (95A) used for gaskets/seals.
* **Diffuser:** PETG-CF, rigid enough to prevent "flutter" or structural vibration during dual EDF operation.
* **Power Mount:** PETG-CF brackets adapted for 3.6 kg thrust levels.
* **External Skeleton:** M5 threaded rods provide axial tension to keep modules compressed and aligned.

**3.2.2 Flow Field Control**

* **Laminar Core:** The contraction design and aluminum honeycomb create a high-quality laminar core in the center of the test section, reducing background disturbance for stable measurements.
* **Shear Simulation:** By applying different RPM commands to EDF1 and EDF2, the internal Y-splitter maintains two distinct velocity zones, creating a controllable shear layer on the model surface to simulate non-uniform wind gradients.
* **Active Gust Modulation:** The STM32H7 performs high-frequency pulse control on the EDFs to simulate atmospheric turbulence and gust loads, testing the structural resonance of models.
* **Angle Control:** An MG90S servo at the bottom of the test section controls the model’s angle of attack (Optional).

### 3.3 Software Solution (Intelligence & Data Architecture)

**3.3.1 Perception Layer**

* **Airspeed Array (MS4525DO, SDP810):** Measures differential pressure to calculate airspeed via Bernoulli’s principle.
* **Omnidirectional Microphone (ICS-43434):** Collects acoustic data to detect potential turbulence and aerodynamic noise.
* **Camera System:** Uses the SolvePnP algorithm and AprilTags for spatial positioning of the object within the test section.
* **IMU (ICM-42688):** Real-time motion and attitude data collection.
* **Pressure Sensors:** Monitors real-time forces acting on the experimental object.

**3.3.2 Processing Layer**

* **STM32H7:** Handles real-time PID control for thrust stability and Fast Fourier Transform (FFT) on microphone data for acoustic profiling.
* **Raspberry Pi (or Orange Pi):** Runs the MCP Server as an interface for Large Language Models (LLM), enabling natural language queries of sensor data and wind condition commands. It will also host small machine learning models for experimental data analysis.

**3.3.3 Visualization Layer**

* Real-time telemetry displayed via a **Web Dashboard**.
* **"Aero-Current" vectors** overlaid on real-time video of the model.
* **Smoke generator** for immediate visual observation of flow states.

---

## 4. Project Roadmap

| Phase | Core Tasks | Key Deliverables |
| --- | --- | --- |
| **Phase 1: Fluid Prototype** | Printing, honeycomb assembly, single EDF testing. | Stable laminar output (Smoke test). |
| **Phase 2: Perception Fusion** | Integrate Pitot, MIC, IMU, and STM32 PID control. | Real-time airspeed & attitude dashboard. |
| **Phase 3: Smart Interaction** | Raspberry Pi & MCP Server; LLM interface. | Voice/Text control of wind tunnel. |
| **Phase 4: AI Analysis** | Data accumulation; regression model training. | Automated experimental evaluation reports. |

*Target for the current term: Achieve at least Phase 2 and partially complete Phase 3.*
