# 🧙‍♂️ Rust Architecture & Design Masterclass: Visual Blueprints

Welcome to the official code companion repository for the **Rust Architecture Masterclass**. 

This repository is designed like an engineering drafting board. The code here strips away the noise of complex production frameworks to isolate and expose core architectural patterns, memory layouts, and type-system design mechanics in Rust.

📺 **Don't just read the implementation. Watch the system breathe.** Every module in this repository maps directly to a high-fidelity, animation-driven breakdown on YouTube where we trace data permissions, ownership flow, and compiler constraints using custom visual sigils.

▶️ [**Click Here to Watch the Full Masterclass Playlist**](https://tinyurl.com/SEInRustVideos)

---

## 🛠️ Repository Architecture

This repository is organized as a unified **Cargo Workspace**. Each directory corresponds to a specific phase and episode in the architectural class, allowing you to isolate the exact code state seen on your screen.

```text
rust-architecture-masterclass/
├── README.md                            <-- You are here
├── Cargo.toml                           <-- Workspace manifest
├── video_01_the_diagram/                <-- Tracking state via the central sigil
├── video_02_tdd_ownership/              <-- Prototyping data boundaries with tests
├── video_03_bad_getters/                <-- Why getters are a bad idea and what to do instead
├── video_04_methods_vs_free/            <-- How to choose between making a free function and a method
├── video_05_proliferation_of_clones/    <-- What do clones tell you about your design?
├── video_06_closed_polymorphism/        <-- Collecting objects of different types using enums
├── video_07_open_polymorphism/          <-- Collecting objects of different types using Box
├── video_07b_hybrid_polymorphism/       <-- Adding traits to video 6's solution
├── video_08_state_machine_enums/        <-- Building a simple state machine with enums
├── video_08b_nested_state_machines/     <-- Nesting state machines to control complexity
├── video_09_typestate/                  <-- TypeState pattern for compile time safety 
├── video_10_designing_errors/           <-- Designing custom error enums 
├── video_11_error_chaining/             <-- Designing error chaining (source, map_error vs. from) 
├── video_12_error_contexts/             <-- Adding context information to errors 
```

# 🗺️  Curriculum & Code Index
## 🔹 Phase 1: Ownership and Borrowing and how they affect API design
### 📁 Phase 01: Ownership and Your API (Videos 1 - 5)
Core Concept: Establishing the visual grammar of Rust memory tracking. Learning to read the blueprint canvas and understanding the central permission sigil.

Architectural Lesson: Moving past compiler friction by visualizing exclusive mutations vs. shared borrows.


### 📁 Phase 02: Type Driven Architecture (Videos 6 - 13)
Core Concept: How Selecting your types drives your code design

Architectural Lessons: Enums, The Typestate Pattern and Designing for Failures


### 📁 Phase 03: Behavioral Abstraction (Videos 14-22)
Core Concept: Traits have a LOT of power and can help you in a LOT of ways

Architectural Lessons: Examples: designing type conversion and dual traits, choosing behavior at run time, Closures

### 📁 Phase 04: Architecture (Videos 23 - 26)
Core Concept: Structuring the big parts or your system and how they interact

Architectural Lesson: Design boundaries, dependency injection, type erasure 

### 📁 Phase 05: Advanced Ownership (Videos 27 - ?)
Core Concept: Run-time borrow checking

Architectural Lesson: So many things to think about once we get there!!!! 

** Rest of series to come **

# 🎨 Visual Grammar: The Ownership Sigil
When exploring the code and watching the companion videos, system behaviors are mapped using a centralized architectural sigil. Keep this legend in mind as you trace data flowing through the modules:

* 🌌 **Dotted Blue Vectors:** Denote full **Ownership** transfer. The data is moving to a new context entirely.
* 🟡 **Dashed Yellow Vectors:** Trace a **Shared Borrow** (`&T`). Multiple concurrent paths can safely read and observe this data, but it is immutable.
* 🔴 **Solid Red Vectors:** Trace an **Exclusive, Mutable Borrow** (`&mut T`). The system enters a lockout phase—only this path can alter the data, and all other channels are blocked.

By mapping your tests using these constraints, your Rust designs become intentional instead of accidental.

# 🚀 How to Use This Repository
1. Clone the Masterclass:

```Bash
git clone git clone (https://github.com/wizardcraftcode/SoftwareEngineeringInRust.git)[https://github.com/wizardcraftcode/SoftwareEngineeringInRust.git]
cd SoftwareEngineeringInRust
```

2. Run a Specific Module's Tests:
You don't need to navigate into subfolders to check the code behavior. Run specific workspace targets directly from the root:

```Bash
# Run the test comparison suite from Video 2
cargo test -p video_02_tdd_ownership
``` 
3. Explore the Scenarios:
Open the target module's src/main.rs file alongside the companion YouTube video to see exactly how moving the ownership boundaries changes the compiler's behavioral design.

# 💬 Join the Architecture Discussion

If you have questions about how these structural patterns scale to large production systems, or if you want to debate alternative design constraints:

* 🔔 **Subscribe to the Channel:** [Wizard Craft Code on YouTube](https://tinyurl.com/SEInRustVideos) 
* 💬 **Get Involved:** Drop a comment on the respective video module with your architectural refactoring ideas. We actively read and debate alternative design patterns!