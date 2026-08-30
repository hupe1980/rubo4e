+++
title = "Documentation"
description = "How rubo4e is put together, what it guarantees, and where the domain rules behind the BO4E standard come from."
sort_by = "weight"
template = "section.html"
page_template = "page.html"
insert_anchor_links = "left"
+++

These pages cover the design and the domain, not the API surface — for signatures
and per-item documentation, use the [API reference](https://docs.rs/rubo4e).

BO4E (*Geschäftsobjekte für die Energiewirtschaft*) is the object model the German
energy industry uses to exchange contracts, metering points, invoices, and the
parties involved. The standard is published as JSON Schema; `rubo4e` generates
Rust types from it and adds the rules the schema itself cannot express — check
digits, cross-field invariants, and strict decoding boundaries.

If you are new to the crate, read **Architecture** first for the layout, then
**Identifiers** for the part that carries the most domain weight.

If you are extending it — because a market rule needs a fact BO4E has no field
for — read **Beyond the Schema**. It carries the one test every addition passes,
and the reason a generated enum is never forked.
