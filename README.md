# CTO Parallel Task Execution Test Project

This project is designed to test and validate the parallel task execution system in the CTO platform.

## Purpose

- **Test parallel task orchestration** with realistic dependency scenarios
- **Validate execution level generation** from TaskMaster dependencies  
- **Test integration PR coordination** across parallel tasks
- **Measure actual speedup** vs theoretical calculations
- **Validate conflict detection** when tasks modify overlapping files

## Test Scenario: Simple Rust API

We simulate building a basic Rust API with these components:

### Execution Levels (Expected)

**Level 0 (Parallel):**
- Task 1: Database Schema Setup
- Task 3: User Authentication Module  
- Task 4: Product Catalog Module
- Task 6: Frontend Components

**Level 1 (Sequential):**
- Task 2: API Endpoints (depends on Task 1)
- Task 5: Shopping Cart API (depends on Tasks 3,4)

**Level 2 (Final):**
- Task 7: Integration Tests (depends on Tasks 2,5,6)

### Expected Results

- **Theoretical speedup**: 7 tasks / 3 levels = 2.33x
- **Max parallel tasks**: 4 (in Level 0)
- **Integration PRs**: 3 (one per level)

## Test Validation

The test will verify:

1. ✅ **Dependency parsing** - TaskMaster dependencies correctly parsed
2. ✅ **Level generation** - Correct execution levels created  
3. ✅ **Parallel execution** - Multiple tasks run simultaneously in each level
4. ✅ **Integration PRs** - PRs created per level with proper merging
5. ✅ **Conflict detection** - File overlaps detected and reported
6. ✅ **Performance metrics** - Speedup calculations and telemetry

## Usage

Deploy via CTO platform with parallel execution enabled:

```yaml
parameters:
  - name: parallel-execution
    value: "true"
  - name: repository  
    value: "5dlabs/cto-parallel-test"
```

Monitor execution and validate the parallel orchestration works as expected.
