# Splitting language counterexample.
## Example 1.
En este caso, el problema es que solo eliminamos mundos del input.

```rust
    let mut f1: ModalFormula = build_formula("(diamond p) and (diamond (not p)) and (diamond q) and (diamond (not q))").unwrap();
    let mut f2: ModalFormula = build_formula("box p").unwrap();
    let mut f3: ModalFormula = build_formula("(box p) and (diamond q) and (diamond (not q))").unwrap();
```

## Example 2.
En este caso es interesante porque en DSUM depende del conjunto de átomos.
```rust
    let mut f1: ModalFormula = build_formula("box p").unwrap();
    let mut f2: ModalFormula = build_formula("not p").unwrap();
    let mut f3: ModalFormula = build_formula("(not p) and (diamond p) ").unwrap();
```
