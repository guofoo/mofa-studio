# Conference Controller Policy System

The conference controller uses a policy system to determine **who speaks next** in multi-participant conversations. This document explains the policy syntax and behavior.

## Policy Configuration

Set the policy in your dataflow YAML under the conference-controller node:

```yaml
- id: conference-controller
  env:
    DORA_POLICY_PATTERN: "[(human, 0.001), (tutor, *), (student1, 1), (student2, 1)]"
```

## Two Policy Modes

### 1. Sequential Mode (`→` syntax)

Simple round-robin ordering. Each participant speaks in order, then loops.

**Syntax:**
```
[judge → defense → prosecution]
A → B → C
```

**Behavior:**
```
Turn 1: judge
Turn 2: defense
Turn 3: prosecution
Turn 4: judge (cycle 1)
Turn 5: defense
...
```

### 2. Ratio/Priority Mode (`,` syntax)

Flexible turn-taking based on weights and priorities.

**Syntax:**
```
[(judge, 2), (defense, 1), (prosecution, 1)]    # Explicit ratios
[(judge, *), (defense, 1), (prosecution, 1)]    # Priority + ratios
[judge, defense, prosecution]                    # Equal ratios (1.0 default)
```

## Weight Types

| Weight | Syntax | Description |
|--------|--------|-------------|
| **Priority** | `*` | Always speaks next (unless they just spoke) |
| **Ratio** | `2.0`, `1.5`, `1`, etc. | Target proportion of speaking time |

## Algorithm

### Priority Selection
1. If a priority (`*`) participant exists AND didn't just speak → they go next
2. Priority participants take precedence over ratio-based selection

### Ratio-Based Selection
For each non-priority participant, calculate:
```
ideal_words = (participant_ratio / total_ratio) × total_words_spoken
ratio_difference = (ideal_words - actual_words) / participant_ratio
```

Select the participant with the **highest ratio_difference** (most "behind" their target).

### Cold Start (Initial Conversation Only)
On **initial** conversation start (no one has spoken yet), priority participants are skipped to let ratio-based participants speak first. This prevents priority speakers from always dominating the opening.

**Important:** Cold start only applies at the very beginning. When human speaks (interrupt), it is NOT treated as cold start - the system sets `last_speaker = "human"` so priority participants (tutor) respond first.

### No Back-to-Back
A participant cannot speak twice in a row, regardless of priority or ratio.

## Examples

### Example 1: Study Session (voice-chat.yml)

```yaml
DORA_POLICY_PATTERN: "[(human, 0.001), (tutor, *), (student1, 1), (student2, 1)]"
```

| Participant | Weight | Behavior |
|-------------|--------|----------|
| `human` | 0.001 (ratio) | Included in ratio calculation but rarely selected due to tiny weight |
| `tutor` | `*` (priority) | Always responds after others (unless just spoke) |
| `student1` | 1.0 (ratio) | Equal share with student2 |
| `student2` | 1.0 (ratio) | Equal share with student1 |

**Typical flow:**
```
1. [Cold start] → student1 or student2 speaks (priority skipped at start)
2. student1 done → tutor speaks (priority kicks in)
3. tutor done → student2 speaks (ratio balancing)
4. student2 done → tutor speaks (priority)
5. tutor done → student1 speaks (ratio balancing)
...
[Human interrupts - NOT cold start]
6. human speaks → system resets, tutor responds (priority, human just spoke)
7. tutor done → student1 or student2 continues (ratio balancing)
8. student done → tutor speaks (priority)
...cycle continues
```

**What happens when human speaks:**
1. All pending TTS audio is cleared
2. All bridges reset (clear buffered messages)
3. All LLMs receive cancel signal
4. Text segmenter discards old segments
5. Word counts reset to 0, but `last_speaker` is set to `"human"`
6. Tutor (priority) responds first because human just spoke
7. Normal conversation continues with ratio balancing

### Example 2: Debate Format

```yaml
POLICY: "[(judge, *), (defense, 1), (prosecution, 1)]"
```

- Judge always responds after each argument (priority)
- Defense and prosecution alternate based on word count ratio

### Example 3: Panel Discussion

```yaml
POLICY: "[(moderator, 3), (expert1, 2), (expert2, 2), (guest, 1)]"
```

Speaking time distribution over conversation:
- moderator: ~37.5% (3/8)
- expert1: ~25% (2/8)
- expert2: ~25% (2/8)
- guest: ~12.5% (1/8)

### Example 4: Simple Round-Robin

```yaml
POLICY: "[A → B → C]"
```

Strict sequential order: A, B, C, A, B, C, ...

## Policy State

The controller tracks:

| State | Purpose |
|-------|---------|
| `word_counts` | Cumulative words per participant (for ratio balancing) |
| `last_speaker` | Who just spoke (prevents back-to-back) |
| `position` | Current position in sequence (sequential mode) |
| `cycle` | How many complete rounds have occurred |
| `round_speakers` | Who has spoken in current round |

## Reset Behavior

When human speaks (interrupt), the controller performs a full system reset:

### Pipeline Reset
1. **Cancel all LLMs** - Abort any ongoing streaming responses
2. **Reset all bridges** - Clear buffered messages waiting to be forwarded
3. **Reset audio pipeline** - Text segmenter discards old segments, audio player clears buffer
4. **Increment round number** - New `question_id` allows downstream nodes to discard stale data

### Policy Reset
1. **Reset word counts to 0** - Fresh ratio balancing for new conversation segment
2. **Set `last_speaker` to `"human"`** - Prevents cold start, ensures priority speaker (tutor) responds
3. **Clear round tracking** - Reset cycle counters

### Result
- Tutor (priority `*`) speaks first after human input
- Then normal conversation continues with ratio-based turn-taking
- Each conversation "segment" (between human inputs) has balanced participation

## Participant Name Mapping

The policy uses logical names that map to dataflow node IDs:

| Policy Name | Dataflow Node | Description |
|-------------|---------------|-------------|
| `human` | ASR output | Human speaker via microphone |
| `tutor` | `tutor` node | Tutor LLM (e.g., deepseek-chat) |
| `student1` | `student1` node | Student 1 LLM (e.g., gpt-4.1) |
| `student2` | `student2` node | Student 2 LLM (e.g., gpt-4.1) |
| `judge` | Judge LLM | Moderator/judge role |

## Best Practices

1. **Use priority (`*`) for moderators/tutors** - Ensures they respond after each participant
2. **Use equal ratios for peer participants** - `student1=1, student2=1` gives balanced discussion
3. **Use higher ratios for experts** - `expert=2, novice=1` lets experts speak more
4. **Use tiny ratio for human** - `(human, 0.001)` includes human in policy but AI rarely selects them
5. **Avoid too many priority participants** - Only one or two should have `*` weight
6. **Priority speaker should respond to human** - Tutor with `*` will always respond after human speaks

## Debugging

Get policy stats via control command:
```json
{"command": "stats"}
```

Returns:
```json
{
  "mode": "ratio_priority",
  "participants": ["human", "tutor", "student1", "student2"],
  "weights": [
    {"name": "human", "weight": 0.001},
    {"name": "tutor", "weight": "*"},
    {"name": "student1", "weight": 1.0},
    {"name": "student2", "weight": 1.0}
  ],
  "word_counts": {
    "human": 150,
    "tutor": 890,
    "student1": 420,
    "student2": 380
  },
  "cycle": 3,
  "current_speaker": "student1"
}
```
