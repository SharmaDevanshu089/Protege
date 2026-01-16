# Requirements Document: Protégé

## Introduction

Protégé is an educational AI coding assistant that inverts the traditional AI helper model through "Pedagogical Inversion" - instead of automating code generation, it automates the Socratic Method. The system forces developers to think deeply about architecture, syntax, and logic by acting as a junior apprentice that asks questions rather than providing direct solutions.

The system operates in two distinct phases:
1. **The Architect (Decision Mode)**: Interrogates users about architectural decisions before code is written
2. **The Apprentice (Wannabe Idiot Mode)**: Watches for coding errors and asks confused questions to force understanding

## Glossary

- **Protégé_System**: The complete educational AI coding assistant application
- **Architect_Mode**: The first phase where architectural decisions are interrogated
- **Apprentice_Mode**: The second phase where the AI acts confused about user errors
- **Ghost_Overlay**: The visual UI element that appears when errors are detected
- **Watcher_Daemon**: The Rust background process monitoring file system changes
- **Roadmap_Generator**: Component that creates project scaffolding plans
- **Interrogation_Engine**: AI component that generates Socratic questions
- **Error_Interceptor**: Component that captures and hides compiler/runtime errors
- **Bedrock_Client**: AWS Bedrock integration for cloud-based AI reasoning
- **Local_AI_Engine**: Rust candle + AMD ROCm for offline AI capability
- **Project_Scaffolder**: Component that generates initial project structure

## Requirements

### Requirement 1: Architectural Decision Interrogation

**User Story:** As a developer, I want to be interrogated about architectural decisions before writing code, so that I understand the trade-offs and make informed choices.

#### Acceptance Criteria

1. WHEN a user inputs a project idea, THE Interrogation_Engine SHALL generate architectural questions based on the project type
2. WHEN a user provides an answer to an architectural question, THE Protégé_System SHALL validate the answer for completeness
3. IF a user indicates they are stuck on a question, THEN THE Interrogation_Engine SHALL provide educational explanations of trade-offs without making the decision
4. WHEN all architectural questions are answered, THE Protégé_System SHALL generate a roadmap.json file containing the architectural decisions
5. THE Interrogation_Engine SHALL ask questions covering database choices, architecture patterns, state management, and deployment strategies

### Requirement 2: Roadmap Generation and Project Scaffolding

**User Story:** As a developer, I want the system to generate a project roadmap and scaffold my Rust project based on my architectural decisions, so that I can start coding with a solid foundation.

#### Acceptance Criteria

1. WHEN architectural decisions are finalized, THE Roadmap_Generator SHALL create a roadmap.json file containing all decisions and implementation steps
2. WHEN roadmap.json is created, THE Project_Scaffolder SHALL generate a Rust project structure matching the architectural decisions
3. THE Project_Scaffolder SHALL create necessary configuration files (Cargo.toml, .gitignore, etc.)
4. THE Project_Scaffolder SHALL generate placeholder modules and files based on the roadmap
5. WHEN scaffolding is complete, THE Protégé_System SHALL display a summary of created files and next steps

### Requirement 3: File System Monitoring

**User Story:** As a developer, I want the system to monitor my code changes in real-time, so that it can detect errors and provide educational feedback immediately.

#### Acceptance Criteria

1. WHEN Apprentice_Mode is activated, THE Watcher_Daemon SHALL monitor all project files for changes using the notify library
2. WHEN a file is modified, THE Watcher_Daemon SHALL trigger compilation or validation within 500ms
3. THE Watcher_Daemon SHALL run as a lightweight background process consuming minimal system resources
4. WHEN the Watcher_Daemon detects a compilation error, THE Error_Interceptor SHALL capture the error before it reaches the user's terminal
5. THE Watcher_Daemon SHALL utilize AMD Ryzen AI NPU when available to minimize battery consumption

### Requirement 4: Error Interception and Hiding

**User Story:** As a developer, I want my compilation errors to be intercepted and hidden, so that I am forced to think through problems rather than copy-paste solutions.

#### Acceptance Criteria

1. WHEN a compilation error occurs, THE Error_Interceptor SHALL capture the complete error message
2. WHEN an error is captured, THE Error_Interceptor SHALL prevent the error from displaying in the user's terminal or IDE
3. THE Error_Interceptor SHALL parse error messages to extract error type, location, and context
4. WHEN an error is intercepted, THE Protégé_System SHALL trigger the Ghost_Overlay to appear
5. THE Error_Interceptor SHALL support Rust compiler errors, borrow checker errors, and runtime panics

### Requirement 5: Socratic Questioning Through Ghost Overlay

**User Story:** As a developer, I want the AI to ask me confused questions about my errors, so that I am forced to understand and explain the solution myself.

#### Acceptance Criteria

1. WHEN an error is intercepted, THE Ghost_Overlay SHALL appear over the user's editor with a semi-transparent visual effect
2. WHEN the Ghost_Overlay appears, THE Interrogation_Engine SHALL generate a confused question related to the specific error
3. THE Interrogation_Engine SHALL phrase questions from the perspective of a confused apprentice (e.g., "Wait, Boss... you moved data into that thread. Doesn't that mean we can't use it here?")
4. WHEN a user provides an explanation, THE Protégé_System SHALL validate whether the explanation demonstrates understanding
5. IF the explanation is insufficient, THEN THE Interrogation_Engine SHALL ask follow-up questions to guide understanding
6. WHEN the user demonstrates sufficient understanding, THE Ghost_Overlay SHALL disappear and reveal the actual error message
7. THE Ghost_Overlay SHALL provide a "Give Up" option that reveals the error after 3 failed explanation attempts

### Requirement 6: Multi-Modal AI Intelligence

**User Story:** As a developer, I want the system to use cloud AI for complex reasoning and local AI for privacy-sensitive operations, so that I get the best of both worlds.

#### Acceptance Criteria

1. WHEN complex architectural reasoning is required, THE Bedrock_Client SHALL send requests to AWS Bedrock (Claude 3.5)
2. WHEN the user is offline or requests privacy mode, THE Local_AI_Engine SHALL handle all AI operations using Rust candle and AMD ROCm
3. THE Protégé_System SHALL detect network connectivity and automatically switch between cloud and local AI
4. WHEN using cloud AI, THE Bedrock_Client SHALL not transmit user code without explicit permission
5. THE Local_AI_Engine SHALL load AI models on first use and cache them for subsequent operations
6. THE Protégé_System SHALL allow users to configure AI mode preference (cloud-first, local-first, or cloud-only)

### Requirement 7: Tauri Application Interface

**User Story:** As a developer, I want a native desktop application with high performance, so that the educational experience is smooth and responsive.

#### Acceptance Criteria

1. THE Protégé_System SHALL be built using Tauri v2 for native desktop performance
2. THE Protégé_System SHALL use Svelte 5 for the frontend UI components
3. WHEN the application starts, THE Protégé_System SHALL display a mode selection screen (Architect Mode or Apprentice Mode)
4. THE Protégé_System SHALL provide a settings panel for configuring AI preferences, project paths, and watcher behavior
5. THE Protégé_System SHALL support Windows, macOS, and Linux operating systems
6. THE Protégé_System SHALL have a system tray icon for quick access to pause/resume the Watcher_Daemon

### Requirement 8: Architectural Question Database

**User Story:** As a developer, I want the system to ask relevant architectural questions based on my project type, so that I consider all important design decisions.

#### Acceptance Criteria

1. THE Interrogation_Engine SHALL maintain a database of architectural questions categorized by project type (web app, CLI tool, library, etc.)
2. WHEN a project type is identified, THE Interrogation_Engine SHALL select relevant questions from the database
3. THE Interrogation_Engine SHALL ask questions about database selection (SQL vs NoSQL), state management, error handling strategies, and concurrency models
4. WHEN a user answers a question, THE Interrogation_Engine SHALL use the answer to inform subsequent questions
5. THE Interrogation_Engine SHALL support custom question sets that users can define for specific domains

### Requirement 9: Error Context Analysis

**User Story:** As a developer, I want the AI to understand the context of my errors, so that it can ask relevant and helpful questions.

#### Acceptance Criteria

1. WHEN an error is intercepted, THE Error_Interceptor SHALL analyze the surrounding code context (10 lines before and after)
2. THE Error_Interceptor SHALL identify the error category (borrow checker, type mismatch, lifetime, trait bounds, etc.)
3. WHEN generating questions, THE Interrogation_Engine SHALL use both the error message and code context
4. THE Interrogation_Engine SHALL reference specific variable names and code patterns from the user's code in questions
5. THE Interrogation_Engine SHALL avoid generic questions and tailor each question to the specific error scenario

### Requirement 10: Progress Tracking and Learning Analytics

**User Story:** As a developer, I want to track my learning progress and see which concepts I struggle with, so that I can focus my learning efforts.

#### Acceptance Criteria

1. THE Protégé_System SHALL track each error type encountered and how many attempts were needed to explain it
2. THE Protégé_System SHALL maintain a learning journal showing architectural decisions made and their rationales
3. WHEN a user requests analytics, THE Protégé_System SHALL display statistics on error categories, explanation success rate, and learning velocity
4. THE Protégé_System SHALL identify recurring error patterns and suggest focused learning resources
5. THE Protégé_System SHALL store all analytics data locally and provide an option to export for personal review

### Requirement 11: Roadmap Validation and Iteration

**User Story:** As a developer, I want to review and modify my architectural roadmap before implementation, so that I can refine my decisions.

#### Acceptance Criteria

1. WHEN roadmap.json is generated, THE Protégé_System SHALL display it in a readable format with explanations
2. THE Protégé_System SHALL allow users to edit architectural decisions in the roadmap
3. WHEN a roadmap decision is changed, THE Interrogation_Engine SHALL ask follow-up questions about the implications
4. THE Protégé_System SHALL validate that the roadmap is internally consistent (e.g., chosen libraries are compatible)
5. WHEN the user approves the roadmap, THE Project_Scaffolder SHALL proceed with project generation

### Requirement 12: Explanation Quality Assessment

**User Story:** As a developer, I want the system to accurately assess whether I understand the problem, so that I'm not forced to repeat explanations unnecessarily.

#### Acceptance Criteria

1. WHEN a user provides an explanation, THE Interrogation_Engine SHALL analyze it for key concepts related to the error
2. THE Interrogation_Engine SHALL check if the explanation mentions the root cause, not just symptoms
3. IF an explanation is partially correct, THEN THE Interrogation_Engine SHALL ask targeted follow-up questions about missing concepts
4. THE Interrogation_Engine SHALL accept multiple valid explanation approaches (e.g., different ways to fix a borrow checker error)
5. WHEN an explanation demonstrates understanding, THE Protégé_System SHALL provide positive reinforcement before revealing the error

### Requirement 13: Daemon Lifecycle Management

**User Story:** As a developer, I want to control when the watcher daemon is active, so that I can work without interruption when needed.

#### Acceptance Criteria

1. THE Protégé_System SHALL provide commands to start, stop, and restart the Watcher_Daemon
2. WHEN the Watcher_Daemon is stopped, THE Protégé_System SHALL not intercept errors or show the Ghost_Overlay
3. THE Protégé_System SHALL remember the daemon state across application restarts
4. THE Watcher_Daemon SHALL gracefully shut down when the application closes, releasing all file system watches
5. THE Protégé_System SHALL display the daemon status (running, stopped, error) in the system tray icon

### Requirement 14: Project Type Detection

**User Story:** As a developer, I want the system to automatically detect what type of project I'm building, so that it asks relevant questions.

#### Acceptance Criteria

1. WHEN a user inputs a project idea, THE Protégé_System SHALL analyze keywords to determine project type
2. THE Protégé_System SHALL support detection of web applications, CLI tools, libraries, embedded systems, and game projects
3. IF project type is ambiguous, THEN THE Protégé_System SHALL ask clarifying questions
4. WHEN project type is determined, THE Interrogation_Engine SHALL load the appropriate question set
5. THE Protégé_System SHALL allow users to manually override the detected project type

### Requirement 15: Educational Resource Integration

**User Story:** As a developer, I want access to learning resources when I'm stuck, so that I can learn concepts without leaving the application.

#### Acceptance Criteria

1. WHEN a user indicates they are stuck on an architectural question, THE Protégé_System SHALL provide links to relevant documentation
2. THE Protégé_System SHALL maintain a curated library of Rust learning resources organized by topic
3. WHEN an error pattern repeats, THE Protégé_System SHALL suggest specific chapters from "The Rust Book" or other resources
4. THE Protégé_System SHALL support adding custom learning resources that users find helpful
5. THE Protégé_System SHALL display resources in an embedded browser panel without requiring external navigation
