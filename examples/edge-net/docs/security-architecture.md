# Edge-Net Relay Security Architecture

## System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                     Edge-Net Security Layers                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Layer 1: Connection Security                                   │
│  ┌────────────────────────────────────────────────────────┐    │
│  │ • Origin Validation (CORS)                              │    │
│  │ • Connection Limits (5 per IP)                          │    │
│  │ • Heartbeat Timeout (30s)                               │    │
│  └────────────────────────────────────────────────────────┘    │
│                          ↓                                       │
│  Layer 2: Message Security                                      │
│  ┌────────────────────────────────────────────────────────┐    │
│  │ • Rate Limiting (100 msg/min per node)                  │    │
│  │ • Message Size Limits (64KB max)                        │    │
│  │ • Message Type Validation                               │    │
│  └────────────────────────────────────────────────────────┘    │
│                          ↓                                       │
│  Layer 3: Identity Security (⚠️ NEEDS IMPROVEMENT)              │
│  ┌────────────────────────────────────────────────────────┐    │
│  │ • Public Key Registration                               │    │
│  │ • ❌ Signature Verification (NOT IMPLEMENTED)           │    │
│  │ • ⚠️ No Proof of Key Ownership                          │    │
│  └────────────────────────────────────────────────────────┘    │
│                          ↓                                       │
│  Layer 4: Task Security (✅ SECURE)                             │
│  ┌────────────────────────────────────────────────────────┐    │
│  │ • Assignment Tracking (Map-based)                       │    │
│  │ • Node ID Verification                                  │    │
│  │ • Replay Prevention (Set-based)                         │    │
│  │ • Task Expiration (5 min)                               │    │
│  └────────────────────────────────────────────────────────┘    │
│                          ↓                                       │
│  Layer 5: Credit Security (✅ SECURE)                           │
│  ┌────────────────────────────────────────────────────────┐    │
│  │ • QDAG Firestore Ledger (Source of Truth)               │    │
│  │ • Server-Only Crediting                                 │    │
│  │ • Credit Self-Reporting BLOCKED                         │    │
│  │ • Public Key-Based Ledger                               │    │
│  └────────────────────────────────────────────────────────┘    │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Task Lifecycle Security

```
┌─────────────┐
│  Submitter  │
│   (Node A)  │
└──────┬──────┘
       │
       │ 1. task_submit
       │    (task details + maxCredits)
       ↓
┌─────────────────────────────────────┐
│         Relay Server                │
│  ┌──────────────────────────────┐   │
│  │ 2. Generate Task ID           │   │
│  │    task-{timestamp}-{random}  │   │
│  └──────────────┬───────────────┘   │
│                 │                    │
│                 │ 3. Add to taskQueue│
│                 ↓                    │
│  ┌──────────────────────────────┐   │
│  │ 4. Select Random Worker       │   │
│  │    (from connected nodes)     │   │
│  └──────────────┬───────────────┘   │
│                 │                    │
│                 │ 5. Store Assignment│
│                 │    assignedTasks.set(taskId, {
│                 │      assignedTo: workerNodeId,
│                 │      assignedToPublicKey: workerPubKey,
│                 │      submitter: submitterNodeId,
│                 │      maxCredits: credits,
│                 │      assignedAt: timestamp
│                 │    })
│                 ↓                    │
└─────────────────┼────────────────────┘
                  │
                  │ 6. task_assignment
                  ↓
           ┌──────────────┐
           │   Worker     │
           │   (Node B)   │
           └──────┬───────┘
                  │
                  │ 7. Processes task
                  │
                  │ 8. task_complete
                  │    (taskId + result + reward)
                  ↓
┌─────────────────────────────────────┐
│         Relay Server                │
│  ┌──────────────────────────────┐   │
│  │ 9. SECURITY CHECKS:           │   │
│  │    ✓ Task in assignedTasks?   │   │
│  │    ✓ Node ID matches?         │   │
│  │    ✓ Not already completed?   │   │
│  │    ✓ Public key exists?       │   │
│  └──────────────┬───────────────┘   │
│                 │                    │
│                 │ 10. Mark Complete  │
│                 │     completedTasks.add(taskId)
│                 │     assignedTasks.delete(taskId)
│                 ↓                    │
│  ┌──────────────────────────────┐   │
│  │ 11. Credit Worker (Firestore)│   │
│  │     creditAccount(             │   │
│  │       publicKey,              │   │
│  │       rewardAmount,           │   │
│  │       taskId                  │   │
│  │     )                         │   │
│  └──────────────┬───────────────┘   │
│                 │                    │
│                 │ 12. Notify Worker  │
│                 │     credit_earned  │
│                 │                    │
│                 │ 13. Notify Submitter│
│                 │     task_result    │
└─────────────────┼────────────────────┘
                  │
                  ↓
         ┌────────────────┐
         │  QDAG Ledger   │
         │  (Firestore)   │
         │                │
         │  publicKey:    │
         │    earned: +X  │
         │    tasks: +1   │
         └────────────────┘
```

---

## Attack Vector Protections

### 1. Task Completion Spoofing Attack

```
ATTACK SCENARIO:
┌─────────────┐                    ┌─────────────┐
│  Attacker   │                    │   Victim    │
│  (Node C)   │                    │  (Node B)   │
└──────┬──────┘                    └──────┬──────┘
       │                                  │
       │                                  │ Assigned task-123
       │                                  │
       │ ❌ task_complete (task-123)      │
       │    "I completed it!"             │
       ↓                                  ↓
┌─────────────────────────────────────────────────┐
│              Relay Server                       │
│  ┌──────────────────────────────────────────┐   │
│  │ SECURITY CHECK:                          │   │
│  │   assignedTasks.get('task-123')          │   │
│  │   → { assignedTo: 'node-b' }             │   │
│  │                                          │   │
│  │   if (assignedTo !== attackerNodeId) {   │   │
│  │     ❌ REJECT: "Not assigned to you"     │   │
│  │   }                                      │   │
│  └──────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘

RESULT: ✅ Attack BLOCKED
```

### 2. Replay Attack

```
ATTACK SCENARIO:
┌─────────────┐
│   Worker    │
│  (Node B)   │
└──────┬──────┘
       │
       │ ✅ task_complete (task-123) → CREDITED 1000 RFI-IRFOS
       │
       │ Wait 1 second...
       │
       │ ❌ task_complete (task-123) → TRY AGAIN for another 1000 RFI-IRFOS
       ↓
┌─────────────────────────────────────────────────┐
│              Relay Server                       │
│  ┌──────────────────────────────────────────┐   │
│  │ FIRST COMPLETION:                        │   │
│  │   1. Verify assignment ✅                │   │
│  │   2. completedTasks.add('task-123')      │   │
│  │   3. assignedTasks.delete('task-123')    │   │
│  │   4. creditAccount() ✅                  │   │
│  └──────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────┐   │
│  │ SECOND COMPLETION (REPLAY):              │   │
│  │   1. Check: completedTasks.has(task-123) │   │
│  │      → TRUE                              │   │
│  │   2. ❌ REJECT: "Already completed"      │   │
│  └──────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘

RESULT: ✅ Replay Attack BLOCKED
```

### 3. Credit Self-Reporting Attack

```
ATTACK SCENARIO:
┌─────────────┐
│  Attacker   │
│  (Node C)   │
└──────┬──────┘
       │
       │ ❌ ledger_update
       │    {
       │      publicKey: "my-key",
       │      ledger: {
       │        earned: 999999999,  ← FAKE!
       │        spent: 0
       │      }
       │    }
       ↓
┌─────────────────────────────────────────────────┐
│              Relay Server                       │
│  ┌──────────────────────────────────────────┐   │
│  │ case 'ledger_update':                    │   │
│  │   console.warn("REJECTED")               │   │
│  │   ws.send({                              │   │
│  │     type: 'error',                       │   │
│  │     message: 'Credit self-reporting      │   │
│  │              disabled'                   │   │
│  │   })                                     │   │
│  │   ❌ RETURN (no action taken)            │   │
│  └──────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
       │
       │ Only way to earn credits:
       ↓
┌─────────────────────────────────────────────────┐
│  Complete assigned task → relay calls           │
│  creditAccount() → Firestore updated            │
└─────────────────────────────────────────────────┘

RESULT: ✅ Self-Reporting BLOCKED
```

### 4. Public Key Spoofing Attack

```
ATTACK SCENARIO:
┌─────────────┐                    ┌─────────────┐
│  Attacker   │                    │   Victim    │
│  (Node C)   │                    │  (Node V)   │
│             │                    │             │
│ pubKey:     │                    │ pubKey:     │
│ "victim-pk" │ ← SPOOFED          │ "victim-pk" │
└──────┬──────┘                    └─────────────┘
       │
       │ Register with victim's public key
       ↓
┌─────────────────────────────────────────────────┐
│              Relay Server                       │
│  ┌──────────────────────────────────────────┐   │
│  │ CURRENT PROTECTION (Limited):            │   │
│  │   ⚠️ Registration allowed                │   │
│  │      (no signature verification)         │   │
│  │   ✅ Credits assigned at task time       │   │
│  │      (uses publicKey from assignment)    │   │
│  └──────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘

CURRENT SECURITY:
  ✅ Attacker CANNOT steal victim's existing credits
     (ledger keyed by public key - victim still has their balance)

  ✅ Attacker CANNOT complete victim's assigned tasks
     (checked by NODE ID, not public key)

  ⚠️ Attacker CAN check victim's balance
     (ledger_sync is read-only, returns balance for any public key)

  ⚠️ Attacker working benefits victim
     (credits earned go to victim's public key)

NEEDED IMPROVEMENT:
  ❌ Add Ed25519 signature verification
  ❌ Challenge-response on registration
  ❌ Signature required on sensitive operations

RESULT: ⚠️ Partially Protected (needs signatures)
```

---

## QDAG Ledger Architecture

```
┌───────────────────────────────────────────────────────────┐
│                   QDAG Credit System                       │
│        (Quantum Directed Acyclic Graph Ledger)             │
└───────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                    Firestore Database                        │
│                    (SOURCE OF TRUTH)                         │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Collection: edge-net-qdag                            │  │
│  │  ┌─────────────────────────────────────────────────┐  │  │
│  │  │  Document: {publicKey}                          │  │  │
│  │  │  {                                              │  │  │
│  │  │    earned: 5000000,      ← Can only increase    │  │  │
│  │  │    spent: 1000000,                              │  │  │
│  │  │    tasksCompleted: 5,                           │  │  │
│  │  │    createdAt: 1234567890,                       │  │  │
│  │  │    updatedAt: 1234567899,                       │  │  │
│  │  │    lastTaskId: "task-123-xyz"                   │  │  │
│  │  │  }                                              │  │  │
│  │  └─────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────┬───────────────────────────────────────────┘
                  │
                  │ Synced via
                  ↓
┌─────────────────────────────────────────────────────────────┐
│              Relay Server (In-Memory Cache)                  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  ledgerCache = new Map()                              │  │
│  │    "pubkey1" → { earned: 5000000, spent: 1000000 }    │  │
│  │    "pubkey2" → { earned: 2000000, spent: 500000 }     │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                              │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Credit Functions (SERVER ONLY):                      │  │
│  │                                                        │  │
│  │  async function creditAccount(pubKey, amount, taskId) │  │
│  │    ✅ ONLY way to increase credits                    │  │
│  │    ✅ ONLY called after verified task completion      │  │
│  │    ✅ Writes to Firestore (source of truth)           │  │
│  │                                                        │  │
│  │  async function debitAccount(pubKey, amount, reason)  │  │
│  │    ✅ For spending credits                            │  │
│  │    ✅ Checks balance before debiting                  │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                  │
                  │ Clients can only:
                  ↓
┌─────────────────────────────────────────────────────────────┐
│                      Client Nodes                            │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  ledger_sync (read-only)                              │  │
│  │    → Returns balance from Firestore                   │  │
│  │                                                        │  │
│  │  ledger_update (BLOCKED)                              │  │
│  │    → Error: "Credit self-reporting disabled"          │  │
│  │                                                        │  │
│  │  task_complete (after assignment)                     │  │
│  │    → Relay verifies → calls creditAccount()           │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘

KEY SECURITY PROPERTIES:
  ✅ Credits keyed by PUBLIC KEY (identity, not node)
  ✅ Same public key = same balance across devices
  ✅ Server-side ONLY credit increases
  ✅ Firestore is single source of truth
  ✅ Clients cannot self-report credits
  ✅ Persistent across sessions
  ✅ Atomic operations with cache
```

---

## Security Control Matrix

| Control | Type | Status | Risk Mitigated | Code Location |
|---------|------|--------|----------------|---------------|
| Origin Validation | Preventive | ✅ Implemented | CSRF, unauthorized access | Lines 255-263 |
| Connection Limits | Preventive | ✅ Implemented | DoS (single IP) | Lines 308-315 |
| Rate Limiting | Preventive | ✅ Implemented | Message flooding | Lines 265-279 |
| Message Size Limits | Preventive | ✅ Implemented | DoS (large payloads) | Lines 338-342 |
| Heartbeat Timeout | Preventive | ✅ Implemented | Zombie connections | Lines 320-329 |
| Task Assignment Tracking | Detective | ✅ Implemented | Completion spoofing | Lines 222-229 |
| Assignment Verification | Preventive | ✅ Implemented | Completion spoofing | Lines 411-423 |
| Replay Prevention | Preventive | ✅ Implemented | Replay attacks | Lines 425-430 |
| Self-Report Blocking | Preventive | ✅ Implemented | Credit fraud | Lines 612-622 |
| Server-Only Crediting | Preventive | ✅ Implemented | Credit fraud | Lines 119-129 |
| QDAG Ledger | Preventive | ✅ Implemented | Credit tampering | Lines 66-117 |
| Task Expiration | Preventive | ✅ Implemented | Stale assignments | Lines 243-253 |
| Signature Verification | Preventive | ❌ NOT Implemented | Identity spoofing | Lines 281-286 |
| Challenge-Response | Preventive | ❌ NOT Implemented | Registration fraud | N/A |
| Global Conn Limit | Preventive | ❌ NOT Implemented | Distributed DoS | N/A |

---

## Security Metrics Dashboard

```
┌─────────────────────────────────────────────────────────────┐
│              Edge-Net Security Metrics                       │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Overall Security Score: 85/100                              │
│  ████████████████████████████████░░░░░░░░                   │
│                                                              │
│  Authentication:           50/100  ⚠️                        │
│  ██████████████████████░░░░░░░░░░░░░░░░░░                   │
│  (Missing signature verification)                            │
│                                                              │
│  Authorization:           100/100  ✅                        │
│  ████████████████████████████████████████                   │
│  (Task assignment verification excellent)                    │
│                                                              │
│  Credit System:           100/100  ✅                        │
│  ████████████████████████████████████████                   │
│  (QDAG ledger architecture excellent)                        │
│                                                              │
│  DoS Protection:           80/100  ✅                        │
│  ████████████████████████████████░░░░░░░░                   │
│  (Missing global connection limit)                           │
│                                                              │
│  Data Integrity:          100/100  ✅                        │
│  ████████████████████████████████████████                   │
│  (Firestore source of truth)                                 │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## Recommendations Priority Matrix

```
┌───────────────────────────────────────────────────────────────┐
│          Impact vs Effort Matrix                              │
│                                                                │
│  High Impact │                                                 │
│       ↑      │  🔴 Signature        │                          │
│       │      │     Verification     │                          │
│       │      │  (CRITICAL)          │  🟡 Global Conn          │
│       │      │                      │     Limit                │
│       │      ├──────────────────────┼──────────────────────────┤
│       │      │                      │                          │
│       │      │  🟡 Completed Tasks  │  🟢 Error Messages       │
│       │      │     Cleanup          │     Generic              │
│       │      │                      │                          │
│  Low Impact  │                      │                          │
│              └──────────────────────┴──────────────────────────┘
│                Low Effort  →  →  →  High Effort                │
└───────────────────────────────────────────────────────────────┘

Legend:
  🔴 Critical Priority (Do before production)
  🟡 Medium Priority (Do within 1-2 weeks)
  🟢 Low Priority (Nice to have)
```

---

## Audit Trail Example

```
Typical Secure Task Flow (with all security checks):

2026-01-03 10:15:23 [Relay] Node registered: node-abc-123 with identity 8a7f3d2e...
2026-01-03 10:15:24 [Relay] Task submitted: task-456-xyz from node-abc-123
2026-01-03 10:15:25 [Relay] Assigned task task-456-xyz to node-def-789
                            ↳ Assignment: {
                                assignedTo: 'node-def-789',
                                assignedToPublicKey: '8a7f3d2e...',
                                submitter: 'node-abc-123',
                                maxCredits: 2000000,
                                assignedAt: 1735900525000
                              }
2026-01-03 10:17:30 [Relay] Task task-456-xyz VERIFIED completed by node-def-789
                            ↳ Security checks passed:
                              ✅ Task in assignedTasks
                              ✅ Node ID matches assignment
                              ✅ Not already completed
                              ✅ Public key available
2026-01-03 10:17:31 [QDAG] Credited 0.002 RFI-IRFOS to 8a7f3d2e... for task task-456-xyz
2026-01-03 10:17:32 [QDAG] Saved ledger for 8a7f3d2e...: earned=0.005

Rejected Attack Example:

2026-01-03 10:20:15 [SECURITY] Task task-456-xyz was assigned to node-def-789, not node-evil-999 - SPOOFING ATTEMPT
2026-01-03 10:20:15 [Relay] Rejected task_complete from node-evil-999 (not assigned)
2026-01-03 10:20:20 [SECURITY] Task task-456-xyz already completed - REPLAY ATTEMPT from node-def-789
2026-01-03 10:20:20 [Relay] Rejected duplicate task_complete
2026-01-03 10:20:25 [QDAG] REJECTED ledger_update from node-evil-999 - clients cannot self-report credits
```

---

**Document Version**: 1.0
**Last Updated**: 2026-01-03
**Related Documents**:
- Full Security Audit Report: `SECURITY_AUDIT_REPORT.md`
- Quick Reference: `SECURITY_QUICK_REFERENCE.md`
- Test Suite: `/tests/relay-security.test.ts`
