# Cross-Implementation Compatibility Vectors

This directory holds JSON payloads that represent the wire format produced by
different BO4E implementations. The Rust test suite deserializes all of them to
verify byte-for-byte compatibility with the BO4E standard.

## Directory layout

```
compat/
├── python/        — Payloads representing BO4E-python output format
│   ├── marktlokation.json
│   ├── messlokation.json
│   ├── vertrag.json
│   └── rechnung.json
└── go/            — Payloads representing go-bo4e output format
    ├── marktlokation.json
    ├── messlokation.json
    ├── vertrag.json
    └── rechnung.json
```

## How to regenerate vectors

### Python (BO4E-python ≥ v202501.0.0)

```bash
pip install bo4e
python - <<'EOF'
import json
from bo4e import Marktlokation, Sparte, Energierichtung, Bilanzierungsmethode, Netzebene, Adresse

malo = Marktlokation(
    marktlokations_id="51238696780",
    sparte=Sparte.GAS,
    energierichtung=Energierichtung.EINSP,
    bilanzierungsmethode=Bilanzierungsmethode.PAUSCHAL,
    netzebene=Netzebene.NSP,
    lokationsadresse=Adresse(postleitzahl="10115", ort="Berlin", strasse="Unter den Linden", hausnummer="1"),
)
print(malo.model_dump_json(indent=2))
EOF
```

Update `python/marktlokation.json` with the printed output.

Repeat for the other types (`Messlokation`, `Vertrag`, `Rechnung`).

### Go (go-bo4e)

```bash
go run ./cmd/dump-fixtures  # if such a tool exists in go-bo4e
```

Or instantiate the structs in a small Go program and use `json.MarshalIndent`.

## Running the tests

```bash
cargo test --features json,versioned --test compat
```

## When to update

Update these vectors whenever:
- A new BO4E schema version is adopted (update `_version` strings)
- A breaking field rename or type change occurs upstream
- A new BO4E implementation is added to the compatibility matrix
