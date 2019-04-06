---
title: Conditional compilation
references:
---
![type state picture]()
---
#### Platform specific section

##### If platform {}
```scl
---<Android>---
A -> B @ D
---
```

##### If platform {} else {}
```scl
---<Android>---
A -> B @ D
---<*>---
A -> B @ S
---
```

##### If platform {} else if otherplatform {}
```scl
---<Android>---
A -> B @ D
---<Web>---
A -> B @ S
---
```

##### If platform {} else if otherplatform {} else {}
```scl
---<Android>---
A -> B @ D
---<Web>---
A -> B @ S
---<*>---
A -> B |> kill
---
```

##### Globbing
```scl
---< x86_64-*-gnu >---
A -> B @ D
---

---< arm* >---
A <-> B @ D
---
```