# Digital Storage Units (`gpt-6.2-sol`)

Decimal and binary byte units declared as `unit_base` facts, plugging directly into `vault/core/skills/units.md` with zero new rules.

```karin
unit_base(unit="b", dim="storage", to_base="1")
unit_base(unit="kb", dim="storage", to_base="1000")
unit_base(unit="mb", dim="storage", to_base="1000000")
unit_base(unit="gb", dim="storage", to_base="1000000000")
unit_base(unit="tb", dim="storage", to_base="1000000000000")
unit_base(unit="kib", dim="storage", to_base="1024")
unit_base(unit="mib", dim="storage", to_base="1048576")
unit_base(unit="gib", dim="storage", to_base="1073741824")
unit_base(unit="tib", dim="storage", to_base="1099511627776")
```
