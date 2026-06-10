# 🧙‍♂️ Rust Architecture & Design Masterclass: Visual Blueprints

Welcome to the official code companion repository for the **Rust Architecture Masterclass**. 

This repository is designed like an engineering drafting board. The code here strips away the noise of complex production frameworks to isolate and expose core architectural patterns, memory layouts, and type-system design mechanics in Rust.

📺 **Don't just read the implementation. Watch the system breathe.** Every module in this repository maps directly to a high-fidelity, animation-driven breakdown on YouTube where we trace data permissions, ownership flow, and compiler constraints using custom visual sigils.

▶️ [**Click Here to Watch the Full Masterclass Playlist**](https://tinyurl.com/SEInRustVideos)

---

## 🛠️ Repository Architecture

This repository is organized as a unified **Cargo Workspace**. Each directory corresponds to a specific phase and episode in the architectural masterclass, allowing you to isolate the exact code state seen on your screen.

```text
rust-architecture-masterclass/
├── README.md                   <-- You are here
├── Cargo.toml                  <-- Workspace manifest
├── video_01_the_diagram/       <-- Tracking state via the central sigil
├── video_02_tdd_ownership/     <-- Prototyping data boundaries with tests
```

# 🗺️ Masterclass Curriculum & Code Index
## 🔹 Phase 1: Ownership and Borrowing and how they affect API design
### 📁 Module 01: The Diagram
Core Concept: Establishing the visual grammar of Rust memory tracking. Learning to read the blueprint canvas and understanding the central permission sigil.

Architectural Lesson: Moving past compiler friction by visualizing exclusive mutations vs. shared borrows.


### 📁 Module 02: Designing Rust Ownership Without Guesswork
Core Concept: Test-Driven Development (TDD) as a system design layer, not a verification layer.

Architectural Lesson:  TDD and the borrow checker work together to help you build clean APIs


### 📁 Module 03: Your Getters are Ruining Your Rust Code
Core Concept: Traditional getters give access that causes the borrow checker to be mad

Architectural Lesson: Design your API by what you callers want to do - not by what you are storing.

** Rest of series to come **

# 🎨 Visual Grammar: The Ownership Sigil
When exploring the code and watching the companion videos, system behaviors are mapped using a centralized architectural sigil. Keep this legend in mind as you trace data flowing through the modules:

🔵 Blue Arrows: Denote Ownership.

🔴 Red Vectors: Trace an exclusive, mutable borrow (&mut T) or direct full ownership transfer. All other system pathways are temporarily locked out.

By mapping your tests using these constraints, your Rust designs become intentional instead of accidental.

# 🚀 How to Use This Repository
1. Clone the Masterclass:

```Bash
git clone [https://github.com/YOUR_USERNAME/rust-architecture-masterclass.git](https://github.com/YOUR_USERNAME/rust-architecture-masterclass.git)
cd rust-architecture-masterclass
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

- 🔔 Subscribe for Future Phase Modules

- 💬 Drop a comment on the respective video with your architectural refactoring ideas!