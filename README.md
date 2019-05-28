Current thoughts:
---
Driven by config file for each EDI type. Config file would be TOML. Example:

```
version = "5010x218"
format = "820"
[[loop]]
    [loop.1000A]
        name = "Premium Receiver's Name"
        n1 = [""]
        n2 = [""]
        n3 = [""]
        n4 = [""]
        rdm = [""]
    [loop.1000B]
        name = "Premium Payer's Name"
        n1 = [""]
        n2 = [""]
        n3 = [""]
        n4 = [""]
        per = [""]
    [loop.1000C]
        name = "Intermediary Bank Information"
    [loop.2000A]
        name = "Organization Summary Remittance"
        [loop.2000A.2200A]
            name = "Organization Summary Remittance Level Adjustment For Previous Payment"
        [loop.2000A.2300A]
            name = "Organization Summary Remittance Detail"
            [loop.2000A.2300A.2310A]
                name = "Summary Line Item"
            [loop.2000A.2300A.2312A]
                name = "Service, Promotion, Allowance, Or Charge Information"
            [loop.2000A.2300A.2315A]
                name = "Member Count"

```

That is an 820 file version 5010x218.