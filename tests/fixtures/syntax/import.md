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
Read as: "import all transitions on the `extension.scl`, including the nested transition"

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

### Import as a service (a.k.a external State Machine)
```scl
import './relative/path/explicit/extension.scl' as service extMachine
```
Read as: "import all transition as a *service* with a name **extMachine**"

```scl
use * import './relative/path/explicit/extension.scl' as service extMachine
```
Read as: "use all statements in the `extension.scl` while also import all transitions as a *service* with a name **extMachine**""

### Import as a specific state
```scl
import './relative/path/explicit/extension.scl' as state Extend
```
or
```scl
import './relative/path/explicit/extension.scl' as compound state Extend
```
Read as: "import all transition as a *compound state* with a name **Extend**"

```scl
import './relative/path/explicit/extension.scl' as parallel state Extend
```
Read as: "import all transition as a *parallel state* with a name **Extend**"
