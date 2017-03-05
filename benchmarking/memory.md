# Memory usage

**Openzip** vs **rubyzip**.

---
## Zip file

### File info
* **Type:** zip
* **Files in archive:** 2500
* **Size:** 6 500 079 bytes (6,5 MB)

### Results

```
Calculating -------------------------------------
             openzip   627.000  memsize (    40.000  retained)
                        10.000  objects (     1.000  retained)
                         3.000  strings (     0.000  retained)
             rubyzip   304.757M memsize (   120.000  retained)
                       866.162k objects (     3.000  retained)
                        50.000  strings (     2.000  retained)

Comparison:
             openzip:        627 allocated
             rubyzip:  304757473 allocated - 486056.58x more
```

---
## Excel file

### File info
* **Type:** xlsx
* **Files in archive:** 13
* **Size:** 775 231 bytes (778 KB)
* **Filled rows:** 5000

```
Calculating -------------------------------------
             openzip   626.000  memsize (    40.000  retained)
                        10.000  objects (     1.000  retained)
                         3.000  strings (     0.000  retained)
             rubyzip    62.955M memsize (    40.000  retained)
                         2.763k objects (     1.000  retained)
                        50.000  strings (     0.000  retained)

Comparison:
             openzip:        626 allocated
             rubyzip:   62954673 allocated - 100566.57x more
```
