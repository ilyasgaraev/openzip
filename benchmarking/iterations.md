# Iterations per second

**Openzip** vs **rubyzip**.

---
## Zip file

### File info
* **Type:** zip
* **Files in archive:** 2500
* **Size:** 6 500 079 bytes (6,5 MB)

### Results

```
Warming up --------------------------------------
             openzip     1.000  i/100ms
             rubyzip     1.000  i/100ms
Calculating -------------------------------------
             openzip      0.841  (± 0.0%) i/s -      5.000  in   5.943962s
             rubyzip      0.369  (± 0.0%) i/s -      2.000  in   5.416802s

Comparison:
             openzip:        0.8 i/s
             rubyzip:        0.4 i/s - 2.28x  slower
```

---
## Excel file

### File info
* **Type:** xlsx
* **Files in archive:** 13
* **Size:** 775 231 bytes (778 KB)
* **Filled rows:** 5000

```
Warming up --------------------------------------
             openzip     1.000  i/100ms
             rubyzip     1.000  i/100ms
Calculating -------------------------------------
             openzip     18.270  (± 5.5%) i/s -     91.000  in   5.000976s
             rubyzip     13.548  (± 7.4%) i/s -     68.000  in   5.033290s

Comparison:
             openzip:       18.3 i/s
             rubyzip:       13.5 i/s - 1.35x  slower
```
