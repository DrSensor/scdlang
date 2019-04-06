---
title: File separation
references:
---
![type state picture]()
---
### Import with no side effect
```scl
use './relative/path/explicit/extension.scl'
```
Read as: "use all statements in the `extension.scl`, no transition expression is included"

#### Partial use statement
```scl
use {
  State0
  CompoundState1[NestedCoumpundState],
  CompoundState2[
    NestedCoumpundState as C2,
    NState
  ],
} from './relative/path/explicit/extension.scl'
```

### Side effect import
```scl
import './relative/path/explicit/extension.scl'
```
Read as: "import all things on the `extension.scl`, including the transition expression"

#### Partial use statement but with/include the side effect
```scl
use {
  State0,
  CompoundState1[NestedCoumpundState],
  CompoundState2[
    NestedState as [C2],
    NState
  ],
} import './relative/path/explicit/extension.scl'
```
