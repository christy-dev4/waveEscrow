# Wave Program & WaveEscrow — Complete Implementation Plan

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [The Wave Program Model](#2-the-wave-program-model)
   - 2.1 [What Is a Wave?](#21-what-is-a-wave)
   - 2.2 [Sprint Lifecycle](#22-sprint-lifecycle)
   - 2.3 [Roles & Responsibilities](#23-roles--responsibilities)
   - 2.4 [Reward Economics](#24-reward-economics)
3. [Types of Work in a Wave](#3-types-of-work-in-a-wave)
   - 3.1 [Bug Fixes](#31-bug-fixes)
   - 3.2 [New Features](#32-new-features)
   - 3.3 [Documentation](#33-documentation)
   - 3.4 [Testing & Quality Assurance](#34-testing--quality-assurance)
   - 3.5 [Refactoring & Technical Debt](#35-refactoring--technical-debt)
   - 3.6 [DevOps & Infrastructure](#36-devops--infrastructure)
   - 3.7 [Security Audits & Review](#37-security-audits--review)
   - 3.8 [Community & Governance](#38-community--governance)
   - 3.9 [Research & Prototyping](#39-research--prototyping)
   - 3.10 [Translation & Localization](#310-translation--localization)
   - 3.11 [Design & UX](#311-design--ux)
   - 3.12 [Analytics & Reporting](#312-analytics--reporting)
4. [WaveEscrow Smart Contract Architecture](#4-waveescrow-smart-contract-architecture)
   - 4.1 [Core Contract Design](#41-core-contract-design)
   - 4.2 [Storage Layout](#42-storage-layout)
   - 4.3 [Function Specification](#43-function-specification)
   - 4.4 [Access Control Model](#44-access-control-model)
   - 4.5 [Event System](#45-event-system)
   - 4.6 [Error Handling](#46-error-handling)
5. [Drips GitHub App Integration](#5-drips-github-app-integration)
   - 5.1 [Verification Flow](#51-verification-flow)
   - 5.2 [Oracle Architecture](#52-oracle-architecture)
   - 5.3 [Point Calculation Engine](#53-point-calculation-engine)
   - 5.4 [Webhook System](#54-webhook-system)
6. [Sprint Cycle Implementation](#6-sprint-cycle-implementation)
   - 6.1 [Wave Planning Phase](#61-wave-planning-phase)
   - 6.2 [Sprint Execution Phase](#62-sprint-execution-phase)
   - 6.3 [Review & Finalization Phase](#63-review--finalization-phase)
   - 6.4 [Reward Distribution Phase](#64-reward-distribution-phase)
7. [Point System Design](#7-point-system-design)
   - 7.1 [Point Weighting by Work Type](#71-point-weighting-by-work-type)
   - 7.2 [Difficulty Tiers](#72-difficulty-tiers)
   - 7.3 [Quality Multipliers](#73-quality-multipliers)
   - 7.4 [Bonus Points](#74-bonus-points)
8. [Token Economics](#8-token-economics)
   - 8.1 [Reward Pool Sizing](#81-reward-pool-sizing)
   - 8.2 [USDC/USDT Integration](#82-usdcusdt-integration)
   - 8.3 [Pro-Rata Distribution Math](#83-pro-rata-distribution-math)
   - 8.4 [Fee Model](#84-fee-model)
9. [Security Architecture](#9-security-architecture)
   - 9.1 [Contract Security](#91-contract-security)
   - 9.2 [Oracle Security](#92-oracle-security)
   - 9.3 [Upgrade Path](#93-upgrade-path)
   - 9.4 [Emergency Procedures](#94-emergency-procedures)
10. [Testing Strategy](#10-testing-strategy)
    - 10.1 [Unit Testing](#101-unit-testing)
    - 10.2 [Integration Testing](#102-integration-testing)
    - 10.3 [Fuzz Testing](#103-fuzz-testing)
    - 10.4 [Formal Verification](#104-formal-verification)
    - 10.5 [Testnet Deployment](#105-testnet-deployment)
11. [Deployment Plan](#11-deployment-plan)
    - 11.1 [Environment Configuration](#111-environment-configuration)
    - 11.2 [Deployment Pipeline](#112-deployment-pipeline)
    - 11.3 [Post-Deployment Verification](#113-post-deployment-verification)
12. [Monitoring & Operations](#12-monitoring--operations)
    - 12.1 [On-Chain Monitoring](#121-on-chain-monitoring)
    - 12.2 [Off-Chain Services](#122-off-chain-services)
    - 12.3 [Alerting](#123-alerting)
13. [Governance & Upgrade Path](#13-governance--upgrade-path)
    - 13.1 [DAO Integration](#131-dao-integration)
    - 13.2 [Parameter Adjustments](#132-parameter-adjustments)
    - 13.3 [Contract Upgrades](#133-contract-upgrades)
14. [Risk Assessment](#14-risk-assessment)
    - 14.1 [Smart Contract Risks](#141-smart-contract-risks)
    - 14.2 [Oracle Risks](#142-oracle-risks)
    - 14.3 [Economic Risks](#143-economic-risks)
    - 14.4 [Mitigation Strategies](#144-mitigation-strategies)
15. [Roadmap](#15-roadmap)
    - 15.1 [Phase 1: Foundation](#151-phase-1-foundation)
    - 15.2 [Phase 2: Pilot](#152-phase-2-pilot)
    - 15.3 [Phase 3: Production](#153-phase-3-production)
    - 15.4 [Phase 4: Ecosystem](#154-phase-4-ecosystem)
16. [Appendices](#16-appendices)
    - 16.1 [Glossary](#161-glossary)
    - 16.2 [Reference Implementations](#162-reference-implementations)
    - 16.3 [Useful Commands](#163-useful-commands)

---

## 1. Executive Summary

The Wave Program is a structured incentive framework that bridges open-source software contribution with decentralized finance (DeFi) rewards. It operates on the principle that software contributions can be objectively measured, scored, and compensated through a transparent, on-chain mechanism. At the heart of this system is the WaveEscrow contract — an Arbitrum Stylus smart contract that acts as a trustless escrow for reward pools, releasing funds to contributors only after the Drips GitHub App verifies the points earned during a sprint cycle.

This document serves as the comprehensive implementation plan for the entire Wave Program ecosystem. It covers the conceptual model of how work is structured and rewarded, the smart contract architecture that secures funds, the integration with GitHub for contribution tracking, the point system that quantifies contributions, and the operational procedures for running successful Wave cycles.

The Wave Program is designed for ecosystem foundations, DAOs, and open-source projects that want to create sustainable, merit-based reward systems for their contributors. By automating the reward distribution process through smart contracts and GitHub integration, it eliminates manual payout overhead, reduces dispute potential, and provides contributors with guaranteed, transparent compensation for their work.

---

## 2. The Wave Program Model

### 2.1 What Is a Wave?

A "Wave" is a time-bound sprint cycle during which maintainers post scoped issues and contributors earn points by completing them. Each Wave has:

- **A defined time window**: A start and end timestamp that bounds the sprint period.
- **A fixed reward pool**: A predetermined budget denominated in USDC or USDT that will be distributed among contributors.
- **A set of scoped issues**: Clearly defined tasks that are posted before or at the start of the Wave.
- **A point budget**: Total points allocated to the Wave, against which individual contributions are measured.
- **An oracle or verifier**: The Drips GitHub App (or a decentralized oracle) that verifies completed work and assigns points.

Waves run sequentially or in parallel, depending on the ecosystem's capacity. Each Wave is self-contained — its reward pool is independent of other Waves, and contributors are scored only against the work completed within that specific Wave's time window.

### 2.2 Sprint Lifecycle

Each Wave progresses through four distinct phases:

**Phase 1 — Wave Planning (Days 1–3):**
- The ecosystem foundation (Wave owner) determines the total budget for the Wave.
- Maintainers identify and scope issues that need to be completed.
- Issues are tagged with difficulty ratings and point values.
- The Wave is initialized on-chain via the WaveEscrow contract: the owner calls `initialize_wave(wave_id, budget, end_timestamp)` and deposits the reward pool.
- Contributors review the available issues and signal interest.
- The Drips GitHub App registers the Wave and begins tracking.

**Phase 2 — Sprint Execution (Days 4–24):**
- Contributors pick up scoped issues and begin working.
- Issues are tracked through GitHub: assignment, pull requests, reviews, and merges.
- The Drips GitHub App monitors all activity through webhooks.
- Intermediate milestones can be recorded but points are not yet finalized.
- Maintainers provide feedback and request changes as needed.
- The WaveEscrow contract holds the reward pool securely throughout this phase.

**Phase 3 — Review & Finalization (Days 25–28):**
- The sprint window closes. No new contributions are accepted for this Wave.
- Maintainers conduct final reviews of all pending work.
- Each contributor's completed work is scored according to the point system.
- The total points for the Wave are calculated.
- The Drips GitHub App oracle signs a payload containing:
  - The Wave ID
  - The total points earned across all contributors
  - Each contributor's individual points
- The oracle calls `finalize_wave_points(wave_id, total_points)` on the WaveEscrow contract, locking in the point distribution.

**Phase 4 — Reward Distribution (Days 29–30):**
- Contributors can now claim their rewards by calling `claim_reward(wave_id, contributor_points)`.
- The contract calculates the pro-rata share: `reward = budget * contributor_points / total_points`.
- USDC/USDT is transferred from the contract to each contributor.
- Unclaimed rewards after a grace period may be returned to the ecosystem foundation.
- A full Wave report is generated for transparency.

### 2.3 Roles & Responsibilities

**Ecosystem Foundation / Wave Owner:**
- Sets the strategic direction for each Wave.
- Provides the reward pool funding (USDC/USDT).
- Selects and appoints maintainers.
- Initializes Waves on the WaveEscrow contract.
- Has emergency powers (pause, emergency withdraw).
- Can transfer ownership and update the oracle address.
- Ultimately responsible for the Wave's success.

**Maintainers:**
- Scope and define issues with clear acceptance criteria.
- Assign point values to issues based on difficulty and impact.
- Review pull requests and provide constructive feedback.
- Verify completed work and approve point assignments.
- Act as quality gatekeepers for the Wave.
- Collaborate with the oracle to finalize point totals.
- May also contribute code (with appropriate separation of concerns).

**Contributors:**
- Select and pick up scoped issues.
- Complete the work according to the issue specifications.
- Submit pull requests and iterate on feedback.
- Earn points based on completed, accepted work.
- Claim rewards after the Wave is finalized.
- Build reputation across multiple Waves.

**Drips GitHub App / Oracle:**
- Monitors GitHub activity through webhooks and API polling.
- Tracks issue assignments, pull requests, reviews, and merges.
- Verifies that work was completed within the Wave's time window.
- Calculates point totals based on completed issues and their assigned values.
- Generates a signed attestation of the final point distribution.
- Submits the attestation to the WaveEscrow contract.
- May operate as a centralized service initially, with a path toward decentralization.

**Arbitrum Stylus Network:**
- Provides the execution environment for the WaveEscrow contract.
- Offers low gas costs suitable for periodic reward distribution.
- Supports Rust-based smart contracts via the Stylus SDK.
- Provides L2 security guarantees anchored to Ethereum.

### 2.4 Reward Economics

The reward economics of a Wave are designed to be simple, transparent, and fair:

**Total Reward Pool:** The ecosystem foundation deposits a fixed amount of USDC/USDT into the WaveEscrow contract at the start of the Wave. This pool is known to all contributors in advance.

**Point System:** Each issue is assigned a point value based on its difficulty, scope, and type. When a contributor completes an issue, they earn those points. The sum of all earned points across all contributors is the total wave points.

**Pro-Rata Distribution:** Each contributor's reward is proportional to their share of the total points:
```
reward_i = total_budget * (points_i / total_points)
```

This means:
- If you earn 10% of the total points, you get 10% of the reward pool.
- Rewards are deterministic once points are finalized.
- No negotiation or dispute about payout amounts — the math is fixed.
- Contributors can estimate their payout as they earn points.

**Example:** A Wave with a $50,000 budget and 1,000 total points:
- Contributor A earns 300 points → receives $15,000
- Contributor B earns 500 points → receives $25,000
- Contributor C earns 200 points → receives $10,000

---

## 3. Types of Work in a Wave

The Wave Program supports a wide variety of contribution types. Each type has its own characteristics, review criteria, and point weighting. Below is a detailed taxonomy of work types that can be scoped into a Wave.

### 3.1 Bug Fixes

Bug fixes are the most common and immediately valuable type of contribution. They involve identifying, reproducing, and resolving defects in the codebase.

**Characteristics:**
- Typically well-scoped with clear reproduction steps.
- Point values depend on severity (critical, high, medium, low).
- Fixes may range from a single line change to substantial refactoring.
- Requires passing existing tests and adding regression tests.

**Scoping Guidelines:**
- Each bug issue must include: environment details, steps to reproduce, expected vs. actual behavior, and suggested fix approach.
- Critical bugs (security vulnerabilities, data loss, broken core functionality) should be weighted higher.
- Bugs discovered during a Wave by non-contributors can be added to the issue tracker with appropriate priority.

**Point Weighting:**
- Critical/Security: 50–200 points depending on complexity
- High severity: 20–80 points
- Medium severity: 10–40 points
- Low severity: 5–15 points

**Review Criteria:**
- Does the fix actually resolve the reported issue?
- Are there regression tests covering the fix?
- Does the fix follow the project's coding standards?
- Could the fix introduce new bugs in edge cases?
- Is the root cause addressed, not just the symptom?

**Examples:**
- Fixing a null pointer dereference that causes a node to crash
- Resolving a race condition in the transaction pool
- Correcting incorrect fee calculation in a DeFi protocol
- Patching an XSS vulnerability in the frontend
- Fixing broken API endpoint that returns wrong HTTP status codes
- Addressing memory leak in long-running background worker
- Fixing incorrect sorting behavior in list views
- Resolving authentication session expiry handling bug

### 3.2 New Features

New features extend the functionality of the project, adding capabilities that did not previously exist.

**Characteristics:**
- Larger scope than bug fixes, often requiring design discussion.
- May involve multiple files and components.
- Requires comprehensive testing and documentation.
- Often includes both backend and frontend changes.
- May need migration paths for existing users/data.

**Scoping Guidelines:**
- Feature issues must include: problem statement, proposed solution, acceptance criteria, and technical approach sketch.
- Features should be broken down into the smallest possible increments.
- Each feature issue should be completable within the Wave's sprint window.
- Complex features may need to be split across multiple Waves.

**Point Weighting:**
- Large features (multiple weeks of work): 100–500 points
- Medium features (several days): 40–100 points
- Small features (1–2 days): 15–40 points
- Micro features (hours): 5–15 points

**Review Criteria:**
- Does the feature meet all specified acceptance criteria?
- Is the implementation consistent with the project's architecture?
- Are there adequate unit, integration, and end-to-end tests?
- Is documentation provided (README, API docs, usage examples)?
- Are error conditions handled gracefully?
- Is the performance impact acceptable?
- Does it maintain backward compatibility?

**Examples:**
- Implementing a GraphQL API endpoint for querying user analytics
- Building a dashboard component for visualizing protocol metrics
- Adding WebSocket support for real-time event streaming
- Implementing batch processing for large dataset operations
- Creating a CLI tool for automated deployment
- Adding multi-language support (i18n framework)
- Implementing export/import functionality for user data
- Building a notification system with email and push channels

### 3.3 Documentation

Documentation is critical for project adoption and contributor onboarding. It encompasses all written materials that help users and developers understand and use the project.

**Characteristics:**
- Requires deep understanding of the subject matter.
- May need technical writing skills in addition to coding knowledge.
- Often overlooked but highly valuable for project health.
- Can be scoped to specific modules, features, or workflows.
- Includes code comments, API docs, tutorials, and guides.

**Scoping Guidelines:**
- Documentation issues should specify the target audience (end users, developers, operators).
- Include links to existing documentation to avoid duplication.
- Specify format requirements (Markdown, reStructuredText, Jupyter notebooks).
- Identify any code examples or diagrams that need to be created.

**Point Weighting:**
- Comprehensive documentation rewrite: 50–200 points
- Module/feature documentation: 20–60 points
- Tutorial or getting-started guide: 15–40 points
- API reference documentation: 10–30 points
- README or contributing guide updates: 5–15 points
- Code comment improvements: 3–10 points

**Review Criteria:**
- Is the documentation accurate and technically correct?
- Is it written clearly for the target audience?
- Are code examples tested and working?
- Is the formatting consistent with project standards?
- Are there diagrams or visual aids where helpful?
- Are edge cases and error conditions documented?

**Examples:**
- Writing comprehensive API reference with request/response examples
- Creating a step-by-step tutorial for new contributors
- Documenting the deployment process with infrastructure diagrams
- Writing architecture decision records (ADRs) for major components
- Creating troubleshooting guides for common issues
- Documenting database schema and migration patterns
- Writing security audit documentation and threat models
- Creating contributor onboarding documentation

### 3.4 Testing & Quality Assurance

Testing and QA work ensures the reliability, correctness, and performance of the codebase. This is a high-value contribution type that directly impacts user trust.

**Characteristics:**
- Can be independent of feature development.
- Requires understanding of testing frameworks and methodologies.
- Includes both automated and manual testing.
- May involve setting up test infrastructure.
- Often reveals bugs and edge cases in existing code.

**Scoping Guidelines:**
- Specify which modules or functionality need test coverage.
- Define the type of testing: unit, integration, end-to-end, performance.
- Set coverage targets that are realistic and measurable.
- Include test infrastructure setup in the scope.

**Point Weighting:**
- Full test suite for a major module: 40–120 points
- Comprehensive integration tests: 25–80 points
- Performance/benchmark tests: 20–50 points
- Fuzz testing setup: 30–70 points
- Unit tests for specific functionality: 5–20 points
- Test infrastructure/tooling: 15–40 points

**Review Criteria:**
- Do tests actually verify the intended behavior?
- Are edge cases and boundary conditions covered?
- Are test names descriptive and test structure organized?
- Do tests run reliably (not flaky)?
- Is there appropriate use of mocks/stubs?
- Is performance test data representative of production?

**Examples:**
- Writing comprehensive unit tests for the WaveEscrow contract
- Setting up fuzz testing for input validation functions
- Creating integration tests for the full claim flow
- Building a load testing suite for API endpoints
- Writing snapshot tests for UI components
- Implementing property-based testing for mathematical operations
- Setting up continuous integration test pipeline
- Writing end-to-end tests for critical user journeys

### 3.5 Refactoring & Technical Debt

Refactoring improves the internal structure of the codebase without changing external behavior. It reduces technical debt and improves maintainability.

**Characteristics:**
- Requires deep understanding of existing code.
- Must not introduce behavioral changes.
- Benefits are often long-term and invisible to end users.
- May require significant testing to ensure no regressions.
- Often motivated by upcoming feature work that would be difficult with current architecture.

**Scoping Guidelines:**
- Clearly define the current problem and the target state.
- Specify which modules or code areas are in scope.
- Define acceptance criteria in terms of code quality metrics.
- Include migration plan if API changes are involved.

**Point Weighting:**
- Major architectural refactor: 80–300 points
- Module-level refactoring: 30–100 points
- Code cleanup and linting: 5–20 points
- Dependency upgrades with migration: 20–60 points
- Performance optimization: 20–80 points

**Review Criteria:**
- Does the refactored code maintain the same external behavior?
- Is the new code cleaner, more maintainable, and better documented?
- Are all existing tests passing?
- Has cyclomatic complexity decreased?
- Has code duplication been reduced?
- Are there appropriate abstractions without over-engineering?

**Examples:**
- Extracting a monolithic function into smaller, testable modules
- Replacing custom implementation with well-tested library
- Upgrading deprecated dependencies and migrating code
- Reducing code duplication across similar components
- Improving error handling patterns throughout the codebase
- Optimizing hot paths identified by profiling
- Restructuring project directory layout for better organization
- Migrating from JavaScript to TypeScript

### 3.6 DevOps & Infrastructure

DevOps work encompasses the infrastructure, deployment pipelines, and operational tooling that keeps the project running reliably.

**Characteristics:**
- Often cross-cutting and affects all other work.
- Requires infrastructure-as-code skills.
- Involves security considerations (access control, secrets management).
- May require on-call rotation or incident response.
- Directly impacts developer productivity and user experience.

**Scoping Guidelines:**
- Specify the infrastructure components involved.
- Define performance and reliability targets (SLAs, SLOs).
- Include security review as part of acceptance.
- Document any cost implications.

**Point Weighting:**
- Infrastructure setup from scratch: 60–200 points
- CI/CD pipeline improvements: 20–60 points
- Monitoring and alerting setup: 25–70 points
- Containerization and orchestration: 30–80 points
- Security hardening: 30–100 points
- Automation scripts and tooling: 10–40 points

**Review Criteria:**
- Is the infrastructure defined as code?
- Are there appropriate health checks and monitoring?
- Are secrets properly managed (not hardcoded)?
- Is there documentation for operational procedures?
- Are rollback procedures defined?
- Is the setup reproducible (idempotent)?

**Examples:**
- Setting up Kubernetes cluster with Helm charts
- Configuring CI/CD pipeline with GitHub Actions
- Implementing infrastructure monitoring with Prometheus/Grafana
- Setting up log aggregation and alerting
- Automating database backup and recovery
- Configuring CDN and caching layers
- Implementing zero-downtime deployment strategy
- Setting up development, staging, and production environments

### 3.7 Security Audits & Review

Security work involves identifying vulnerabilities, reviewing code for security issues, and implementing security improvements.

**Characteristics:**
- Requires specialized security expertise.
- Often has strict confidentiality requirements.
- May involve both automated tools and manual review.
- Results in actionable findings that need remediation.
- Critical for DeFi and financial applications.

**Scoping Guidelines:**
- Define the scope of the audit (specific contracts, modules, or entire codebase).
- Specify the threat model and assumptions.
- Include timeline for both audit and remediation.
- Define severity classification for findings.

**Point Weighting:**
- Full security audit: 100–400 points
- Targeted security review: 40–150 points
- Penetration testing: 50–200 points
- Vulnerability remediation: 20–80 points per finding
- Security tooling and automation: 30–100 points

**Review Criteria:**
- Are all findings properly documented with reproduction steps?
- Are severity levels correctly assigned?
- Are remediation recommendations clear and actionable?
- Have all critical/high findings been addressed?
- Are there regression tests for fixed vulnerabilities?
- Is there a security checklist or review process?

**Examples:**
- Auditing the WaveEscrow contract for common vulnerability patterns
- Reviewing access control implementation for privilege escalation
- Analyzing reentrancy protection in token transfer functions
- Performing economic attack analysis on reward distribution
- Reviewing oracle integration for manipulation resistance
- Conducting dependency vulnerability scan
- Implementing emergency pause and upgrade mechanisms
- Writing security documentation and threat model

### 3.8 Community & Governance

Community work involves activities that build, maintain, and grow the project's user and contributor community.

**Characteristics:**
- Less technical, more people-focused.
- Requires good communication and moderation skills.
- Directly impacts project health and sustainability.
- May involve cross-team coordination.
- Often includes creating processes and documentation.

**Scoping Guidelines:**
- Define the community goals and metrics.
- Specify the target audience and engagement channels.
- Include timeline for outreach activities.
- Define success criteria in measurable terms.

**Point Weighting:**
- Community program design: 30–100 points
- Event organization (hackathons, meetups): 40–150 points
- Moderation and community management (per Wave): 20–60 points
- Content creation (blog posts, newsletters): 10–40 points
- Contributor onboarding and mentorship: 15–50 points

**Review Criteria:**
- Are community engagement metrics showing positive trends?
- Is there documentation of processes and guidelines?
- Are community members reporting positive experiences?
- Is there a plan for sustainability beyond the current Wave?

**Examples:**
- Organizing a hackathon for new feature development
- Creating a contributor mentorship program
- Writing weekly ecosystem updates or newsletters
- Managing Discord/Telegram communities and handling support
- Creating governance proposals and managing voting
- Building contributor recognition program
- Organizing community calls and AMAs
- Creating onboarding materials for new contributors

### 3.9 Research & Prototyping

Research work explores new technologies, approaches, or optimizations that may benefit the project in the future.

**Characteristics:**
- Outcomes are less predictable than other work types.
- May not result in production code.
- Requires strong analytical and experimental skills.
- Often involves writing detailed reports.
- Informs decision-making for future development.

**Scoping Guidelines:**
- Define the research question or hypothesis.
- Specify the methodology and experiments.
- Define the deliverable (report, prototype, benchmark results).
- Set boundaries to prevent scope creep.

**Point Weighting:**
- Major research initiative: 50–200 points
- Technology evaluation and comparison: 20–60 points
- Prototype implementation: 30–100 points
- Benchmarking and performance analysis: 15–50 points
- Literature review and report: 10–30 points

**Review Criteria:**
- Is the research question clearly answered?
- Are the methodology and results reproducible?
- Is the report well-structured and actionable?
- Are recommendations supported by evidence?
- Is prototype code documented and tested?

**Examples:**
- Researching and comparing L2 scaling solutions for deployment
- Prototyping alternative reward distribution mechanisms
- Benchmarking different database technologies for analytics
- Evaluating ZK-proof systems for oracle verification
- Researching MEV mitigation strategies
- Analyzing tokenomics models for sustainability
- Prototyping cross-chain bridge integration
- Studying contributor retention patterns in open source

### 3.10 Translation & Localization

Translation work makes the project accessible to non-English speaking users and contributors, expanding the project's global reach.

**Characteristics:**
- Requires fluency in both source and target languages.
- May need technical domain knowledge for accurate translation.
- Can be done independently of code changes.
- Often involves maintaining translation files and workflows.
- Benefits from review by multiple translators.

**Scoping Guidelines:**
- Specify the target languages and priority order.
- Define the content scope (UI, documentation, blog posts).
- Include glossary of technical terms for consistency.
- Set quality standards and review process.

**Point Weighting:**
- Full documentation translation: 30–100 points per language
- UI string translation: 10–40 points per language
- Blog/content translation: 5–20 points per piece
- Translation review and quality assurance: 10–30 points
- Translation tooling and workflow setup: 15–40 points

**Review Criteria:**
- Is the translation accurate and technically correct?
- Does it maintain consistent terminology?
- Are formatting and links preserved?
- Does it read naturally in the target language?
- Has it been reviewed by a second translator?

**Examples:**
- Translating smart contract documentation into Spanish
- Localizing web UI into Japanese and Korean
- Translating deployment guides into Mandarin
- Creating French-language tutorial videos
- Translating security audit reports into Russian
- Building i18n workflow with automated translation checks
- Creating bilingual glossary of technical terms
- Translating community guidelines and code of conduct

### 3.11 Design & UX

Design work focuses on the user experience, visual design, and interaction patterns of the project's user-facing components.

**Characteristics:**
- Requires design skills and tools proficiency.
- May need to work closely with developers for implementation.
- Involves user research and testing.
- Results in design assets, prototypes, or implemented designs.
- Directly impacts user adoption and satisfaction.

**Scoping Guidelines:**
- Define the user personas and use cases.
- Specify design deliverables (wireframes, mockups, prototypes).
- Include user research methodology if applicable.
- Define design system components that need creation.

**Point Weighting:**
- Full product redesign: 60–250 points
- Feature design (research through implementation): 30–120 points
- Design system components: 10–40 points
- User research and usability testing: 20–60 points
- Visual assets and illustrations: 5–25 points

**Review Criteria:**
- Does the design solve the identified user problem?
- Is it consistent with the existing design system?
- Are interactions intuitive and accessible?
- Has the design been tested with users?
- Are design assets properly documented for developers?

**Examples:**
- Redesigning the dashboard for better data visualization
- Creating wireframes for the reward claim flow
- Designing dark mode theme for the UI
- Conducting usability testing on the onboarding flow
- Creating an icon set for the design system
- Designing mobile-responsive layout for key pages
- Creating motion design for loading states and transitions
- Designing accessible color palette meeting WCAG 2.1 AA

### 3.12 Analytics & Reporting

Analytics work involves setting up data collection, creating dashboards, and generating reports that inform decision-making.

**Characteristics:**
- Requires data analysis and visualization skills.
- Involves both backend (data pipeline) and frontend (dashboard) work.
- Privacy and data governance considerations.
- Outputs are used by project leadership and the community.
- Often involves querying blockchain data directly.

**Scoping Guidelines:**
- Define the key metrics and KPIs to track.
- Specify data sources and collection methods.
- Define dashboard and report requirements.
- Include data freshness and accuracy requirements.

**Point Weighting:**
- Analytics infrastructure setup: 30–100 points
- Dashboard creation: 15–50 points
- Automated report generation: 20–60 points
- Data pipeline development: 25–70 points
- Ad-hoc analysis and insights: 10–30 points

**Review Criteria:**
- Are metrics accurately calculated and verified?
- Are dashboards performant with real data?
- Is data properly anonymized where needed?
- Are there documentation for metrics definitions?
- Can reports be regenerated and verified?

**Examples:**
- Building a dashboard showing Wave reward distribution
- Creating contributor activity reports across Waves
- Setting up on-chain analytics for WaveEscrow usage
- Building retention and churn analysis for contributors
- Creating cost-per-point analytics for ecosystem foundations
- Building A/B testing framework for UI changes
- Setting up anomaly detection for contract interactions
- Creating automated weekly ecosystem health reports

---

## 4. WaveEscrow Smart Contract Architecture

### 4.1 Core Contract Design

The WaveEscrow contract is built using the Arbitrum Stylus SDK, which allows smart contracts to be written in Rust and compiled to WASM for execution on Arbitrum. This provides several advantages over traditional Solidity development:

- **Memory safety**: Rust's ownership model prevents common vulnerabilities like buffer overflows and use-after-free.
- **Performance**: WASM execution is faster than EVM bytecode for compute-heavy operations.
- **Expressiveness**: Rust's type system and pattern matching enable cleaner, safer code.
- **Ecosystem**: Access to Rust's crate ecosystem for cryptographic operations, serialization, etc.

The contract follows a modular architecture:

```
WaveEscrow
├── src/lib.rs          # Entry point: sol_storage! + #[public] ABI
├── src/contract.rs     # Business logic implementation
├── src/errors.rs       # Error type definitions
└── src/events.rs       # Event definitions and emission helpers
```

### 4.2 Storage Layout

The contract's storage is defined using the `sol_storage!` macro, which generates efficient storage accessors:

```
WaveEscrow (entrypoint)
├── owner:                    address   — Contract owner (ecosystem foundation)
├── wave_oracle:              address   — Authorized oracle address
├── reward_token:             address   — USDC/USDT token address
├── paused:                   bool      — Emergency pause flag
├── wave_count:               uint256   — Total waves created
├── waves:                    mapping(uint256 => Wave)
│   └── Wave struct:
│       ├── total_budget:       uint256 — Reward pool for this wave
│       ├── total_points:       uint256 — Sum of all contributor points
│       ├── points_finalized:   bool    — Whether oracle has finalized
│       ├── end_timestamp:      uint256 — Sprint deadline
│       └── total_claimed:      uint256 — Total rewards claimed so far
└── has_claimed:              mapping(uint256 => mapping(address => bool))
```

Storage design decisions:
- **Wave struct is self-contained**: Each wave has all the data needed for distribution, enabling parallel independent waves.
- **Nested mapping for claims**: Efficient O(1) lookups for `has_claimed` checks.
- **Explicit tracking of total_claimed**: Enables emergency withdraw calculations and prevents double-spending.
- **Pause flag as first-class storage**: Emergency pause is a core safety mechanism, not an afterthought.

### 4.3 Function Specification

The contract exposes 22 external functions through the `#[public]` ABI:

**Wave Lifecycle Functions:**

| Function | Signature | Access | Description |
|----------|-----------|--------|-------------|
| `initialize_wave` | `(wave_id: U256, budget: U256, end_timestamp: U256) -> Result<(), Vec<u8>>` | Owner only | Creates a new wave reward pool |
| `finalize_wave_points` | `(wave_id: U256, total_points: U256) -> Result<(), Vec<u8>>` | Oracle only | Finalizes total points for a wave |
| `deposit_funds` | `(wave_id: U256, amount: U256) -> Result<(), Vec<u8>>` | Anyone | Adds funds to an existing wave |
| `claim_reward` | `(wave_id: U256, contributor_points: U256) -> Result<(), Vec<u8>>` | Anyone | Claims pro-rata reward for contributor |

**Administrative Functions:**

| Function | Signature | Access | Description |
|----------|-----------|--------|-------------|
| `transfer_ownership` | `(new_owner: Address) -> Result<(), Vec<u8>>` | Owner only | Transfers contract ownership |
| `update_oracle` | `(new_oracle: Address) -> Result<(), Vec<u8>>` | Owner only | Updates oracle address |
| `pause` | `() -> Result<(), Vec<u8>>` | Owner only | Pauses all state mutations |
| `unpause` | `() -> Result<(), Vec<u8>>` | Owner only | Resumes contract operations |
| `emergency_withdraw` | `(wave_id: U256, to: Address) -> Result<(), Vec<u8>>` | Owner only, paused | Recovers remaining funds |

**View Functions:**

| Function | Signature | Description |
|----------|-----------|-------------|
| `get_wave` | `(wave_id: U256) -> Result<(U256, U256, bool, U256, U256), Vec<u8>>` | Returns full wave state |
| `has_claimed` | `(wave_id: U256, contributor: Address) -> bool` | Checks if contributor claimed |
| `get_owner` | `() -> Address` | Returns current owner |
| `get_oracle` | `() -> Address` | Returns current oracle |
| `is_paused` | `() -> bool` | Returns pause status |
| `get_reward_token` | `() -> Address` | Returns reward token address |

### 4.4 Access Control Model

The contract implements a role-based access control model with two privileged roles and a pausability mechanism:

**Owner Role:**
- Designated at contract deployment.
- Can transfer ownership (prevents lockout).
- Responsible for initializing waves.
- Can update the oracle address.
- Can pause/unpause the contract.
- Can trigger emergency withdrawal (only when paused).
- Represented by the ecosystem foundation.

**Oracle Role:**
- Designated by the owner.
- The only role that can finalize wave points.
- Expected to be the Drips GitHub App backend.
- Can be updated by the owner at any time.
- Point finalization is a one-time action per wave (prevents manipulation).

**Unprivileged Users:**
- Can claim rewards for waves where they earned points.
- Can deposit funds into any wave.
- Can call any view function.
- All state-modifying functions have appropriate guards.

The access control is enforced through three helper functions:

```rust
fn only_owner(storage: &WaveEscrow) -> Result<(), Vec<u8>> {
    if msg::sender() != storage.owner.get() {
        return Err(Error::NotOwner.into());
    }
    Ok(())
}

fn only_oracle(storage: &WaveEscrow) -> Result<(), Vec<u8>> {
    if msg::sender() != storage.wave_oracle.get() {
        return Err(Error::NotOracle.into());
    }
    Ok(())
}

fn not_paused(storage: &WaveEscrow) -> Result<(), Vec<u8>> {
    if storage.paused.get() {
        return Err(Error::ContractPaused.into());
    }
    Ok(())
}
```

### 4.5 Event System

The contract emits Solidity-compatible events for all state changes, enabling off-chain indexing and monitoring:

| Event | Parameters | When Emitted |
|-------|------------|--------------|
| `WaveInitialized` | `wave_id` (indexed), `budget`, `end_timestamp`, `owner` (indexed) | New wave created |
| `PointsFinalized` | `wave_id` (indexed), `total_points`, `oracle` (indexed) | Points finalized |
| `RewardClaimed` | `wave_id` (indexed), `contributor` (indexed), `reward` | Reward claimed |
| `FundsDeposited` | `wave_id` (indexed), `depositor`, `amount` | Funds added to wave |
| `OwnershipTransferred` | `previous_owner`, `new_owner` | Ownership changed |
| `OracleUpdated` | `previous_oracle`, `new_oracle` | Oracle changed |
| `EmergencyWithdraw` | `wave_id` (indexed), `to`, `amount` | Emergency withdrawal |
| `ContractPaused` | `account` | Contract paused |
| `ContractUnpaused` | `account` | Contract unpaused |

Events are emitted before function returns (checks-effects-interactions pattern) to ensure they are always emitted even if an external call reenters.

### 4.6 Error Handling

The contract uses a custom error type system with descriptive variants:

```rust
pub enum Error {
    NotOwner,              // Caller is not the contract owner
    NotOracle,             // Caller is not the designated oracle
    WaveAlreadyFinalized,  // Attempt to finalize an already-finalized wave
    WaveNotFinalized,      // Attempt to claim before finalization
    WaveAlreadyExists,     // Attempt to create a duplicate wave
    WaveNotFound,          // Wave ID does not exist
    ZeroBudget,            // Budget cannot be zero
    ZeroPoints,            // Points cannot be zero
    ZeroAddress,           // Address cannot be zero
    AlreadyClaimed,        // Contributor already claimed for this wave
    ContractPaused,        // Contract is paused
    ZeroReward,            // Calculated reward is zero
    InsufficientBalance,   // Not enough tokens for withdrawal
    TokenTransferFailed,   // ERC20 transfer failed
    InvalidTimestamp,      // End timestamp must be in the future
    BudgetExceeded,        // Budget cannot exceed remaining pool
    Unauthorized,          // Generic unauthorized access attempt
}
```

Each error implements `Display` for readable output and `Into<Vec<u8>>` for returning as revert data. The Stylus runtime will include this data in the revert reason, making debugging easier for callers.

---

## 5. Drips GitHub App Integration

### 5.1 Verification Flow

The Drips GitHub App is the verification layer that bridges GitHub activity with on-chain reward distribution. The verification flow works as follows:

1. **Wave Registration**: The app registers a new Wave when the `WaveInitialized` event is detected. It creates an internal mapping of Wave ID to GitHub repository, issue list, and time window.

2. **Issue Tracking**: When a maintainer creates a scoped issue for a Wave, they tag it with the Wave ID (e.g., via a label like `wave:1`). The app tracks:
   - Issue assignment and unassignment events.
   - Pull requests linked to the issue.
   - Review status and merge events.
   - All activity timestamps (must be within the Wave window).

3. **Attestation Generation**: At the end of the Wave, the app:
   - Aggregates all completed issues per contributor.
   - Calculates total points per contributor based on issue point values.
   - Verifies timestamps for all activity.
   - Generates a cryptographic attestation (signed payload).
   - Submits the attestation to the WaveEscrow contract.

4. **Verification**: The contract verifies that:
   - The caller is the registered oracle.
   - The wave has not been finalized.
   - The wave exists.
   - Total points are non-zero.

### 5.2 Oracle Architecture

The oracle component can be implemented in multiple ways, each with different trust assumptions:

**Centralized Oracle (Phase 1):**
- A backend service operated by the ecosystem foundation.
- Signs attestations using a private key that maps to the `wave_oracle` address.
- Simpler to implement and operate.
- Trusted model — suitable for initial pilots.
- Single point of failure risk.

**Threshold Signature Oracle (Phase 2):**
- Multiple independent operators each hold a key share.
- M-of-N signatures are required to finalize points.
- Decentralized trust model.
- More complex infrastructure.
- Resilient to individual operator compromise.

**ZK-Proof Oracle (Phase 3):**
- Zero-knowledge proofs verify GitHub activity without revealing internal data.
- Proof is generated off-chain and verified on-chain.
- Maximum privacy and decentralization.
- Highest technical complexity.
- Long-term goal for the project.

The current implementation targets the centralized oracle model, with the architecture supporting migration to threshold signatures via the `update_oracle` function.

### 5.3 Point Calculation Engine

The point calculation engine is the core of the Drips GitHub App. It translates GitHub activity into point values:

**Inputs:**
- GitHub issue data (labels, assignees, timestamps).
- Pull request data (linked issues, reviews, merge status).
- Repository metadata (labels configuration, point values).

**Calculation Process:**

```
For each issue labeled with the Wave ID:
  - If the issue has a `points:N` label, use N as the base point value.
  - If no explicit points label, use the issue type label to determine base value.
  - Apply quality multipliers:
    - Code review: 0.1x multiplier per reviewer approval.
    - Early completion: 1.2x if completed in first half of sprint.
    - High-quality PR (clean CI, full test coverage): 1.1x.
  - Apply bonus points:
    - First-time contributor bonus: +10 points.
    - Critical fix bonus: +25 points.
  - Sum all points for the contributor across all their completed issues.

Total Points = SUM(issue_points * quality_multiplier + bonus for each completed issue)
```

**Issues Not Accepted:**
- Issues completed outside the Wave time window.
- Pull requests that are not merged or approved.
- Issues not properly labeled with the Wave ID.
- Work not meeting the minimum quality threshold.

### 5.4 Webhook System

The Drips GitHub App uses webhooks to receive real-time notifications about repository activity:

**Subscribed Events:**
- `issues` — Issue creation, assignment, labeling, closing.
- `pull_request` — PR creation, review requests, reviews, merging.
- `pull_request_review` — Review submissions and approval status.
- `label` — Label creation and modification (for points configuration).

**Webhook Processing Pipeline:**

```
GitHub Event → Webhook Receiver → Event Validator → Event Processor → State Store
                                                                         ↓
                                                                   Attestation Engine
                                                                         ↓
                                                                   Signer Service
                                                                         ↓
                                                                   Contract TX
```

Each event is validated for authenticity (HMAC signature verification), deduplicated (idempotency keys), and processed through a state machine that tracks issue progress through the Wave lifecycle.

---

## 6. Sprint Cycle Implementation

### 6.1 Wave Planning Phase

During the planning phase, the ecosystem foundation and maintainers prepare the Wave for execution:

**Day 1: Budget & Scope Definition**
- Foundation determines the total reward pool (e.g., $50,000 USDC).
- Foundation selects maintainers for the Wave.
- Maintainers review project roadmap and prioritize work.
- High-level scope is defined and communicated to the community.

**Day 2: Issue Scoping**
- Maintainers create detailed issues in the project repository.
- Each issue includes:
  - Clear description and motivation.
  - Technical approach and constraints.
  - Acceptance criteria (must be testable).
  - Point value based on difficulty tier.
  - Wave label (e.g., `wave:1`).
- Issues are reviewed by the foundation for alignment with goals.

**Day 3: Wave Initialization**
- Foundation calls `initialize_wave(wave_id, budget, end_timestamp)`.
- Foundation deposits the reward pool via token approval + deposit transaction.
- The Drips GitHub App detects the `WaveInitialized` event.
- Issues are tagged and made available for picking up.
- Wave is announced to the community with links to available issues.

**Planning Checklist:**
- [ ] Reward pool funded and deposited.
- [ ] All issues created and properly scoped.
- [ ] Point values assigned to each issue.
- [ ] Oracle address configured.
- [ ] Drips GitHub App installed on repository.
- [ ] Wave announced with clear timeline.

### 6.2 Sprint Execution Phase

During execution, contributors pick up issues and complete work:

**Contributor Workflow:**
1. Browse available issues tagged with the active Wave.
2. Comment on the issue to signal intent.
3. A maintainer assigns the issue to the contributor.
4. Contributor implements the solution in a fork or branch.
5. Contributor submits a pull request linked to the issue.
6. Maintainer(s) review the PR and provide feedback.
7. Contributor addresses feedback and iterates.
8. PR is approved and merged by a maintainer.
9. Issue is closed.

**Maintainer Responsibilities:**
- Monitor issue assignments and ensure fair distribution.
- Review PRs within 48 hours (target SLA).
- Provide constructive, actionable feedback.
- Ensure all acceptance criteria are met before merging.
- Track contributor progress and address blockers.

**Progress Tracking:**
- Open issues: Not yet assigned or picked up.
- In progress: Assigned, work in development.
- Under review: PR submitted, awaiting review.
- Completed: PR merged, issue closed.
- Blocked: External dependency or clarification needed.

### 6.3 Review & Finalization Phase

At the end of the sprint window, the review phase ensures all work is properly evaluated:

**Day 25: Sprint Closure**
- No new issues can be picked up for this Wave.
- In-progress work can still be completed and submitted.
- Maintainers conduct final pass on all pending PRs.
- A 48-hour grace period allows completion of work-in-progress.

**Day 26–27: Final Review**
- All remaining PRs are reviewed and either merged or deferred.
- Deferred issues may be rolled into the next Wave.
- Each contributor's completed issues are tallied.
- Point values are verified against the issue labels.
- Quality multipliers and bonuses are calculated.

**Day 28: Finalization**
- The Drips GitHub App oracle computes the final point distribution.
- A signed attestation is generated containing:
  - Wave ID.
  - Total points for the Wave.
  - Per-contributor points.
  - Proof data (GitHub event hashes, timestamps).
- The oracle calls `finalize_wave_points(wave_id, total_points)`.
- The contract emits `PointsFinalized`.
- Contributors are notified that rewards are ready to claim.

### 6.4 Reward Distribution Phase

The distribution phase completes the Wave lifecycle with payouts:

**Day 29–30: Claim Window**
- Contributors call `claim_reward(wave_id, contributor_points)`.
- The contract verifies:
  - Wave is finalized.
  - Caller has not already claimed.
  - Contributor points are consistent with the finalized distribution.
- Reward is calculated and transferred.
- Contributors receive USDC/USDT in their wallet.
- Each claim emits a `RewardClaimed` event for transparency.

**Claim UX:**
- Contributors visit the Drips web interface.
- They connect their wallet (via MetaMask, WalletConnect, etc.).
- The interface shows their earned points and calculated reward.
- They click "Claim" to initiate the transaction.
- Gas fees are paid by the contributor or optionally subsidized.

**Post-Claim:**
- Unclaimed rewards after 30 days may be reclaimed by the foundation.
- The foundation can initiate a new Wave with remaining funds.
- All claim data is indexed for analytics and reporting.
- Contributors build a track record across multiple Waves.

**Distribution Checklist:**
- [ ] All contributors notified of claim availability.
- [ ] Claim interface is operational.
- [ ] Gas estimates displayed before transaction.
- [ ] Support channel available for claim issues.
- [ ] Wave completion report generated.

---

## 7. Point System Design

### 7.1 Point Weighting by Work Type

Different types of work contribute differently to the project's goals. The point system reflects these differences:

| Work Type | Base Points | Typical Range | Rationale |
|-----------|-------------|---------------|-----------|
| Bug Fix (Critical) | 100 | 50–200 | Directly protects users and protocol |
| Bug Fix (High) | 40 | 20–80 | Important stability improvement |
| Bug Fix (Medium) | 20 | 10–40 | Moderate improvement |
| Bug Fix (Low) | 10 | 5–15 | Minor fix |
| New Feature (Large) | 200 | 100–500 | Significant new capability |
| New Feature (Medium) | 60 | 40–100 | Modest new feature |
| New Feature (Small) | 25 | 15–40 | Small enhancement |
| Documentation | 30 | 5–200 | Varies widely by scope |
| Testing | 40 | 5–120 | Quality assurance |
| Refactoring | 50 | 5–300 | Technical debt reduction |
| DevOps | 40 | 10–200 | Infrastructure improvements |
| Security | 100 | 20–400 | Critical safety work |
| Community | 30 | 5–150 | Ecosystem growth |
| Research | 50 | 10–200 | Exploration and prototyping |
| Translation | 30 | 5–100 | Localization |
| Design/UX | 40 | 5–250 | User experience |

### 7.2 Difficulty Tiers

Each issue is assigned a difficulty tier that affects the base point value:

| Tier | Label | Description | Point Multiplier |
|------|-------|-------------|------------------|
| S1 | `difficulty:trivial` | Simple change, single file, minimal context needed | 0.5x |
| S2 | `difficulty:easy` | Straightforward, limited scope, some context needed | 1.0x |
| S3 | `difficulty:medium` | Moderate complexity, multiple files, good context needed | 2.0x |
| S4 | `difficulty:hard` | Complex, deep understanding of system required | 4.0x |
| S5 | `difficulty:expert` | Requires specialized knowledge, cross-cutting changes | 8.0x |

**Examples by Tier:**
- **Trivial**: Fix a typo in documentation, update a dependency version.
- **Easy**: Add input validation to an existing function, write tests for a module.
- **Medium**: Implement a new API endpoint, create a dashboard widget.
- **Hard**: Design and implement a new subsystem, migrate a major component.
- **Expert**: Implement a cryptographic protocol, redesign the storage layer for performance.

### 7.3 Quality Multipliers

Quality multipliers reward contributors who go above and beyond the minimum requirements:

| Multiplier | Condition | Effect |
|------------|-----------|--------|
| Early Bird | PR merged in first 50% of sprint | 1.2x |
| Code Review Quality | PR receives 2+ approvals with substantive comments | 1.1x |
| Test Excellence | 95%+ test coverage on new code | 1.1x |
| Documentation | Comprehensive docs included with PR | 1.1x |
| Performance | PR includes benchmarks showing improvement | 1.05x |
| Mentorship | Contributor reviews other PRs | 1.05x per review (max 3) |

Quality multipliers cannot exceed 2.0x total per issue to prevent excessive point inflation.

### 7.4 Bonus Points

Bonus points are awarded for specific achievements that benefit the ecosystem:

| Bonus | Amount | Criteria |
|-------|--------|----------|
| First Contribution | +15 points | Contributor's first accepted issue |
| Critical Bug Fix | +50 points | Fix for a security vulnerability or critical production issue |
| Community Choice | +25 points | Voted by community as most impactful contribution |
| Zero Bug-Back | +10 points | PR merged with zero review iterations |
| Cross-Functional | +20 points | PR includes code + tests + docs |
| Pair Programming | +10 points each | Two contributors collaborate on a single issue |

Bonuses are additive and stack with quality multipliers. All bonuses are applied at the discretion of the maintainer and must be justified in the issue comments.

---

## 8. Token Economics

### 8.1 Reward Pool Sizing

The size of the reward pool for each Wave should be calibrated to attract quality contributors while being sustainable for the ecosystem foundation:

**Sizing Factors:**
- **Project stage**: Early projects may need larger pools to attract contributors.
- **Contributor market rates**: Competitive with typical bounties and grants.
- **Issue complexity**: More complex work requires higher compensation.
- **Frequency**: More frequent Waves can have smaller individual pools.
- **Protocol revenue**: For protocols with revenue, a percentage can fund Waves.

**Recommended Pool Sizes:**
- Pilot Waves: $5,000–$20,000
- Standard Waves: $20,000–$100,000
- Major Waves: $100,000–$500,000
- Ecosystem Waves: $500,000+

### 8.2 USDC/USDT Integration

The contract holds and distributes stablecoins (USDC or USDT) as rewards. Integration details:

**Token Approval:**
- The reward pool must be funded via the standard ERC20 `approve` + `transferFrom` pattern.
- The contract must be approved to spend the reward tokens by the foundation.
- Only one reward token is supported per contract deployment.

**Transfer Mechanism:**
- Reward transfers use the standard ERC20 `transfer` function.
- The contract verifies the return value (for USDC/USDT which return `bool`).
- Non-reverting tokens (like USDT) are handled via safe wrapper.
- Transfers that return `false` are reverted with `TokenTransferFailed` error.

**Balance Tracking:**
- The contract tracks `total_claimed` per wave to know the remaining balance.
- The actual contract balance at any time should equal SUM(all wave budgets) - SUM(all claimed).
- Discrepancies can be detected through monitoring.

### 8.3 Pro-Rata Distribution Math

The core distribution formula is:

```
reward = (budget * contributor_points) / total_points
```

This is implemented in the contract as:

```rust
let total_points = wave.total_points.get();
let budget = wave.total_budget.get();
let reward = budget * contributor_points / total_points;
```

**Mathematical Properties:**
- **Zero-sum**: The sum of all rewards equals the total budget (with possible dust remainder due to integer division).
- **Monotonic**: More points always result in a larger or equal reward.
- **Deterministic**: Given the same inputs, the output is always the same.
- **No rounding issues**: Solidity/U256 integer division truncates toward zero, which is acceptable for this use case.

**Dust Handling:**
- Integer division may leave a small remainder (dust).
- Dust accumulates in the contract and can be:
  - Donated to the next Wave.
  - Withdrawn by the owner after all claims are processed.
  - Left in the contract as a buffer.

### 8.4 Fee Model

The WaveEscrow contract itself does not charge fees. However, there are associated costs:

**Gas Costs:**
- `initialize_wave`: One-time, relatively inexpensive.
- `finalize_wave_points`: One-time per wave.
- `claim_reward`: Per-claim, gas cost borne by claimant.
- For a wave with 100 contributors, total gas is approximately:
  - 1 × initialize = ~50,000 gas
  - 1 × finalize = ~40,000 gas
  - 100 × claim = ~5,000,000 gas (at ~$0.01 on Arbitrum)
  - Total: ~$50–100 on Arbitrum

**Operational Costs:**
- Drips GitHub App hosting (can run on a small server or serverless).
- Oracle signing key management.
- Monitoring and alerting infrastructure.
- Estimated at $100–500/month for a modest setup.

---

## 9. Security Architecture

### 9.1 Contract Security

The WaveEscrow contract incorporates multiple security patterns:

**Checks-Effects-Interactions Pattern:**
All state-modifying functions follow this pattern:
1. **Checks**: Validate all inputs, access control, and preconditions.
2. **Effects**: Update internal state (mark as claimed, update totals).
3. **Interactions**: Make external calls (token transfers).

This prevents reentrancy by ensuring all state changes happen before any external calls.

**Integer Overflow Protection:**
- All arithmetic uses U256 from alloy-primitives, which has built-in overflow protection.
- Division by zero is prevented by checks for zero total points and zero contributor points.
- Subtraction underflow is prevented by balance checks (`reward > remaining`).

**Access Control:**
- Owner and oracle roles are enforced at the function level.
- Ownership transfer requires explicit action (no renounce).
- Oracle can be updated by owner at any time (key rotation).
- Emergency pause can halt all state-modifying functions.

**What the Contract Does NOT Do:**
- It does not mint tokens.
- It does not interact with other contracts beyond the reward token.
- It does not have upgrade mechanisms (handled at the proxy level).
- It does not store user data beyond claim status.

### 9.2 Oracle Security

The oracle is a critical component that must be secured:

**Key Management:**
- The oracle private key should be stored in a Hardware Security Module (HSM) or secure key management service.
- Access to the signing key should require M-of-N approvals.
- Key should be rotated periodically.
- The `wave_oracle` address on the contract must be updated after key rotation.

**Attestation Integrity:**
- Each attestation should include:
  - Wave ID (prevents replay across waves).
  - Block number or timestamp (prevents replay across chains).
  - Total points as a checksum of individual points.
- The signed payload should be logged and verifiable off-chain.

**Failover:**
- Multiple oracle instances can run in active-passive configuration.
- If the primary oracle fails, the backup can be promoted.
- The owner can update the oracle address if all instances fail.
- In extreme cases, the owner can pause and emergency withdraw.

### 9.3 Upgrade Path

The contract uses the Stylus upgradeability pattern:

**Proxy Pattern:**
- A proxy contract delegates calls to the implementation contract.
- The proxy stores the implementation address in storage.
- The owner can upgrade by pointing the proxy to a new implementation.
- Storage layout must be preserved across upgrades.

**Upgrade Criteria:**
- Critical security fix: Immediate upgrade.
- New feature: Deployed via new implementation + proxy update.
- Parameter change: Via governance vote.
- Deprecation: Contract frozen via pause.

**Storage Compatibility:**
- New implementations must append new storage fields only.
- Existing storage fields cannot be removed or reordered.
- Storage gaps should be left for future fields.
- Storage layout tests verify compatibility.

### 9.4 Emergency Procedures

**Scenario 1: Contract Bug Discovered**
1. Owner calls `pause()` to halt all operations.
2. Points are finalized off-chain for completed work.
3. Owner calls `emergency_withdraw()` to recover funds.
4. Funds are redistributed manually or via a new contract.
5. Bug is fixed and new contract is deployed.

**Scenario 2: Oracle Compromised**
1. Owner calls `update_oracle()` with a new address.
2. Owner calls `pause()` as a precaution.
3. Compromised key is revoked.
4. Operations resume with new oracle.
5. Incident is investigated and reported.

**Scenario 3: Reward Token Issue**
1. Owner calls `pause()` to prevent claims.
2. Funds remain locked until token issue is resolved.
3. If token must be swapped, emergency withdraw and redeploy.
4. Contributors are manually compensated if needed.

**Scenario 4: Unexpected Upgrade**
1. Owner calls `pause()` immediately.
2. All funds are secure in the contract.
3. Investigation determines the nature of the upgrade.
4. Appropriate action is taken (accept, revert, or countermeasure).

---

## 10. Testing Strategy

### 10.1 Unit Testing

Unit tests verify individual function behavior in isolation:

**Test Categories:**
- Initialization: Wave creation, duplicate prevention, input validation.
- Finalization: Oracle-only access, double-finalize prevention.
- Claiming: Successful claim, double-claim prevention, pre-finalization rejection.
- Access Control: Owner-only functions, oracle-only functions.
- Pausability: Pause/unpause, blocked actions while paused.
- Emergency: Withdraw logic, state verification.

**Test Coverage Targets:**
- 100% of error paths covered.
- 100% of access control paths covered.
- 100% of public function happy paths covered.
- 90%+ overall code coverage.

**Test File Structure:**
```rust
// tests/integration_test.rs
mod initialization;
mod finalization;
mod claiming;
mod access_control;
mod pausability;
mod emergency;
mod edge_cases;
```

### 10.2 Integration Testing

Integration tests verify the contract works correctly with other components:

**Scenarios:**
- Full Wave lifecycle: Create → Fund → Work → Finalize → Claim.
- Multiple contributors claiming in parallel (simulated).
- Wave point edge cases (zero, very large, uneven distribution).
- Token transfer edge cases (not enough balance, non-standard ERC20).

**Environment:**
- Local testnet (Arbitrum Nitro devnet).
- Forked Arbitrum mainnet for realistic conditions.
- Stylus VM test environment for accurate WASM execution.

### 10.3 Fuzz Testing

Fuzz testing discovers unexpected behavior with random inputs:

**Fuzz Targets:**
- Point calculation: Verify no overflow for extreme values.
- Reward distribution: Verify sum of rewards <= budget for all inputs.
- Timestamp handling: Verify edge cases around sprint boundaries.
- Address handling: Verify zero address, invalid addresses, contract addresses.

**Tools:**
- `cargo-fuzz` for Rust-level fuzzing.
- Echidna or Foundry for EVM-level fuzzing (via Stylus interop).
- Property-based testing with `proptest` crate.

**Properties to Test:**
```rust
// For any valid distribution:
// 1. Sum of all rewards should not exceed budget
// 2. No single reward should exceed budget
// 3. Rewards should be monotonically increasing with points
// 4. Equal points should produce equal rewards
```

### 10.4 Formal Verification

For critical safety properties, formal verification provides mathematical guarantees:

**Properties to Verify:**
- Total rewards claimed never exceed total budget for any wave.
- Only finalize_wave_points can set points_finalized = true.
- has_claimed can only be set once per (wave_id, contributor) pair.
- emergency_withdraw can only reduce budget, never increase it.
- Owner transfer cannot set owner to zero address.

**Approach:**
- Use SMT solvers (Z3) via symbolic execution.
- Model contract state transitions as a state machine.
- Verify invariants hold for all reachable states.
- Rust's strong type system reduces the need for some verification.

### 10.5 Testnet Deployment

Real-world testing on testnets before mainnet deployment:

**Stages:**
1. Local Nitro devnet (instant, fully controlled).
2. Arbitrum Sepolia testnet (public, realistic conditions).
3. Arbitrum One mainnet (production).

**Testnet Verification:**
- Deploy contract via `cargo stylus deploy`.
- Verify ABI matches expectations.
- Execute full Wave lifecycle with test USDC.
- Confirm event emissions match specifications.
- Run claim flow with multiple test accounts.
- Verify gas costs are within acceptable ranges.

---

## 11. Deployment Plan

### 11.1 Environment Configuration

**Environment Variables:**
```
RPC_URL=https://sepolia-rollup.arbitrum.io/rpc
PRIVATE_KEY=0x...
OWNER_ADDRESS=0x...
ORACLE_ADDRESS=0x...
REWARD_TOKEN=0x...  (USDC on Arbitrum: 0xaf88d065e77c8c...)
```

**Configuration File (style.toml):**
```toml
[deploy]
endpoint = "https://sepolia-rollup.arbitrum.io/rpc"
private-key = "..."
```

### 11.2 Deployment Pipeline

**Manual Deployment Steps:**

```bash
# 1. Build the WASM binary
cargo stylus build

# 2. Estimate gas
cargo stylus estimate \
  --endpoint $RPC_URL \
  --private-key $PRIVATE_KEY

# 3. Deploy
cargo stylus deploy \
  --endpoint $RPC_URL \
  --private-key $PRIVATE_KEY

# 4. Verify deployment
#    - Check contract address
#    - Verify code hash on explorer
#    - Test with a small transaction

# 5. Initialize contract state
#    - Set owner
#    - Set oracle address
#    - Set reward token address
```

**Automated CI/CD Deployment:**
```yaml
# .github/workflows/deploy.yml
deploy:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - name: Build & Deploy
      run: |
        cargo stylus build
        cargo stylus deploy \
          --endpoint ${{ secrets.RPC_URL }} \
          --private-key ${{ secrets.DEPLOYER_KEY }}
```

### 11.3 Post-Deployment Verification

**Verification Checklist:**
- [ ] Contract code hash matches build output.
- [ ] Owner address is correctly set.
- [ ] Oracle address matches Drips backend key.
- [ ] Reward token address is correct (USDC/USDT).
- [ ] Contract is not paused.
- [ ] `get_owner()` returns expected address.
- [ ] `get_oracle()` returns expected address.
- [ ] `get_reward_token()` returns expected token address.
- [ ] `get_wave(0)` returns error (no waves yet).
- [ ] Can initialize and retrieve wave info.
- [ ] Non-owner cannot call owner functions.
- [ ] Pause and unpause work as expected.

---

## 12. Monitoring & Operations

### 12.1 On-Chain Monitoring

**Metrics to Track:**
- Total value locked (TVL) across all waves.
- Number of active waves.
- Number of contributors per wave.
- Claim rate per wave (% of total rewards claimed).
- Gas costs per operation.
- Time between finalization and first claim.

**Monitoring Tools:**
- Dune Analytics dashboards for on-chain data.
- The Graph subgraph for indexed event data.
- Custom Grafana dashboards for real-time metrics.
- Block explorer (Arbiscan) for manual verification.

**Alerting Triggers:**
- Unexpected large token transfers from the contract.
- Multiple failed claim attempts from the same address.
- Unusually high gas consumption.
- Contract pause event.
- Ownership transfer event.

### 12.2 Off-Chain Services

**Drips GitHub App Monitoring:**
- Webhook delivery success rate.
- Attestation generation time.
- Oracle signing key status.
- Database health and replication lag.
- API response times and error rates.

**Backend Infrastructure:**
- Server CPU, memory, disk usage.
- Database connection pool utilization.
- Cache hit rates.
- Queue depths for async processing.
- SSL certificate expiry.

### 12.3 Alerting

**Alert Channels:**
- PagerDuty/OpsGenie for critical alerts.
- Slack/Telegram for warning-level alerts.
- Email for informational notifications.
- On-chain events via webhook to monitoring system.

**Alert Severity Levels:**

| Severity | Response Time | Examples |
|----------|---------------|----------|
| Critical | 15 minutes | Contract paused, oracle key compromised |
| High | 1 hour | Oracle service down, high error rate |
| Medium | 4 hours | Claim rate drop, slow attestation generation |
| Low | 24 hours | Disk space warning, stale backups |

---

## 13. Governance & Upgrade Path

### 13.1 DAO Integration

Long-term, the WaveEscrow contract can be governed by a DAO:

**Transition Path:**
1. Initially, the ecosystem foundation holds the owner role.
2. A time-lock multisig (e.g., Gnosis Safe) is set as the owner.
3. A governance token vote can control the multisig.
4. Eventually, a DAO smart contract directly owns the WaveEscrow.

**DAO Responsibilities:**
- Approving budget for each Wave.
- Appointing and removing maintainers.
- Adjusting point weights and difficulty tiers.
- Upgrading the contract (via proxy).
- Emergency actions (pause, emergency withdraw).

### 13.2 Parameter Adjustments

Parameters that can be adjusted without contract upgrade:

| Parameter | Method | Current | Adjustable By |
|-----------|--------|---------|---------------|
| Oracle address | `update_oracle()` | Set at deploy | Owner/DAO |
| Contract owner | `transfer_ownership()` | Set at deploy | Owner |
| Reward token | Requires upgrade | Set at deploy | Governance |
| Pause status | `pause()` / `unpause()` | Unpaused | Owner/DAO |
| Wave budget | Per-wave parameter | Per wave | Owner |

### 13.3 Contract Upgrades

Major upgrades follow a formal process:

1. **Proposal**: Describe the upgrade, motivation, and technical approach.
2. **Review**: Community review period (minimum 7 days).
3. **Audit**: New implementation is audited (if logic changes).
4. **Vote**: Governance vote (if DAO-controlled).
5. **Deploy**: New implementation is deployed as a separate WASM binary.
6. **Upgrade**: Proxy is updated to point to the new implementation.
7. **Verify**: Post-upgrade verification of storage and behavior.

---

## 14. Risk Assessment

### 14.1 Smart Contract Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Integer overflow/underflow | Low | Critical | Safe math via U256 type; input validation |
| Reentrancy | Low | Critical | Checks-effects-interactions pattern |
| Access control bypass | Low | Critical | Role-based guards; thorough testing |
| Logic error in reward calculation | Low | High | Fuzz testing; formal verification |
| Storage collision (upgrade) | Low | Medium | Storage layout tests; gap fields |
| Denial of service (gas griefing) | Medium | Medium | View functions for claims; batch processing |

### 14.2 Oracle Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Oracle key compromise | Low | Critical | HSM; M-of-N signing; key rotation |
| Oracle unavailability | Medium | High | Failover; manual finalization path |
| Incorrect point calculation | Low | High | Independent verification; audit trail |
| Oracle manipulation | Low | Critical | Timestamp verification; on-chain checks |
| Replay attacks | Low | Medium | Nonce via wave_id + block number |

### 14.3 Economic Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Insufficient reward pool | Low | High | Pre-funding check; deposit before launch |
| Token depeg (USDC/USDT) | Low | Medium | Diversify across tokens; short Waves |
| Gas price spikes | Medium | Low | Arbitrum low fees; subsidy consideration |
| Contributor fraud | Low | Medium | Maintainer review; quality checks |
| Sybil attacks | Low | Medium | Reputation system; identity verification |

### 14.4 Mitigation Strategies

**Technical Mitigations:**
- Multiple independent oracles for cross-verification.
- Time-locks on critical operations.
- Circuit breakers for anomalous activity.
- Gradual release of large reward pools.
- Transparent, verifiable point calculation algorithms.

**Process Mitigations:**
- All point calculations are reviewable on GitHub.
- Clear dispute resolution process.
- Public dashboards for contract activity.
- Regular security audits (quarterly minimum).
- Bug bounty program for vulnerability discovery.

---

## 15. Roadmap

### 15.1 Phase 1: Foundation (Weeks 1–4)

**Goals:**
- Deploy functional WaveEscrow contract on testnet.
- Prove the Wave lifecycle end-to-end.
- Establish contributor onboarding flow.

**Deliverables:**
- [x] WaveEscrow smart contract implementation.
- [x] Comprehensive test suite (20+ tests).
- [x] CI/CD pipeline.
- [ ] Deploy to Arbitrum Sepolia.
- [ ] Drips GitHub App prototype (basic webhook handling).
- [ ] Internal pilot Wave with 5–10 contributors.
- [ ] Documentation and contributor guides.

**Metrics:**
- Contract passes all tests.
- Testnet deployment verified.
- At least one complete Wave cycle executed.
- Zero security incidents.

### 15.2 Phase 2: Pilot (Weeks 5–8)

**Goals:**
- Launch on Arbitrum One mainnet.
- Run 2–3 Waves with real contributors.
- Gather feedback and iterate.

**Deliverables:**
- [ ] Mainnet deployment.
- [ ] Production Drips GitHub App.
- [ ] 2–3 completed Waves.
- [ ] Reward distribution tracking dashboard.
- [ ] Contributor feedback survey.
- [ ] Security audit of contract and oracle.
- [ ] Bug bounty program launch.

**Metrics:**
- 25+ active contributors.
- >90% claim rate.
- <1% dispute rate on point calculations.
- Average contributor satisfaction >4/5.

### 15.3 Phase 3: Production (Weeks 9–16)

**Goals:**
- Scale to multiple parallel Waves.
- Optimize gas costs and UI.
- Establish governance framework.

**Deliverables:**
- [ ] Multi-Wave support (parallel execution).
- [ ] Optimistic attestation (reduced oracle overhead).
- [ ] Gas optimization pass on contract.
- [ ] DAO integration for governance.
- [ ] Time-lock multisig for owner role.
- [ ] Advanced analytics dashboard.
- [ ] Contributor reputation system.
- [ ] Mobile-friendly claim interface.

**Metrics:**
- 100+ active contributors per month.
- <$0.01 average claim cost.
- <48 hour average finalization time.
- >95% contributor retention rate.

### 15.4 Phase 4: Ecosystem (Weeks 17–32)

**Goals:**
- Decentralize oracle.
- Expand to multiple chains.
- Build ecosystem around WaveEscrow.

**Deliverables:**
- [ ] Threshold signature oracle (M-of-N).
- [ ] ZK-proof oracle prototype.
- [ ] Cross-chain Wave support (Optimism, Base, zkSync).
- [ ] WaveEscrow factory for permissionless deployment.
- [ ] Open-source Drips GitHub App.
- [ ] Ecosystem grants for tooling and integrations.
- [ ] Third-party oracle integrations (Chainlink, Pyth).
- [ ] Contributor DAO formation.

**Metrics:**
- 500+ active contributors across ecosystems.
- $5M+ total rewards distributed.
- <2 hour average finalization time.
- 10+ ecosystem foundations using WaveEscrow.

---

## 16. Appendices

### 16.1 Glossary

| Term | Definition |
|------|------------|
| Wave | A time-bound sprint cycle with a fixed reward pool and scoped issues |
| Sprint | The execution period of a Wave during which contributors complete work |
| Point | A unit of contribution measurement used to determine reward share |
| Reward Pool | The total USDC/USDT amount allocated for distribution in a Wave |
| Pro-Rata | Proportional distribution based on each contributor's point share |
| Oracle | The entity (Drips GitHub App) that verifies contributions and finalizes points |
| Attestation | A signed payload containing finalized point distribution data |
| Maintainer | A project member who scopes issues and reviews contributions |
| Contributor | A participant who completes issues and earns points |
| Scoped Issue | A clearly defined task with acceptance criteria and point value |
| WaveEscrow | The smart contract that holds reward pools and distributes rewards |
| Stylus | Arbitrum's Rust-to-WASM smart contract framework |
| USDC/USDT | Stablecoins used for reward payouts |

### 16.2 Reference Implementations

**Arbitrum Stylus Documentation:**
- https://docs.arbitrum.io/stylus/stylus-gentle-introduction
- https://github.com/OffchainLabs/stylus-sdk-rs

**Related Projects:**
- Gitcoin Grants: Quadratic funding for public goods.
- Radicle: Peer-to-peer code collaboration.
- SourceCred: Cred-based contribution tracking.
- Govrn: On-chain contribution attestations.

**Smart Contract Patterns:**
- OpenZeppelin Contracts: Industry-standard access control and security patterns.
- solady: Gas-optimized Solidity libraries.
- Arbitrum Stylus SDK Examples: Reference implementations for Stylus patterns.

### 16.3 Useful Commands

**Building and Testing:**

```bash
# Build the WASM binary
cargo stylus build

# Run unit tests (host target)
cargo test

# Run specific test
cargo test test_initialize_wave

# Check WASM binary size
ls -la target/wasm32-unknown-unknown/release/wave-escrow.wasm

# Generate ABI
cargo stylus export-abi
```

**Deployment:**

```bash
# Deploy to Arbitrum Sepolia
cargo stylus deploy \
  --endpoint https://sepolia-rollup.arbitrum.io/rpc \
  --private-key 0x...

# Activate the contract
cargo stylus activate \
  --endpoint https://sepolia-rollup.arbitrum.io/rpc \
  --private-key 0x... \
  --deployer-address 0x...
```

**Monitoring:**

```bash
# Check contract balance
cast balance 0x... --erc20 0xaf88d065e77c8c... 

# Query wave info
cast call 0x... "get_wave(uint256)(uint256,uint256,bool,uint256,uint256)" 0x1

# Query contract pause status
cast call 0x... "is_paused()(bool)"

# Listen for events
cast logs --address 0x... --from-block latest
```

---

*This plan was generated for the WaveEscrow project by WaveEscrow.

This document is version-controlled alongside the contract code in the repository.
