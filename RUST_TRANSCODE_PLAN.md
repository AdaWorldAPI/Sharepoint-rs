# RUST_TRANSCODE_PLAN.md

Full transcode plan from the original C# Sharepoint repo.

## Phase 0 - Foundation
- ST-06: smb-qos crate (QScore + lane scheduling)

## Phase 1 - Direct transcodes
- ST-17: QoS/QScoreCalculator.cs → smb-qos/src/qscore.rs (COMPLETED)

## Next
- ST-06 Lane Scheduler
- ST-10 DeltaTokenManager, etc.