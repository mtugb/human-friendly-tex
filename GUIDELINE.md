# mytex Guideline

mytex is a human-friendly shorthand that compiles to LaTeX.
Write indented, readable expressions — mytex turns them into valid LaTeX.

---

## Core concept: indentation = structure

Every line is either a **command** or a **leaf** (plain text).
Indented lines beneath a command become its arguments or body.

```
command
  argument1
  argument2
```

---

## Math block

Wrap math content in `$` (inline) or `$$` (display, multi-line).

```
$
  x^2 + y^2 = z^2
```

Compiles to:
```latex
\begin{align*}x^2 + y^2 = z^2\end{align*}
```

---

## Commands

### `frac` / `f` — Fraction

```
frac
  numerator
  denominator
```

→ `\frac{numerator}{denominator}`

---

### `sqrt` — Square root

```
sqrt
  x + 1
```

→ `\sqrt{x + 1}`

---

### `nsqrt` — N-th root

```
nsqrt
  3
  x + 1
```

→ `\sqrt[3]{x + 1}`

---

### `abs` — Absolute value

```
abs
  x - a
```

→ `\left|x - a\right|`

---

### `norm` — Norm

```
norm
  v
```

→ `\left\|v\right\|`

---

### `ceil` — Ceiling

```
ceil
  x
```

→ `\left\lceil x\right\rceil`

---

### `floor` — Floor

```
floor
  x
```

→ `\left\lfloor x\right\rfloor`

---

### `lim` — Limit

```
lim
  n
  inf
  a_n
```

→ `\lim_{n \to \infty} a_n`

---

### `sum` — Summation

```
sum
  i=0
  n
  a_i
```

→ `\sum_{i=0}^{n} a_i`

---

### `prod` — Product

```
prod
  i=1
  n
  x_i
```

→ `\prod_{i=1}^{n} x_i`

---

### `int` / `integral` — Integral

```
int
  0
  inf
  f(x)
  x
```

→ `\int_{0}^{\infty} f(x) \, dx`

---

## Matrix environments

Rows are child lines. Columns are separated by **spaces** within a row.

### `mat` / `m` — Bracket matrix

```
mat
  a b
  c d
```

→ `\begin{bmatrix}a&b\\c&d\end{bmatrix}`

---

### `|mat|` / `|m|` — Determinant (vertical bar matrix)

```
|mat|
  a b
  c d
```

→ `\begin{vmatrix}a&b\\c&d\end{vmatrix}`

---

### `mat^t` / `m^t` — Transposed matrix

```
mat^t
  a b
  c d
```

→ `\begin{bmatrix}a&b\\c&d\end{bmatrix}^{\top}`

---

## Document structure

These use Regex-type matching — write the pattern directly as a leaf line.

| Input | Output |
|---|---|
| `# Title` | `\section{Title}` |
| `## Title` | `\subsection{Title}` |
| `### Title` | `\subsubsection{Title}` |

---

### `itemize` — Bullet list

```
itemize
  First item
  Second item
  Third item
```

→ `\begin{itemize}\item First item\item Second item\item Third item\end{itemize}`

---

## Leaf symbol replacements

These are applied automatically to any plain-text (leaf) line.

### Arrows & logic

| Write | LaTeX |
|---|---|
| `->` | `\rightarrow` |
| `<-` | `\leftarrow` |
| `<->` | `\leftrightarrow` |
| `=>` | `\Rightarrow` |
| `AA` | `\forall` |
| `EE` | `\exists` |

### Comparison

| Write | LaTeX |
|---|---|
| `<=` | `\leq` |
| `>=` | `\geq` |
| `!=` | `\neq` |

### Constants

| Write | LaTeX |
|---|---|
| `inf` | `\infty` |
| `...` | `\ldots` |

### Greek letters

| Write | LaTeX |
|---|---|
| `alpha` | `\alpha` |
| `beta` | `\beta` |
| `gamma` | `\gamma` |
| `theta` | `\theta` |
| `lambda` | `\lambda` |
| `sigma` | `\sigma` |
| `omega` | `\omega` |
| `pi` | `\pi` |

---

## Full example

Input:

```
$
  |mat|
    a b
    c d
  = a*d - b*c
```

Output:

```latex
\begin{align*}\begin{vmatrix}a&b\\c&d\end{vmatrix}= a*d - b*c\end{align*}
```
