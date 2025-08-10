# Process API Simulator - Homework Solutions

This document contains solutions and analysis for the process tree simulator exercises using `fork.py`.

## Question 1: Basic Process Tree Prediction

**Task**: Run `./fork.py -s 10` and predict the process tree at each step.

### Prediction Process:
Looking at the actions generated with seed 10:
```
Action: a forks b
Action: a forks c  
Action: c EXITS
Action: a forks d
Action: a forks e
```

### Step-by-Step Analysis:

**Initial State:**
```
a
```

**Step 1: a forks b**
```
a
└── b
```
Process `a` creates child process `b`. The tree now has two processes.

**Step 2: a forks c**
```
a
├── b
└── c
```
Process `a` creates another child process `c`. Now `a` has two children: `b` and `c`.

**Step 3: c EXITS**
```
a
└── b
```
Process `c` terminates and is removed from the tree. Only `a` and `b` remain.

**Step 4: a forks d**
```
a
├── b
└── d
```
Process `a` creates a new child process `d`.

**Step 5: a forks e**
```
a
├── b
├── d
└── e
```
Process `a` creates another child process `e`. Final tree has `a` as root with three children.

### Key Insights:
- Parent processes can create multiple children
- When a process exits, it's removed from the tree entirely
- Children of exiting processes may be reparented (see Question 4)
- Process names are assigned alphabetically in order of creation

## Question 2: Fork Percentage Effects

**Task**: Run with `-a 100` and vary fork percentage from 0.1 to 0.9 to observe tree shapes.

### Low Fork Percentage (0.1):
```bash
./fork.py -a 20 -f 0.1 -c
```

**Observations:**
- **Shallow trees**: Most actions are exits, so processes die quickly
- **Sequential pattern**: Typically `a` forks child, child immediately exits
- **Simple structure**: Usually just root with one or no children at end
- **High turnover**: Many processes created and destroyed

**Example result**: Root process `a` with possibly one surviving child, as most children exit shortly after being created.

### High Fork Percentage (0.9):
```bash
./fork.py -a 20 -f 0.9 -c
```

**Observations:**
- **Deep, branching trees**: Processes keep creating children before exiting
- **Multi-generational**: Grandchildren, great-grandchildren, etc.
- **Complex hierarchy**: Multiple levels of parent-child relationships
- **Wide branches**: Each process may have multiple children

**Example result**: 
```
a
├── b
│   └── l
├── d
│   ├── e
│   │   ├── k
│   │   └── m
│   ├── f
│   │   ├── g
│   │   ├── j
│   │   │   ├── n
│   │   │   │   └── o
│   │   │   └── s
│   │   └── q
│   └── p
└── i
    └── r
```

### Fork Percentage Impact:
- **0.1**: Flat, simple trees with high process turnover
- **0.5**: Balanced trees with moderate depth and width  
- **0.9**: Deep, complex hierarchical structures

## Question 3: Tree-to-Action Analysis

**Task**: Use `-t` flag to show trees and determine what actions were taken.

### Example with seed 10:
```bash
./fork.py -s 10 -t
```

**Given process trees, determine actions:**

```
Step 0: a
Step 1: a
        └── b           → Action: a forks b

Step 2: a
        ├── b           → Action: a forks c
        └── c

Step 3: a
        └── b           → Action: c EXITS (c disappears)

Step 4: a
        ├── b           → Action: a forks d
        └── d

Step 5: a
        ├── b           → Action: a forks e
        ├── d
        └── e
```

### Analysis Strategy:
1. **New process appears**: Someone forked (determine parent by tree position)
2. **Process disappears**: That process exited
3. **Structural changes**: May indicate reparenting due to exits

### Challenges:
- **Ambiguous forks**: When multiple processes could be the parent
- **Timing uncertainty**: Can't tell exact order of simultaneous actions
- **Reparenting effects**: Process exits may change apparent relationships

## Question 4: Orphaned Processes

**Task**: Analyze what happens to children when parent exits using: `./fork.py -A a+b,b+c,c+d,c+e,c-`

### Without -R flag (default reparenting):
```
Initial: a
Step 1:  a → b (a forks b)
Step 2:  a → b → c (b forks c)  
Step 3:  a → b → c → d (c forks d)
Step 4:  a → b → c → d (c forks e)
                  → e
Step 5:  a → b (c exits)
         a → d (orphaned children reparented to init/root)
         a → e
```

**Result**: `d` and `e` become direct children of `a` (reparented to closest ancestor).

### With -R flag (local reparenting):
```bash
./fork.py -A a+b,b+c,c+d,c+e,c- -R -c
```

**Result**: 
```
a
└── b
    ├── d  (reparented to immediate parent b)
    └── e
```

### Key Insights:
- **Default behavior**: Orphaned processes reparented to init process (usually root)
- **Local reparenting (-R)**: Orphaned processes reparented to nearest living ancestor
- **Real systems**: In Unix, orphaned processes become children of init (PID 1)
- **Process cleanup**: Parent processes are responsible for cleaning up child resources

### Real-World Context:
In actual Unix systems:
- When parent dies before child, child becomes "orphan"
- Orphans are adopted by init process (PID 1)
- Init process handles cleanup when orphaned children exit
- This prevents "zombie" processes that consume system resources

## Question 5: Final Tree Prediction

**Task**: Use `-F` flag to predict final tree from action sequence.

### Example with seed 15:
```bash
./fork.py -s 15 -F
```

**Given actions:**
```
Action: a forks b
Action: a forks c
Action: c EXITS
Action: b EXITS  
Action: a forks d
```

### Step-by-step analysis:
1. **a forks b**: Tree = `a → b`
2. **a forks c**: Tree = `a → b, c`
3. **c EXITS**: Tree = `a → b` (c removed)
4. **b EXITS**: Tree = `a` (b removed)
5. **a forks d**: Tree = `a → d`

**Final prediction:**
```
a
└── d
```

### Strategy for Final Tree Prediction:
1. **Track all forks**: Note parent-child relationships
2. **Process exits**: Remove process and handle orphans  
3. **Apply reparenting rules**: Based on flags used
4. **Final state**: Only surviving processes remain

## Question 6: Reverse Engineering Actions

**Task**: Use `-t -F` to determine actions from final tree only.

### Example 1 - Deterministic case:
```bash
./fork.py -s 20 -t -F -c
```

**Final tree:**
```
a
├── c
│   └── d
└── e
```

**Determined actions:**
```
Action: a forks b
Action: b EXITS
Action: a forks c
Action: c forks d  
Action: a forks e
```

### Cases where actions CAN be determined:
1. **Linear chains**: `a → b → c` suggests `a` forked `b`, then `b` forked `c`
2. **Simple structures**: Few processes with clear hierarchies
3. **Unique patterns**: Distinctive arrangements that suggest specific sequences

### Cases where actions CANNOT be definitively determined:

**Example of ambiguity:**
```
Final tree:
a
├── b
└── c
```

**Possible action sequences:**
- Sequence 1: `a+b, a+c` (a forks both b and c)
- Sequence 2: `a+b, a+c, a+d, d-, b+e, e-` (many intermediate steps with exits)
- Sequence 3: `a+b, b+c, a+d, d-` (b creates c, then b reparented)

### Ambiguity Factors:
1. **Multiple valid sequences**: Same final tree can result from different actions
2. **Hidden exits**: Processes that were created and exited leave no trace
3. **Reparenting effects**: Orphaned processes may appear to have different origins
4. **Timing independence**: Order of some operations doesn't affect final result

### Analysis Strategy:
1. **Start simple**: Look for most straightforward explanation
2. **Consider exits**: Missing intermediate nodes suggest exits occurred
3. **Check constraints**: Use knowledge of simulator rules (alphabetical naming, etc.)
4. **Accept ambiguity**: Sometimes multiple valid solutions exist

## Summary of Key Learning Points

### Process Tree Concepts:
1. **Hierarchical structure**: Parent-child relationships form tree
2. **Dynamic nature**: Processes can be created and destroyed
3. **Orphan handling**: System must manage processes whose parents exit
4. **Resource management**: Process trees help track system resource usage

### Simulator Insights:
1. **Fork percentage**: Controls tree depth vs breadth
2. **Reparenting policies**: Different strategies for handling orphans
3. **Action sequences**: Multiple sequences can yield same final state
4. **Reverse engineering**: Final state doesn't always uniquely determine history

### Real-World Applications:
1. **Process monitoring**: Tools like `ps`, `pstree` show similar hierarchies
2. **System administration**: Understanding process relationships for troubleshooting
3. **Container orchestration**: Managing process trees in containerized environments
4. **Shell job control**: Shells manage process groups and sessions

This simulator effectively demonstrates the complexity and nuances of process management in operating systems, highlighting both the deterministic rules and the ambiguities that can arise in process tree analysis.