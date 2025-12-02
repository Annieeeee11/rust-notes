# Mutable vs Immutable

## Concepts

- By default each variable is **immutable** in Rust
- Add `mut` keyword to make it mutable (means you can change value)

---

## Scope

Variables exist only inside the block `{}` where you define them. Once the block ends, the variable is dropped and no longer accessible

---

## Shadowing

When you declare a new variable with the same name as an existing one, it **shadows** (hides) the previous one

- The old variable still exists, just hidden
- Inner block shadows only affect that block
- After block ends, outer variable is visible again

**Shadowing vs mut:**
- `mut` lets you change the value of the same variable
- Shadowing creates a completely NEW variable (can even change type)

---

## Freezing

A mutable variable becomes temporarily immutable when it's borrowed immutably. You can't modify it while the borrow is active

**Two ways freezing happens:**
1. **Immutable borrow** - can't modify while someone holds a reference
2. **Shadowing with immutable binding** - shadow a `mut` variable without `mut`

Once the borrow ends (goes out of scope), the variable is "unfrozen" and can be modified again