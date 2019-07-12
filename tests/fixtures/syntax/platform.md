---
title: Conditional compilation
references:
status: 🤔
---
![type state picture]()
---
#### Platform specific section
This feature akin to environment variable that enable/disable specific code.

##### If platform {}
<details><summary>multi-line</summary>

```scl
---<Android>---
A -> B @ D
---
```
</details>

<details><summary>one liner</summary>

```scl
Android>- A -> B @ D
```
</details>

##### If platform {} else {}
<details><summary>multi-line</summary>

```scl
---<Android>---
A -> B @ D
---<*>---
A -> B @ S
---
```
</details>

<details><summary>one liner</summary>

```scl
Android>- A -> B @ D
*>- A -> B @ D
```
</details>

##### If platform {} else if otherplatform {}
<details><summary>multi-line</summary>

```scl
---<Android>---
A -> B @ D
---<Web>---
A -> B @ S
---
```
</details>

<details><summary>one liner</summary>

```scl
Android>- A -> B @ D
Web>- A -> B @ S
```
</details>

##### If platform {} else if otherplatform {} else {}
<details><summary>multi-line</summary>

```scl
---<Android>---
A -> B @ D
---<Web>---
A -> B @ S
---<*>---
A -> B |> kill
---
```
</details>

<details><summary>one liner</summary>

```scl
Android>- A -> B @ D
Web>- A -> B @ S
*>- A -> B |> kill
---
```
</details>

##### Globbing
<details><summary>multi-line</summary>

```scl
---< x86_64-*-gnu >---
A -> B @ D
---

---< arm* >---
A <-> B @ D
---
```
</details>

<details><summary>one liner</summary>

```scl
x86_64-*-gnu>- A -> B @ D
arm*>- A <-> B @ D
```
</details>
