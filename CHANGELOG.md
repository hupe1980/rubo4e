# Changelog

All notable changes to `rubo4e` are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Each release that changes schema-derived enum membership or codelist coverage
also carries a **Schema deltas** section (see [Versioning](https://hupe1980.github.io/rubo4e/docs/versioning/))
so downstream guards (SQL `CHECK` lists, variant-count assertions, coverage
tests) can be updated deliberately instead of discovering drift at runtime.

## [0.12.0] — unreleased

This release is a wire-format correction. Every payload rubo4e produced carried
an invalid `_version`, COMs never stamped `_typ`, and `Rechnung` could not read
what the reference implementation emits. If you exchange JSON with any other
BO4E implementation, this is not an optional upgrade.

It also advances the schema snapshot to **v202607.1.0** and makes the versioning
contract say what BO4E actually does inside a series — see **Schema deltas** and
**Changed** below.

### Fixed *(breaking)*

- **Every payload rubo4e produced carried an invalid `_version`.** The generator
  filled the field in from the schema *release tag*, which BO4E spells
  `v202607.0.0`. The value the standard puts inside a payload has no `v`:
  `202607.0.0`. Every BO and every COM this crate serialized therefore claimed a
  version string that no BO4E schema accepts and no other implementation writes,
  breaking the byte-compatibility the README promises.

  `_version` is now read out of the schema's own `default`, and
  `Bo4eObject::schema_version()` returns the same wire spelling so the accessor
  and the field cannot disagree. `tests/generated_contract.rs` pins both against
  the committed schemas.

  **Action required:** code matching on `schema_version()` or on a stored
  `bo4e_version` column must match the wire spelling, not the tag. Match the
  **series** — `version.split('.').next() == Some("202607")` — rather than the
  full triple, or the next BO4E patch release breaks the match again; the new
  `Bo4eObject::schema_series()` returns exactly that value. Rows written by an
  earlier release carry the invalid `v`-prefixed value and need a one-time
  `UPDATE … SET _version = '202607.1.0'`.

- **COMs never stamped `_typ`.** Every BO4E COM schema pins its discriminant with
  a JSON Schema `const`, so pydantic emits it on every component the reference
  implementation produces — `Adresse`, `Betrag`, `Zeitraum`, all of them. rubo4e
  left the field `None`, making a Rust-built component distinguishable from one
  produced anywhere else in the ecosystem. `Default` and the builder now stamp it
  for COMs exactly as they always did for BOs.

  The test that should have caught this asserted the opposite — that a COM *must
  not* carry `_typ` — so it pinned the defect in place. It has been replaced by a
  schema-driven guard over all ~96 struct types.

- **`Zeitraum` read `enddatum` as exclusive; BO4E says it is inclusive.** The
  schema states it on the field — *"Enddatum des betrachteten Zeitraums ist
  **inklusiv**"* — and gives `'2025-01-01'` as the example for `startdatum`
  *and* `enddatum`, so `start == end` is a one-day period. The crate modelled the
  interval half-open, which dropped a day from every period:

  - `Zeitraum::contains(enddatum)` returned `false`, so the last day of a billing
    month fell outside its own `rechnungsperiode`, and
    `PreisblattNetznutzung::is_valid_at` reported a price sheet invalid on its
    final day of validity.
  - `validate_zeitraum` required a strict `startdatum < enddatum` and therefore
    **rejected every single-day Zeitraum** — the most common shape in a
    daily-granularity payload.

  Both are fixed. The accessors changed shape to make the convention
  unmisreadable:

  | Before | After |
  |---|---|
  | `as_closed_range() -> Option<(Date, Date)>` | `as_inclusive_range() -> Option<RangeInclusive<Date>>` |
  | `as_half_open_range() -> Option<(Date, Option<Date>)>` | `bounds() -> (Option<Date>, Option<Date>)` |
  | `Rechnung::billing_period() -> Option<(Date, Date)>` | `-> Option<RangeInclusive<Date>>` |
  | `PreisblattNetznutzung::validity() -> Option<(Date, Option<Date>)>` | `-> (Option<Date>, Option<Date>)` |

  Returning a `RangeInclusive` rather than a tuple puts the convention in the
  type: `range.contains(&d)` is correct by construction, and there is no
  `start..end` / `start..=end` decision left to get wrong.

  **This is not a uniform rule across BO4E** — the same release uses three
  interval conventions, and the crate had generalised the wrong one:

  | Kind | Interval |
  |---|---|
  | `date-time` pairs (`vertragsbeginn`/`vertragsende`, `von`/`bis`) | `[start, end)` |
  | `Zeitraum`'s **date** pair | `[start, end]` |
  | `Zeitraum`'s **time** pair (`startuhrzeit`/`enduhrzeit`) | `[start, end)` |
  | price-tier bounds (`staffelgrenzeVon`/`Bis`) | `[von, bis]` |

  `tests/interval_conventions.rs` now reads each statement out of the committed
  schema and checks it against the code, so a release that flips one fails CI.
  The `Vertrag` rule was already right and is unchanged.

- **The snake_case key transform rewrote keys inside extension data.** An
  extension field holding `{"a": 3, "marktlokations_id": "x"}` came back out of a
  `to_json_snake_case` → `from_json_snake_case` round-trip spelled
  `{"A": 3, "marktlokationsId": "x"}`, because `A` (from `Sigmoidparameter`) and
  `marktlokationsId` are real BO4E field names *somewhere*. A producer's own
  payload was silently rewritten into names it does not use, and the round-trip
  identity the `json` module documents for extension data did not hold.

  The transform renames keys as the parser yields them, long before serde knows
  which struct they belong to, so on its own it renamed *every* key at *every*
  depth. It now switches itself off — for the whole subtree — the moment it
  descends into a value under a key the schema does not define. The generator
  emits the key set it consults (`KNOWN_FIELD_KEYS`), so it is exact rather than
  heuristic. `tests/extension_round_trip.rs` pins both halves: extension subtrees
  pass through byte-for-byte, and schema keys are still renamed at every depth.

  Two ambiguities remain, and are now documented and pinned rather than silent: a
  *top-level* extension key that is a field's snake spelling is
  indistinguishable from the field, and `ZusatzAttribut.wert` — free-form JSON
  under a name that is also `Betrag`'s decimal and `Messwert`'s nested COM — is
  still descended into. Use `to_json_german` where extension data matters.

- **`Rechnung` could not read the reference implementation's own output.**
  `rechnungsdatum` and `faelligkeitsdatum` were forced to `time::Date` by a
  per-field override table, on the grounds that BDEW INVOIC transmits them as DTM
  qualifier 102. The BO4E schema declares both `format: date-time`, so a
  `Rechnung` produced by BO4E-python or the .NET implementation **failed to
  deserialize outright**. No fixture set either field, so nothing caught it.

  Both are now `time::OffsetDateTime`, following the schema. The override
  mechanism that allowed inference to beat an explicit schema annotation is
  removed: a `$ref` and a `"format"` are now always authoritative. The date-only
  reading is still one call away via `Rechnung::rechnungsdatum_date()` and
  `faelligkeitsdatum_date()`, which now return the calendar date of the timestamp.

  **Action required:** if you were reading these fields as `time::Date`, either
  switch to the `*_date()` accessors or take `.date()` yourself.

- **Two different gas meter sizes shared one enum variant.** `MESSPREIS_G2_5`
  (meter size **G 2.5**) and `MESSPREIS_G25` (**G 25**) both converted to
  `Messpreistyp::MesspreisG25`. The collision "resolver" then renamed the second
  to `MESSPREISG25` — a SCREAMING identifier that reads as a typo and gives no
  hint which size it means, so half of every call site that picked
  `MesspreisG25` meant the other meter. `SMART_METER_MESSPREIS_G2_5` /
  `_G25` collided the same way.

  Enum variant naming now preserves the separator between two digit runs:
  `MesspreisG2_5` and `MesspreisG25` are distinct and self-describing. Generation
  now **fails** if two wire values ever collapse onto one identifier, instead of
  papering over it.

  **Action required:** `Messpreistyp::MESSPREISG25` → `Messpreistyp::MesspreisG25`;
  the old `MesspreisG25` (which meant G 2.5) → `MesspreisG2_5`. Same for the
  `SmartMeter…` pair.

- **`Rechnung`'s `zuZahlen` rule rejected correctly discounted invoices.** The
  validator enforced `zu_zahlen == gesamtbrutto − rabatt_netto − Σ vorauszahlungen`.
  `rabattNetto` is a **net** discount; subtracting it from a gross total is short
  by the VAT on the discount. The equation the schema's own text names references
  a `rabattBrutto` field v202607 does not ship, so it is not reconstructible from
  the payload at all — the rule was invented rather than derived.

  The `zuZahlen` check is **removed**. In its place the validator gained a rule
  BO4E does state outright: `steuerbetraege` must sum to `gesamtsteuer`. The
  currency-agreement guard and `gesamtnetto + gesamtsteuer == gesamtbrutto` are
  unchanged.

- **`Kostenposition` line-total arithmetic rejected ordinary invoices.** It
  compared `einzelpreis × menge` against the stated amount at ten decimal places.
  A unit price of `0.2843 €/kWh` over `3333 kWh` is `947.5719`, which every
  invoice writes as `947.57` — so the rule failed on essentially all real data.
  It also applied to time-proportional positions, which the schema computes with
  a different formula entirely.

  The comparison now allows half a unit in the amount's own last decimal place
  (accepting either rounding mode), and positions carrying a `zeitmenge` are
  skipped rather than measured against the wrong formula.

- **Three fields took a Rust type from their *name* instead of their schema, and
  every object containing one failed to deserialize.** Field typing was keyed on
  bare names and on name suffixes, so:

  | Field | Schema says | Was typed as |
  |---|---|---|
  | `Kontaktweg.kontaktwert` | `"string"` — *"Die Nummer oder E-Mail-Adresse"* | `Decimal` (suffix `wert`) |
  | `MarktgebietInfo.marktgebiet` | `"string"` — *"Der Name des Marktgebietes"* | `EicCode` |
  | `StandorteigenschaftenStrom.regelzone` | `"string"` — *"Der Name der Regelzone"* | `EicCode` |

  The failure was never local to the field: a `Geschaeftspartner` carrying **any**
  contact method failed to deserialize whole — organisation name, address, and
  VAT ID with it. `Marktteilnehmer` was exposed the same way, since a market
  partner's contact details are exactly what it stores. The `schemars` / `utoipa`
  output was wrong too, advertising `pattern: ^-?\d+(\.\d+)?([eE]\d+)?$` for an
  e-mail field.

  All three are now `String`. The two `EicCode` cases were homonyms of fields
  that genuinely do carry a code — `Marktlokation.marktgebiet` and
  `Marktlokation.regelzone` are documented *"Code vom EIC"* and keep their
  validated type.

  **Action required:** `Kontaktweg.kontaktwert`,
  `MarktgebietInfo.marktgebiet`, and `StandorteigenschaftenStrom.regelzone` are
  now `Option<String>`. Any workaround that lifted `kontaktwege` out of a payload
  before a typed read can be dropped.

- **Without `decimal`, a JSON *number* would not deserialize at all.** The
  `String` fallback used serde's own `String` impl, which rejects a number — so a
  `versioned`-only build could read BO4E-python output (which writes decimals as
  strings) but not go-bo4e's (which writes numbers), despite the docs promising
  the fallback preserves the value. The fallback now accepts either spelling and
  keeps the lexical form.

- **`to_json_canonical()` did not sort everywhere it claimed to.** Sequences,
  tuples, tuple structs, and both enum-variant shapes delegated straight to the
  inner serializer, so an object nested inside any of them came out unsorted while
  the method still advertised canonical output. All five shapes now route through
  the sorting serializer; the enum variants keep their external tag.

- **Every `from_json_*` entry point accepted trailing content after the
  document.** They built a `serde_json::Deserializer` by hand and never called
  `end()`, so the parser stopped at the close of the first JSON value and never
  looked at the rest: `{"marktlokationsId":"51238696781"} <anything>` decoded
  successfully, while `serde_json::from_str` on the same bytes rejects it.

  A reader that accepts what the reader in front of it rejects is a parser
  differential. A gateway, a schema gate, or a signature check validating with
  plain `serde_json` and a service decoding with rubo4e would disagree about what
  the payload even *is* — and the bytes past the first document are the ones only
  one of them sees. All eight entry points (German and snake_case, `&str` and
  `&[u8]`, hardened and not) now consume the whole input.
  `tests/json_strictness.rs` asserts acceptance matches `serde_json` exactly,
  trailing whitespace included.

- **The snake_case readers skipped the nesting-depth cap entirely.** The docs say
  every path enforces a depth limit of 128, hardened or not;
  `from_json_snake_case` and `from_json_snake_case_bytes` wrapped the
  deserializer for the key transform but not for depth, so
  `JsonParseLimits::max_nesting_depth` was silently ignored on the non-hardened
  snake path and only `serde_json`'s own limit applied. Both now carry the guard.

- **`ObisCode` broke the `Borrow<str>` contract, so map lookups by string
  silently missed.** It derives `Hash` over its cached `ObisComponents` as well
  as its canonical string, while `impl_identifier_traits!` gives every identifier
  `Borrow<str>` — and `Borrow` requires the borrowed form to hash and compare
  exactly as the owned one does. `HashMap<ObisCode, _>::get("1-0:1.8.0")`
  returned `None` for a key that was present: no error, no panic, a wrong answer.

  `Eq`, `Hash`, `PartialOrd`, and `Ord` are now written by hand against the
  canonical string alone (`components` is derived from it, so nothing is lost).
  `ObisCode` also gains the `Ord` the identifier docs have always promised for
  every member of the family. Look up by the **canonical** spelling —
  `"1-0:1.8.0"`, not `"01-00:01.08.00"`.

- **`validate_kostenposition_arithmetic` panicked on an amount at `Decimal`'s
  maximum scale.** The rounding tolerance is half a unit in the amount's last
  stated place, built with `Decimal::new(5, scale + 1)` — and `Decimal::new`
  *panics* above scale 28 rather than returning an error. A payload can carry a
  28-scale amount, and this validator is what an ingest boundary runs on
  untrusted input, so a decodable payload could abort the process.

  It now uses `try_new` and falls back to an exact comparison, which is the right
  answer at a scale that leaves no room for a tolerance anyway. The fuzz build
  gained the `validate` feature and every BO target now validates what it
  decoded, which is the configuration that would have found this.

- **Identifier validation failures logged the rejected value.** With `tracing`
  on, a `MeloId`, `MaloId`, or `Iban` that failed to deserialize was emitted
  verbatim at `warn!` — personal and payment data copied into whatever the log
  sink happens to be, in exactly the case where the value is least trustworthy.
  The event now carries the identifier type, the input's byte length, and the
  error (which already names the offending position and the expected shape).

- **`Marktlokation` / `Messlokation` validation required an Ortsangabe that BO4E
  does not.** Both validators demanded that **exactly one** of
  `lokationsadresse` (`messadresse`), `geoadresse`, and `katasterinformation` be
  present. BO4E states mutual exclusivity, not presence — *"Es darf immer nur
  eine Art der Ortsangabe vorhanden sein"* — and the schema backs that exactly:
  no `required` array, no `oneOf`, all three properties `"default": null`, and
  BO4E-python carrying the rule as a comment over three `Optional[…] = None`
  fields with no validator behind it.

  The reading is **at most one**, and the difference is not academic. BO4E has no
  reference type, so a location referenced from a `Rechnung`, a `Vertrag`, or an
  `Angebot` is a full `Marktlokation` carrying little more than its ID — the most
  common shape in circulation, and `Validated::new` rejected every one of them.
  For `Messlokation` the schema is blunter still: `messadresse` is documented
  *"Nur angeben, wenn diese von der Adresse der Marktlokation abweicht"*, so a
  Messlokation matching its Marktlokation is **supposed** to carry none.

  The message now names which fields conflict rather than restating the rule.

- **`validate_rechnung_arithmetic` mixed a house rule in with the sourced ones.**
  Three of its four checks quote a sentence of the schema. The fourth — "if two
  of the three totals are present the third must be too" — is this crate's own
  judgement, and BO4E states nothing of the kind. Because `Validated::new` is
  all-or-nothing, a consumer checking a document a **counterparty** sent could
  not assert *"this conforms to BO4E"* without also asserting *"…and satisfies
  `rubo4e`"*, with no way to separate the two.

  `.validate()` now runs **only** rules traceable to the schema. The house rule
  moved to `rubo4e::validation::current::quality::rechnung_totals_are_complete`,
  which is not wired into any derive and is called by name. That boundary is now
  a documented contract rather than an accident of what seemed sensible.

- **`bo_type()` reported the `_typ` the payload carried, not the type it was
  read into.** It was `self.typ.unwrap_or(<the schema constant>)`, so a
  `Marktlokation` decoded from `{"_typ":"VERTRAG", …}` answered
  `BoTyp::Vertrag` — and a `match bo.bo_type()` on a concrete value took the
  branch the *sender* named rather than the one the value's own type calls for.
  That is a type confusion reachable from any payload.

  `bo_type()` now returns the discriminant the schema pins for that Rust type,
  which is what "what kind of business object is this" means for a value whose
  type is already known. The `_typ` field is public and unchanged, so a payload's
  claim is still there — and `value.typ != Some(value.bo_type())` is now the way
  to detect a payload whose discriminant disagrees with the type it was read
  into. `AnyBo` is unaffected: it dispatches on `_typ` by design, and each of its
  variants then holds the matching concrete type.

- **`.validate()` did not descend into nested values.** `garde(dive)` was emitted
  only on identifier-typed fields, so a struct's own cross-field rules ran and
  nothing below them did: `Rechnung::validate()` never checked the `Zeitraum` on
  `rechnungsperiode`, `Kosten::validate()` never checked a `Kostenposition`'s
  line total two levels down, and an invalid identifier below the first level was
  not looked at at all. `Validated<T>` proved considerably less than its
  documentation claimed.

  The generator now emits `garde(dive)` for every field whose type carries rules
  — the identifier newtypes and every nested BO and COM — so one `.validate()`
  covers the tree, and each failure is reported at its path
  (`kostenbloecke[0].kostenpositionen[0]`).

  **Action required:** values that passed `.validate()` before may now fail, and
  the ones that do were already carrying a rule violation this crate was not
  reporting. Enums and scalars are untouched; presence is still not checked,
  because BO4E declares almost every field optional.

### Changed *(breaking)*

- **The `_typ` facts are associated constants on a new `Bo4eTyped` trait, over
  BOs *and* COMs, and the trait is not `dyn`-compatible.** `TYP`, `TYP_WIRE`,
  `SCHEMA_VERSION`, and `SCHEMA_SERIES` are readable from a type without a value;
  `typ_wire()`, `schema_version()`, and `schema_series()` are provided methods.
  `Bo4eObject` and `Bo4eComponent` are sealed markers over it, narrowing the set
  and binding `Typ` to `BoTyp` or `ComTyp`.

  The point is generic code. Reading a discriminant used to mean constructing a
  value — `T::default().bo_type()` — which silently excludes the two types the
  schema marks `required`, because a `T: Default` bound does not admit `Lastgang`
  or `Tarif`. Nothing failed; they were never reached.

  Putting the constants on a trait that spans both kinds is what lets a consumer
  write **one bound over "anything with a `_typ`"**, which is what a gate at a
  wire boundary actually wants — a service gating `Marktlokation` and `Rechnung`
  alongside `Energiemix`, `Zahlungsinformation`, and `Vorauszahlung` no longer
  needs two accessors. It also removes the `Default` dependency from the COM
  half, where no schema declares a `required` field *today* but one release doing
  so would drop that COM out of a `T: Default` bound exactly as `Lastgang` was.

  `ZusatzAttribut` implements none of the three: it is the single BO4E schema
  that declares no `_typ`.

  There is deliberately **no `typ()` method** beside the public `typ` field. Two
  spellings a keystroke apart, one meaning the payload's claim and the other the
  type's own identity, is the confusion `TYP` exists to prevent.

  Associated constants make a trait dyn-incompatible, so `Box<dyn Bo4eObject>` is
  gone. The traits are **sealed**, so their implementors are a closed set, and
  this crate already ships the sum type over exactly `Bo4eObject`'s. `AnyBo` is
  `Clone`, `PartialEq`, `Serialize`, `Deserialize`, and matchable — none of which
  a trait object is — and it now carries the same facts (`typ_wire()`,
  `schema_version()`, and `schema_series()` are new; the latter two return
  `Option` because the `Unknown` catch-all has no generated type to report one
  for).

  **Action required:** `T::BO_TYP` becomes `T::TYP` and `v.bo_type()` becomes
  `T::TYP`; import `Bo4eTyped` where you imported `Bo4eObject` for the accessors.
  Replace `Vec<Box<dyn Bo4eObject<BoTyp = BoTyp>>>` with `Vec<AnyBo>` — `.into()`
  converts each element.

- **`schemars`'s `rust_decimal1` and `utoipa`'s `decimal` / `time` features moved
  to the features that need them.** They are enabled as
  `schemars?/rust_decimal1`, `utoipa?/decimal`, and `utoipa?/time` from the
  `decimal` and `time` features, so they turn on exactly when the generated
  fields *are* those types, and neither pulls the integration in.

  Enabling them unconditionally made this crate a silent sole provider of
  `impl JsonSchema for Decimal` to its whole workspace: Cargo unifies features,
  so a sibling crate deriving `JsonSchema` over its own `Decimal` field compiled
  for as long as *something* in the graph kept `rubo4e/schemars` on — and broke
  when that was turned off for an unrelated reason.

  **Action required:** if you derive `JsonSchema` over a `rust_decimal::Decimal`
  of your own, declare `schemars = { features = ["rust_decimal1"] }` in your own
  manifest. It was never this crate's impl to provide.

- **`BoTyp` and `ComTyp` variants are named after the types they discriminate.**
  Those two enums are the one place where BO4E's SCREAMING_SNAKE_CASE values have
  a *known* word split — `"PREISBLATTKONZESSIONSABGABE"` names the
  `PreisblattKonzessionsabgabe` schema sitting beside it in the same release — and
  the generator was throwing that away. Renames:

  | Before | After |
  |---|---|
  | `BoTyp::Preisblattdienstleistung` | `BoTyp::PreisblattDienstleistung` |
  | `BoTyp::Preisblatthardware` | `BoTyp::PreisblattHardware` |
  | `BoTyp::Preisblattkonzessionsabgabe` | `BoTyp::PreisblattKonzessionsabgabe` |
  | `BoTyp::Preisblattmessung` | `BoTyp::PreisblattMessung` |
  | `BoTyp::Preisblattnetznutzung` | `BoTyp::PreisblattNetznutzung` |
  | `BoTyp::Technischeressource` | `BoTyp::TechnischeRessource` |
  | `BoTyp::Steuerbareressource` | `BoTyp::SteuerbareRessource` |
  | `ComTyp::Aufabschlag` | `ComTyp::AufAbschlag` |
  | `ComTyp::Einheitspreisposition` | `ComTyp::EinheitsPreisposition` |
  | `ComTyp::Lastvariablepreisposition` | `ComTyp::LastvariablePreisposition` |
  | `ComTyp::Relativepreisposition` | `ComTyp::RelativePreisposition` |
  | `ComTyp::Zeitvariablepreisposition` | `ComTyp::ZeitvariablePreisposition` |
  | `ComTyp::Marktgebietinfo` | `ComTyp::MarktgebietInfo` |
  | `ComTyp::Standorteigenschaftengas` | `ComTyp::StandorteigenschaftenGas` |
  | `ComTyp::Standorteigenschaftenstrom` | `ComTyp::StandorteigenschaftenStrom` |
  | `ComTyp::Verwendungszweckpromarktrolle` | `ComTyp::VerwendungszweckProMarktrolle` |

  Wire values are unchanged; only the Rust identifiers moved.

- **Word boundaries inside all-caps enum values are now recovered.**
  `Zaehlergroesse::G2komma5` → `G2Komma5`,
  `Rechnungstyp::Integrierte13teRechnung` → `Integrierte13TeRechnung` (likewise
  `Zusaetzliche13teRechnung` and `NetznutzungRechnungstyp`'s pair), and
  `Dienstleistungstyp::Auslesung2xTaeglichFernauslesung` →
  `Auslesung2XTaeglichFernauslesung`. Wire values unchanged.

- **The schema snapshot advanced to v202607.1.0**, and with it the documented
  versioning contract. The old wording said minor bumps inside a series are
  "additive" and that a version module's enum membership is "fixed for the
  series". BO4E's own `v202607.0.0` → `v202607.1.0` removed a value and two whole
  enums, so neither was true. The contract now reads:

  > The Rust module path pins the **series**. The `rubo4e` version pins the
  > **values**.

  Pin the crate version in `Cargo.toml` for a frozen variant set; guard the rest
  structurally with `T::VARIANTS` / `T::COUNT`.

- **`Zeitraum::as_closed_range` → `as_bounded_range`.** The name promised an
  inclusive end that no other method in the crate delivers: `contains` has always
  read `[startdatum, enddatum)`, and `validate_zeitraum` has always required
  `startdatum < enddatum`. A caller who read "closed" and wrote `start..=end`
  billed one day too many on every period. The behaviour is unchanged; the name
  and the docs now say what it is, and the exclusive-end convention is stated
  once, with its provenance, on the `Zeitraum` impl block.

- **`JsonParseLimits` is `#[non_exhaustive]`.** Build one from
  `untrusted_defaults()` or `unlimited()` plus the new `with_*` methods rather
  than a struct literal. A hardening knob set grows as new amplification paths
  are found, and a struct literal made each of those a breaking change — exactly
  the pressure that keeps such an API from gaining the cap it needs. It now also
  derives `PartialEq` / `Eq`.

  ```rust
  // Before
  let limits = JsonParseLimits { max_nesting_depth: Some(16), ..Default::default() };
  // After
  let limits = JsonParseLimits::unlimited().with_max_nesting_depth(Some(16));
  ```

- **`LimitedExtensionMap::try_insert` returns a `Result`, not a `bool`.** It now
  yields the displaced value the way `HashMap::insert` does, or an
  `ExtensionInsertError` naming which cap stopped it. It also **fixes a defect**:
  a map at `MAX_EXTENSION_FIELDS` refused to *overwrite* an existing key, which
  does not grow the map and had no reason to fail — a full extension map's
  contents were unwritable.

- **Decimal fields deserialize through `crate::decimal_serde` in both builds.**
  Previously only the `decimal`-off (`String`) path had a custom deserializer.
  Behaviour for well-formed input is unchanged; see *Added* for what this buys.

### Added

- **Three more code-bearing fields are now validated newtypes.** Auditing the
  name/code pairs above turned up code halves left untyped while their name half
  was over-typed:

  | Field | Schema says | Now |
  |---|---|---|
  | `StandorteigenschaftenStrom.regelzoneEic` | *"De EIC-Nummer der Regelzone"* | `EicCode` |
  | `Fremdkostenposition.gebietcodeEic` | *"EIC-Code … Z.B. '10YDE-EON------1'"* | `EicCode` |
  | `Netzlokation.obiskennzahl` | *"Die OBIS-Kennzahl für die Netzlokation"* | `ObisCode` |

  The last one was skipped only because BO4E spells it `obiskennzahl` where the
  other three OBIS fields use `obisKennzahl`; a name-keyed table could not see
  past the casing.

- **`sqlx::postgres::PgHasArrayType` for every generated enum**, so `Vec<Sparte>`
  binds to a `TEXT[]` column the way `Vec<MaloId>` already did. The README and
  the ecosystem guide had claimed this for several releases while only
  identifiers carried the impl; `tests/sqlx_compile.rs` now asserts both.
- **`Rechnung` validates that `steuerbetraege` sum to `gesamtsteuer`** — a rule
  the BO4E schema states outright and the crate was not checking.
- **The fuzz targets build with `time` and `decimal`.** Those two features
  replace `String` fields with `time::OffsetDateTime`, `time::Date`, and
  `rust_decimal::Decimal` — three real parsers over attacker-controlled text.
  Fuzzing ran in the one configuration where none of them is compiled in.
  `fuzz_parse_identifiers` now covers all sixteen identifier types (it covered
  eight), asserts each accepts its own `Display` output, and exercises the
  accessors that do arithmetic and slicing on a parsed value.
- **`tests/generated_contract.rs`** — schema-driven drift guards over the whole
  committed codegen: schema ↔ module coverage, `_typ` and `_version` stamping for
  all ~96 struct types, discriminant variant naming, and injectivity of the wire
  value → Rust variant mapping. It replaces `tests/module_coverage.rs`, whose
  generated-file assertions silently never ran (they looked for
  `src/generated/v202607/bo/`, a directory the generator does not create).
- **`tests/sqlx_compile.rs`** — compile-time assertions that every identifier and
  every enum carries the `Type` / `Encode` / `Decode` / `PgHasArrayType` surface
  the docs promise.

- **`Bo4eObject::schema_series()`** — the `YYYYMM` prefix of the release
  (`"202607"`), which is the granularity a module actually covers and therefore
  the only sane key for version dispatch. `schema_version()` keeps returning the
  full wire triple; matching on *that* rejects a payload from a sender one BO4E
  patch ahead, for types that read it perfectly. The docs' dispatch example was
  written the wrong way round and is corrected.

- **`Eq` and `Hash` on generated structs and `AnyBo`** (without the `json`
  feature), so a BO can key a `HashMap` or a `HashSet`. `Hash` alone shipped
  before — with a comment claiming it enabled exactly that — and a `HashMap` key
  requires `Eq`, so it never did. `tests/hash_keys.rs` uses the types as keys, in
  both feature modes.

- **`PartialOrd` and `Ord` on every generated enum**, so one can key a `BTreeMap`,
  be sorted, or let a caller's struct derive `Ord`. The order is BO4E's own
  declaration order with `Unknown` last: total, deterministic, and explicitly not
  a business ranking — documented once on `Bo4eEnum`, deliberately *not* in each
  enum's rustdoc, which schemars and utoipa lift verbatim into the wire-contract
  `description`.

- **`decimal_serde::decimal_from_json_number_count()`** — a process-wide counter
  of decimals read from a JSON *number* rather than a JSON string, exported as
  `bo4e_decimal_from_json_number_total` with the `metrics` feature.

  BO4E spells a decimal both ways: BO4E-python writes `"wert": "119.00"`, go-bo4e
  writes `"wert": 119.00`. Both were already read, but only the string spelling
  is exact — a JSON number has passed through `f64` before any Rust deserializer
  sees it, so `119.00` arrives as `119` and anything past ~15 significant digits
  is rounded. That is unrecoverable in serde's data model, so it is instead made
  visible and documented in full, with the boundary pinned by a test. Integers
  now take an exact path rather than going through `f64`.

- **`LimitedExtensionMap::get` / `len` / `is_empty` / `iter`**, and
  `ExtensionInsertError`. The extension map had a public mutator and no way to
  read back what it held without going through a generated type's
  `Bo4eExtensionData`. Its `PartialEq` is now hand-written so an allocated-empty
  map compares equal to one that never allocated.

- **`JsonParseLimits::with_*` builders** — see *Changed*.

- **`Zeitraum::whole_days()` and `Zeitraum::duration()`.** `whole_days` counts
  both inclusive bounds, so a one-day period is 1 and January is 31.
  `duration()` parses `dauer`, which BO4E stores as an ISO 8601 string
  (`"P1DT30H4S"`) that neither serde nor `time` reads.

- **`Iban` and `Bic`** (ISO 13616 / ISO 9362) — the two fields on a BO4E invoice
  that money moves against, and the one obvious hole left in the identifier
  family. `Iban` verifies the ISO 7064 MOD-97-10 check digits, which catch every
  single-character error and every adjacent transposition, and enforces the
  registered per-country length (a 21-character German IBAN is rejected here
  rather than by the bank). Grouping spaces and lowercase normalise away, so a
  value pasted from a statement parses; `to_grouped_string()` renders the print
  form back. Accessors split a German IBAN into Bankleitzahl and Kontonummer, and
  a BIC into institution, country, location and branch, with `is_head_office()`
  and `is_passive()`.

  A country the crate's length table does not list is **not** rejected on length
  — the ISO registry grows, and a stale table refusing a valid IBAN would be
  worse than one the checksum alone vetted.

  **`Zahlungsinformation.iban` / `.bic` stay `String` on the generated struct.**
  That COM hangs off `Rechnung` and nothing else, so typing them would mean a
  masked IBAN (`DE89 **** **** 3000` — routine on an invoice) destroys the entire
  invoice: line items, amounts, periods and all. `iban_checked()` and
  `bic_checked()` run the check on demand and return an error instead. The
  generator's typing rules gained a fourth rule recording this trade-off, and
  `Bilanzierung.bilanzkreis` is now documented as the same kind of exception.

- **`rubo4e::iso8601_duration`** — the parser behind `duration()`, public because
  `dauer` is not the only place BO4E spells a duration this way. `P1Y` and `P1M`
  are **refused**, not approximated: a year is 365 or 366 days and a month 28 to
  31, so converting either without a start date is a guess. Weeks and below are
  exact and parse fine. The error names which unit was ambiguous and why.

- **`rubo4e::offset_time`, and the accessors that use it** —
  `Zeitraum::startuhrzeit_parsed` / `enduhrzeit_parsed` and
  `Umschaltzeit::umschaltzeit_parsed`. BO4E annotates those three properties
  `format: "time"` and gives `"18:00:00+01:00"` as the example: a time of day
  **with a UTC offset**, which no `time` type holds — `Time` carries no zone and
  `OffsetDateTime` demands a date. The fields therefore stay `String` rather than
  silently dropping the offset, and this reads them into
  `(Time, Option<UtcOffset>)`.

  The offset is returned as `Option` rather than defaulted to UTC: BO4E does not
  require one, and "local time, zone not stated" is a different claim. It is also
  load-bearing — a Doppeltarif switch written `06:00:00+01:00` is a different
  wall-clock moment in summer than in winter, so discarding it moves the tariff
  boundary by an hour.

- **`Preisstaffel::contains` and `PreisstaffelSliceExt::select_for`** — price-tier
  lookup, including BO4E's gap rule. The schema states tiers as
  `0 – 1000, 1001 – 2000` and rules that a value *between* two tiers
  (`1000.6`) *"rutscht in die obere Zone"* — a naive `von <= x <= bis` scan finds
  no tier for it and bills nothing. `select_for` picks the tier with the smallest
  `staffelgrenzeBis` still ≥ the value, which satisfies the in-tier and in-gap
  cases together and does not depend on slice order.

  Note that BO4E's own wording for `staffelgrenzeBis` contradicts itself —
  *"**Exklusiver** oberer Wert, bis zu dem die Staffel gilt (**inklusiv**)"* —
  and it is the worked example in the same sentence that settles it. Both the
  contradiction and the resolution are pinned by tests.

- **`tests/prelude_surface.rs`** — a drift guard proving `rubo4e::prelude`
  re-exports *every* identifier type. It did not: `CrId`, `NebeId`, `SgId`, and
  `PaketId` were reachable only through `rubo4e::identifiers::`, so the BDEW
  Ressourcen-ID family was half in and half out with nothing marking the line.
  All four, plus `MaloVergabestelle`, `MpIdAuthority`, and `LengthExpectation`,
  are now in the prelude.

- **`tests/pinned_tag.rs`** — the schema tag and the MSRV each have one source of
  truth, and nothing may write either out again. The tag is the single directory
  name under `generator/schemas/`; the MSRV is `rust-version` in `Cargo.toml`.
  The justfile (`just pinned-tag`), the CI workflow's regenerate-and-diff step,
  the test helpers, and the site config all derive them.

  Without this the values drift silently in the places least likely to be read:
  a workflow step fails on a directory that no longer exists, and the site's
  landing page and footer advertise a schema release the crate is not generated
  from.

- **A guard over the schema's `format` annotations.** `resolve_field_type` ends
  in a catch-all that maps an unrecognised `format` to `String` — the right
  default, but a silent one: a release that starts annotating a field
  `"format": "uri"` would map it to `String` with nobody deciding it should.
  `generator/tests/round_trip.rs` now pins the set, so a new format fails there
  and the decision gets made. `"time"` is recorded as a deliberate passthrough
  rather than an oversight.

- **The generator deletes generated files a schema release retired.** BO4E
  dropping `Lokationstyp` and `Mengenoperator` would otherwise have left two
  orphan modules in `src/generated/`, unreferenced by `mod.rs`, compiling
  nowhere, and passing every drift check — indistinguishable from a live type.

- **`rubo4e::validation::current`** — the counterpart of `rubo4e::current`, so no
  downstream file has to name a schema version to reach a validator, and a CI
  guard that greps for `rubo4e::v202607` stays clean. Without it, a consumer who
  wanted one well-sourced rule had to either hardcode the version or take
  `Validated::new` and every rule with it. `tests/validation.rs` pins the alias
  to the versioned module by function-pointer identity.

- **`rubo4e::validation::current::quality`** — rules this crate considers
  sensible that BO4E does not state, kept out of `.validate()` on purpose and
  called by name. `rechnung_totals_are_complete` is the first.

- **`Lastgang::new(…)` and `Tarif::new(…)`.** These are the only two BOs the
  v202607 schema marks `required`, so neither derives `Default` — which left
  them the only generated types that could not be constructed without the
  `builder` feature or writing out every optional field by hand. `new` takes the
  required fields and defaults the rest, stamping `_typ` and `_version` exactly
  as `Default` does elsewhere. The generator emits one for any struct with
  required fields, and `tests/generated_contract.rs` reads `required` out of the
  schemas and pins the correspondence in both directions.

- **`Validated<T>` is now `Deserialize`, and the impl validates.** It had
  `Serialize` but no `Deserialize`, which left out the single most obvious use
  for a proof-of-validity wrapper: taking one as a request body. Decoding a
  `Validated<T>` and getting a value back is now the proof, so there is no
  `.validate()` call left to forget. An invalid payload fails at
  deserialization with the whole `garde::Report` rendered into the error;
  decode the plain `T` and call `Validated::new` where you need the structured
  report back for a 422 body.

- **`rubo4e::json::DEFAULT_MAX_NESTING_DEPTH`** — the always-on nesting cap (128)
  the docs referred to, now a public constant rather than a figure to copy.

- **`tests/json_strictness.rs`** — pins that every JSON entry point consumes its
  whole input and caps nesting, and that acceptance matches `serde_json` exactly.
  A reader that accepts what the reader in front of it rejects is the shape a
  validating proxy gets bypassed through.

- **A `Borrow<str>` contract guard over the whole identifier family** in
  `tests/prelude_surface.rs`: every identifier must be findable in a `HashMap`
  and a `BTreeMap` by the string it renders as. `ObisCode` was not.

- **`tests/generated_contract.rs` guards COM coverage too.** A COM missing its
  `Bo4eTyped` impl is invisible to a bound written over "anything with a `_typ`",
  and nothing else fails when one is absent — so the schemas are read and the
  correspondence checked in both directions, `ZusatzAttribut`'s absence included.

- **`tests/generated_contract.rs` guards the new constants and constructors.**
  `TYP_WIRE` is emitted as a literal because `Bo4eEnum::as_wire` is a trait
  method and cannot run in a const initializer — two spellings of one fact, which
  is exactly the shape that drifts, so both are pinned against the schema and
  against each other. A second test reads `required` out of every schema and
  asserts the type has a `new(...)` and no `Default`, or a `Default` and no
  required fields.

- **A guard over the examples' documented run commands.** Each example's
  `Run with:` header must name exactly the features its `[[example]]` entry
  requires. `examples/builder.rs` documented a command cargo refuses — it omitted
  `decimal`, which the example's own `rust_decimal` use needs — and nothing
  noticed, because the header is a comment and the manifest is data.

- **A drift guard over the `sqlx` impl list.** `impl_sqlx_text!` names its types
  by hand, and a new identifier missing from it compiles fine and simply cannot
  be a column. The list is now compared against the module's own exports, with
  the four helper enums named explicitly as non-identifiers.

- **`fuzz_deserialize_kosten`**, and `validate` in the fuzz feature set. Every BO
  target now runs three readers over the same bytes — `serde_json::from_slice`,
  the hardened German reader, and the hardened snake_case reader — and validates
  whatever decoded. The validators do `Decimal` arithmetic over wire values, and
  `rust_decimal` panics rather than erroring on several of its constructors, so a
  validator that aborts on a decodable payload is as exploitable as a
  deserializer that does.

### Removed

- **The `simd-json` feature.** It was measured, and on this crate's own types it
  was **slower than `serde_json` at every payload size tested** — 265 bytes to
  166 KB, 1.2× to 1.6× slower throughout.

  | Payload | `serde_json` | `simd-json` |
  |---|---|---|
  | 1.7 KB | 5.65 µs | 8.89 µs |
  | 16.7 KB | 55.7 µs | 75.6 µs |
  | 166 KB | 544 µs | 676 µs |

  The reason is structural rather than a tuning miss: every generated struct
  carries `#[serde(flatten)]` for its extension map, so deserialization is
  dominated by serde's `Content` buffering, not by the JSON tokenizer that SIMD
  accelerates. `simd-json`'s mutable-slice API then forces a `Vec<u8>` copy of
  every payload, and the depth guard needed a *second* full pass over the bytes
  because `simd-json` cannot wrap a visitor.

  The perf gate that should have caught this only asserted that `simd-json` was
  not more than 1.1–1.8× *slower* — it never asserted a speedup — and it had
  rotted into a broken state: it invoked the benchmark without the `time` and
  `decimal` features the target requires, so it failed before measuring
  anything. It is deleted along with the feature.

  Removing it also removes a second, weaker nesting-depth mechanism (a byte-scan
  pre-pass that existed only for the SIMD path), two magic size thresholds, a
  heavy optional dependency tree, two `cargo-deny` duplicate-version skips, a CI
  matrix entry, and a documented claim that was not true.

  **Action required:** drop `simd-json` from your feature list. Nothing else
  changes — the entry points, their signatures, and their behaviour are the same.


- **Name-based and suffix-based field typing.** `generator/src/inference.rs` had
  three tables: a suffix map, a bare-name map, and a per-struct override map. The
  suffix map's only *effective* action across the whole BO4E schema set was to
  mistype `Kontaktweg.kontaktwert` — every other suffix either never matched or
  matched a field the schema already typed the same way. The bare-name map caught
  homonyms in unrelated structs. The override map let a name beat an explicit
  `"format"`, and two of its four entries named fields v202607 does not have.

  All three are replaced by one exhaustive `(struct, field)` table, consulted only
  for properties the schema declares as a plain, unannotated `"string"`. A `$ref`,
  a `"format"`, and `"type": "number"` are now always authoritative.
- **`tests/module_coverage.rs`** — its live assertions were schema-file counts
  with a floor, and its generated-code assertions never executed. Replaced by
  `tests/generated_contract.rs`. The `no_v202402_*` tests, which guarded against a
  schema version that has never existed in this repository, are dropped outright.

### Documentation

- **The README and the ecosystem page destructured `billing_period()` as a
  tuple.** It has returned a `RangeInclusive<Date>` since the interval-convention
  fix earlier in this release, and `PreisblattNetznutzung::validity()` returns a
  pair rather than an `Option`. Nothing type-checked the convenience-API
  snippets, which is why they rotted; `tests/site_examples.rs` now compiles and
  runs them.
- **The README said `Zeitraum` requires `startdatum` strictly before
  `enddatum`.** It is `<=` — both bounds are inclusive, so `start == end` is a
  valid one-day period, exactly as the rest of the documentation and the code
  say.
- **The README's OpenAPI example quoted a `MaloId` pattern and example value
  neither schema emits.** The pattern is `^[1-9][0-9]{10}$` (the Vergabestelle
  digit is never `0`) and the example is the BDEW worked example `41373559241`.
- **`schema_helpers::bilanzkreis_id_schema` was documented as object type `'Z'`
  (Bilanzierungszone).** A Bilanzkreis is a market **party** — object type `'X'`,
  which is what the schema it emits has always said.
- **The `Kostenposition` rounding example computed `0.2843 × 3333` as
  `947.5119`.** It is `947.5719`, which is what the site already said; the two
  now agree.
- **The extension-map DoS claim was overstated.** `MAX_EXTENSION_FIELDS` bounds
  what a payload leaves *retained*, not what parsing it allocates:
  `#[serde(flatten)]` makes serde buffer a struct's unrecognised entries into an
  intermediate `Content` before the extension map's visitor ever runs.
  `max_payload_bytes` is the cap that bounds peak memory, and the docs now say so
  and say to set it first.
- **Extension data was described as round-tripping "byte-for-byte".** Names and
  values do, and top-level extension keys keep their arrival order — but key
  order *inside* a nested extension object does not, because `serde_json::Value`
  stores an object in a sorted map unless `preserve_order` is enabled somewhere
  in the build. Documented rather than quietly untrue.
- **The ecosystem page's proptest strategy computed a Luhn check digit and
  called it BDEW §8.1.** The two are different: §8.1 weights odd positions by 1
  and even positions by 2 and sums the products whole, while Luhn reduces any
  product ≥ 10 by 9. On the BDEW specification's own worked example the snippet
  produced `9` where the answer is `1`, so every ID it generated would have
  failed the `MaloId::new(&s).unwrap()` on the next line of the same sample. It
  now shows the pattern this crate's own property tests use: generate the base
  and let `from_base` derive the digit.
- **The `sqlx` integration list omitted `Iban` and `Bic`**, which have always had
  the impls. `tests/prelude_surface.rs` now compares the list against the
  module's exports so it cannot drift again.
- **The identifier trait surface is documented in full** — `TryFrom<String>`,
  `Into<String>`, `Borrow<str>`, and `Deref<Target = str>` were implemented but
  unlisted — along with the `Borrow` contract that makes `map.get("41373559241")`
  work without constructing and re-validating an identifier.
- **The architecture page's module tree listed five modules and omitted seven.**
  `strict`, `decimal_serde`, `offset_time`, `iso8601_duration`, `bank`,
  `sqlx_impls`, and the four files under `json/` are now in it, as is the
  generated `key_map.rs`.
- **`JsonParseLimits::max_nesting_depth` was documented as only ever lowering the
  cap.** That is true in effect, but for a reason worth stating: `serde_json`
  enforces its own recursion limit at the same 128 and parses first, so a higher
  value never takes effect. `DEFAULT_MAX_NESTING_DEPTH` is now public so the
  figure can be referenced rather than copied.
- **The generated `from_wire` doctest never ran.** Its positive assertion was
  emitted with a doubled `/// `, which made it a Rust line comment *inside* the
  code block — so every one of ~190 generated enums shipped an example that
  compiled and asserted nothing. Fixed; the assertions now execute.
- **`docs/generator.md` rewritten.** It documented an AST (`BoTypeDef`,
  `ComTypeDef`), an inference API (`infer_type` / `RustType`), a `_version`
  constant to hand-edit, and a `src/migration/` module — none of which exist. It
  now describes the real pipeline, the schema-driven metadata, the identifier
  naming rules, and the drift guards.
- **`docs/validation.md` corrected.** It described an "Avis Sum Constraint" over
  a type BO4E does not define, a `rabatt_brutto` field that does not exist, and a
  `#[garde(transparent)]` derive pattern the crate does not use. Every rule listed
  is now traceable to a sentence in the BO4E schema, with the reasoning for the
  one rule that is deliberately absent.
- **`docs/versioning.md` rewritten** around the contract above, with a
  within-series upgrade procedure — the common case, previously undocumented —
  alongside the format-version-cutover one.
- **`docs/architecture.md`** gained a "what a generated type gives you" table:
  the full derive and trait surface for structs and enums, and which feature each
  entry needs.
- **`docs/serialization.md`** documents the decimal string-vs-number asymmetry
  with a worked table, and what the hardening limits do *not* bound — a payload
  inside `max_payload_bytes` can still expand by two orders of magnitude, because
  `[{},{}…]` is three wire bytes and one struct per element.
- The pinned schema tag is no longer written out in the test suite: the helpers
  read it off the committed snapshot directory, so a within-series bump does not
  turn into a scavenger hunt through `tests/`.

### Schema deltas   (v202607.0.0 → v202607.1.0)

```
- Messgroesse    +3 (PHASENWINKEL, LEISTUNGSFAKTOR, FREQUENZ)   -1 (PREISE)
- Mengeneinheit  +2 (HZ, DIMENSIONSLOS)
- removed enums: Lokationstyp, Mengenoperator
```

`Messgroesse::Preise` and the two enums are **gone**, not deprecated. Both enums
were already unreferenced by every BO and COM in v202607.0.0, so the practical
impact is limited to code that named them directly. A SQL `CHECK` list or an
exhaustive mapping that still carries `PREISE` needs updating; `Messgroesse::COUNT`
and `Messgroesse::VARIANTS` turn that into a test failure rather than a runtime
surprise. `_version` now reads `202607.1.0`.

Library-side changes in the same release that behave like schema deltas:

- Several Rust **identifiers** for existing values changed — see *Changed
  (breaking)* above. Wire values are untouched, so stored JSON and SQL `CHECK`
  lists need no migration; only Rust source that names those variants does.
- Three fields changed Rust type from a domain newtype to `String`
  (`Kontaktweg.kontaktwert`, `MarktgebietInfo.marktgebiet`,
  `StandorteigenschaftenStrom.regelzone`). The schema always declared them as
  strings, so no stored payload changes; JSON Schema / OpenAPI output for them
  loses a numeric `pattern` it should never have carried.
- One stored value **does** need migration: `_version` written by 0.9.0 and
  earlier is `"v202607.0.0"`, which is not a valid BO4E version string.

## [0.9.0] — 2026-08-17

### Fixed *(breaking)*

- **`BilanzkreisId` required EIC object type `'Z'`, which no Bilanzkreis uses.**
  A Bilanzkreis is held by a Bilanzkreisverantwortlicher — a market participant —
  so its EIC carries object type **`'X'` (Party)**. ENTSO-E `'Z'` is a
  *measurement point*. The type therefore **rejected every Bilanzkreis-ID that
  exists**: `11XSUEDWESTSTRO8`, `11XENERGIE2----H`, and `11XENAGISME----J` are
  all real published codes, and all three failed to construct.

  `BilanzkreisId` now pins `'X'`, and the tests are pinned to those real codes
  rather than to a synthetic `11Z…` value that no registry would ever issue.

  **Action required:** any `11Z…` value your code constructed was not a
  Bilanzkreis-ID. Re-source the value from the ECS code registry.

- **`EicCode::domain()` classified EIC object types incorrectly.**
  It mapped `T`/`V` to "Party" and everything else to "Area". Per the ENTSO-E EIC
  Reference Manual, `X` is the party type, `Y` is area/domain, and `T`/`V` are
  *Tieline*/*Location*. Every market-participant code was reported as an area.

  `EicDomain` is **removed** and replaced by `EicType`, which carries all seven
  ENTSO-E object types (`Substation`, `Tieline`, `Location`, `ResourceObject`,
  `Party`, `Area`, `MeasurementPoint`) instead of collapsing them into two wrong
  buckets. Use `EicCode::eic_type()` in place of `EicCode::domain()`.

- **`AnyBo` discarded the deserializer it was given, causing silent data loss.**
  `AnyBo::deserialize` captured a `Box<RawValue>` and re-parsed it with
  `serde_json::from_str`, throwing away the wrapping deserializer. Two consequences:

  - `AnyBo::from_json_snake_case` **returned `Ok` with every typed field empty.**
    The snake_case → German key transform never ran, so values were diverted into
    `_additional` instead of their fields. A round-trip through `AnyBo` silently
    emptied the object rather than failing.
  - `from_json_*_hardened`'s `max_nesting_depth` was **not enforced** for `AnyBo`.
    A `RawValue` capture is one level deep as far as the depth limiter can see, so
    a configured limit silently did nothing on the polymorphic ingest path — the
    one that most often faces untrusted input.

  `AnyBo` now buffers through the caller's deserializer before dispatching on
  `"_typ"`, so both wrappers apply exactly as they do for a concrete BO type.
  This costs an intermediate buffer; deserialize the concrete type on hot paths.

- **`MaloId` check digits were computed with the wrong algorithm.** Releases
  through 0.8 used a Luhn-style variant (per-digit weights `2,1,2,1,…` with a
  `−9` reduction) instead of the *Lok- und Waggon-Kennzeichnungsverfahren* that
  BDEW §8.1 specifies. `MaloId::new` therefore **rejected virtually every real
  MaLo-ID** and accepted invalid ones. The tests generated their expectations
  with the same wrong function, which is why it went unnoticed.

  0.9 implements the specified algorithm, verified against the worked example in
  the BDEW document, the German Wikipedia article, the BO4E-python reference
  implementation, and 631 published Marktpartner-IDs (all 631 validate).

  **Action required:** MaLo-IDs your code previously stored as valid are almost
  certainly wrong. Recompute them with `MaloId::from_base(base)` and re-verify
  persisted data. `51238696780` is the old form of the fixture that appears
  throughout the previous docs and test data; `51238696781` is correct.

- **Hardened parse limits only applied to the root object.**
  `max_extension_value_bytes` and `max_extension_field_count` were checked after
  deserialization on the root's own `_additional` map, so extension data nested
  inside any COM escaped them entirely — a 50 KB payload hidden in
  `marktlokation.lokationsadresse` passed a 16-byte budget. Both limits are now
  enforced **during** parsing at **every** nesting level, which also makes
  rejection fail-fast instead of happening after the whole object tree has been
  allocated. Semantics are unchanged from what the fields always documented:
  `max_extension_value_bytes` is cumulative across the payload,
  `max_extension_field_count` is per struct.

- **`--features time` alone did not compile.** `time_serde` used `serde`
  unconditionally while being gated only on `time`; it is now gated on both.

- **`_version` was never populated, so Rust-built payloads were distinguishable
  from every other implementation's.** `Vertrag::default()` serialized to
  `{"_typ":"VERTRAG"}` where BO4E-python and go-bo4e both emit
  `{"_typ":"VERTRAG","_version":"v202607.0.0",…}` — including on nested COMs,
  which carry `_version` but no `_typ`. The docs told callers to set it
  explicitly, which meant hardcoding a version literal that silently goes stale
  on upgrade, even though the crate already knew the value statically via
  `Bo4eObject::schema_version()`.

  `_version` is now pre-filled on construction for **every** BO and COM — through
  `Default::default()`, the typed builder, and `..Default::default()` — matching
  the reference implementations. `_typ` remains BO-only, as before.

  Deserialization is unchanged and deliberately so: `_version` records the
  provenance of the data, so a payload arriving stamped `v202501.0.0` keeps that
  value, and one arriving without `_version` stays without one. Only construction
  fills it in; the setter is still available to re-stamp a value deliberately.

  Neither the golden corpus nor the compat vectors could catch this — both only
  round-trip existing JSON, which carries `_version` in from the input.
  `tests/compat.rs` now has an `outbound_tests` module covering the direction
  rubo4e *produces*.

  **Action required:** if you asserted on exact serialized output, those payloads
  now contain `_version`. If you were setting it manually, you can stop.

- **snake_case JSON silently moved fields into extension data.** The key
  transform derived the camelCase↔snake_case mapping with a heuristic, and a
  heuristic has no correct inverse: `hoechstpreis_ht` is an equally valid
  rendering of `hoechstpreisHt` and `hoechstpreisHT`, and `a` of both `a` and
  `A`. BO4E uses all of those shapes, so
  `from_json_snake_case(to_json_snake_case(x))` did not return `x` for
  `Tarifberechnungsparameter` (`hoechstpreisHT`, `hoechstpreisNT`),
  `PreisblattKonzessionsabgabe` (`kundengruppeKA`), and `Sigmoidparameter`
  (`A`, `B`, `C`, `D`). The values were not lost — they were deserialized into
  `_additional` instead of their typed field, so the round-trip looked
  successful while the typed accessors returned `None`.

  The generator now emits the exact bidirectional mapping
  (`src/generated/key_map.rs`) from the same field data it uses to emit the
  structs, so the round-trip is lossless by construction and cannot drift from
  the generated types. Lookups resolve to `&'static str`, so renaming a key now
  allocates on neither path.

  **Behaviour change:** keys the schema does not define — extension data — are
  no longer rewritten between modes. Previously `{"fooBAR": 1}` became
  `foo_bar` on the way out and `fooBar` on the way back in; it now passes
  through byte-for-byte in both directions. BO4E metadata keys (`_typ`,
  `_version`, `_id`) keep their leading underscore in every mode, as before.

  **Action required:** if you persisted snake_case JSON produced by 0.8 or
  earlier, extension keys in it carry heuristic-mangled names, and the four
  types above stored their affected fields under `_additional`. Re-serialize
  from the German wire format, which was never affected.

### Added

- **Four more BDEW identifier types**, completing the §8.2 ASCII-Verfahren
  family: `NebeId` (Netzbereich, Codetyp `F`), `CrId` (Cluster Ressource, `A`),
  `SgId` (Steuergruppe, `B`), and `PaketId` (Netzbetreiberwechsel, `P9`).
- **`AkivId`, `BilanzkreisId`, `TranchennummerId`** — Aktivierungsidentifikator
  (Redispatch 2.0), Bilanzkreis (EIC object type `X`), and MABIS Tranchennummer.
- **`EicType`** — all seven ENTSO-E EIC object types, with `as_char`,
  `from_char`, `description`, and an exhaustive `ALL` that is the single source of
  truth for which position-3 characters `EicCode` accepts.
- **`BilanzierungsgebietId`** — EIC pinned to object type `'Y'` (Area), the MaBiS
  Bilanzierungsgebiet counterpart to `BilanzkreisId`. Having both as distinct
  types means a balance group cannot be passed where a balancing area is
  expected. `StandorteigenschaftenStrom.bilanzierungsgebiet_eic` now generates as
  this type instead of `String`: the schema documents it as "Die EIC-Nummer des
  Bilanzierungsgebietes", and all 645 codes in the TSOs' published
  VNB-Bilanzierungsgebiete list carry object type `Y`.
- **`EicCode::new_from_prefix`** — builds a complete code from a 15-character
  prefix by computing the ENTSO-E check character.
- **`ObisCode::as_str`** — the canonical string, matching `as_ref`/`Display`.
- **Shared API across every §8.2 identifier**: `from_base()` computes and appends
  the check digit, `check_digit()` returns it without constructing the value,
  `base()` returns the 10-character body, and `CODETYP` exposes the fixed prefix.
- **`MaloId::vergabestelle()`** returning `MaloVergabestelle`, and
  **`MarktpartnerId::authority()`** returning `MpIdAuthority`, plus
  `nad_agency_code()` / `unb_agency_code()` for EDIFACT NAD DE3055 and UNB DE0007.
- **Opt-in MP-ID check-digit verification** — `MarktpartnerId::new_checked`,
  `has_valid_bdew_check_digit`, `has_valid_gln_check_digit`. Construction still
  does not enforce a check digit, because an MP-ID may carry either the BDEW
  (§8.1) or the GS1/EAN-13 procedure and the leading digits do not reliably
  separate them.
- **`sqlx::postgres::PgHasArrayType` for every identifier**, so `Vec<Id>` binds
  to a `TEXT[]` column. This has to live in this crate: both the trait and the
  types are foreign to any consumer, so the orphan rule rules out a local impl.
- **`prelude::Validate`** — the `garde` trait that provides `.validate()` is now
  re-exported, so callers no longer need a direct `garde` dependency to use the
  `validate` feature.
- **Documentation site** at <https://hupe1980.github.io/rubo4e>, built from
  `site/` with Zola.

### Changed *(breaking)*

- `NeloId`, `SrId`, and `TrId` moved into a shared `ascii_ids` module with the
  other §8.2 identifiers. The public paths (`rubo4e::identifiers::NeloId`, …) are
  unchanged; only the internal module layout moved.
- `MaloId` now enforces the leading Vergabestelle digit (`1`–`9`) per §3.2.
- **`sqlx` no longer implies `json`.** Identifier and enum SQL impls both
  round-trip through plain `&str` (`as_ref` / `as_wire` / `from_wire`), so
  `serde_json` is no longer pulled in. Enum `Decode` also stops allocating a
  `serde_json::Value` per row.
- The `*_hardened` methods on `Bo4eJsonExt` dropped their
  `where Self: Bo4eExtensionData` bound, which the parse-time budget made
  unnecessary.
- **MSRV raised from 1.87 to 1.88.** Not a source change — `time`, `simd-json`,
  and `home` (reached through `sqlx`) all now require 1.88, so the declared
  `rust-version` was no longer achievable and CI's MSRV job failed at dependency
  *resolution*, before compiling anything. The crate's own source still builds on
  1.87; only the dependency tree does not. `garde` is no longer the binding
  constraint, and the feature table no longer lists an MSRV impact for `validate`.
- **`deny.toml` records six known-safe duplicate-version splits.** All are
  transitive splits between upstream crates that each pin their own major
  (`windows-sys`, `hashbrown` ×2, `foldhash`, `redox_syscall`, `syn`), reachable
  only through the optional `sqlx` / `simd-json` features and dev-only tooling.
  None can be collapsed from this crate. Skips are pinned at minor-version
  granularity so a patch bump upstream does not silently re-break the gate.
- **`just deny-check` now runs `--all-features`**, matching what
  `cargo-deny-action` does in CI. The recipe previously checked only the default
  feature set, which left every optional dependency out of the graph — a
  duplicate-version ban that CI rejected passed cleanly on the same tree locally.
- **`ObisCode` stores a canonical form and its parsed value groups.**
  Previously the input string was stored verbatim apart from `&`→`*`, and
  `components()` re-parsed it — allocating on every call and carrying an
  `expect` that a stored value was still parseable. Now the value is parsed once
  at construction, so `components()` is infallible and free.

  Canonicalisation also drops redundant leading zeros, which makes equality
  semantic: `01-00:01.08.00` and `1-0:1.8.0` are now equal and hash alike, where
  before they were distinct. `as_ref`, `Display`, and `serde` all emit the
  canonical form, so a value may not round-trip byte-for-byte to its input.

  `ObisCode::to_bo4e_string()` is **removed** — the canonical form is what the
  type stores, so `as_str()` (or `as_ref`/`Display`) returns it without
  allocating. `to_pia_string()` is unchanged.
- **OBIS value groups are `u8` rather than `u32`.** IEC 62056-61 §4 defines each
  of A–F as a single octet, so `ObisComponents` fields are now `u8`/`Option<u8>`
  and a group above 255 is rejected with an error naming the offending group.
- **Uniform trait surface across all identifiers.** `EicCode`, `BilanzkreisId`,
  and `ObisCode` hand-rolled a subset of the conversions the macro-generated
  identifiers already had. All identifiers now share one implementation, so
  `Deref<Target = str>`, `Borrow<str>`, and `From<T> for String` are available on
  every one of them — previously `String::from(malo_id)` compiled but
  `String::from(eic_code)` did not.
- `ObisCode` now carries a real `schemars`/`utoipa` schema (grammar pattern,
  description, examples) instead of a bare `String`.

### Removed

- **`src/identifiers/proptest_impls.rs`** — 224 lines of `#[cfg(test)]`
  `Arbitrary` impls with no callers. The integration suite has its own strategy
  table, and the unused copy had silently rotted: its `EicCode` generator placed
  the object-type character at position 1 instead of 3 (so most draws were
  discarded), and its `BilanzkreisId` generator produced `'Z'` codes. The
  integration table now also covers `BilanzkreisId` and `BilanzierungsgebietId`.
- **`json::peek_typ_field`** — an internal helper that existed only for the
  `AnyBo` raw-capture path removed above.

### Documentation

- Every code example in the crate is now compiled and run: the 205 `rust,ignore`
  doctest blocks are gone, and the generated per-enum examples carry real
  assertions including a wire→variant mapping taken from the schema. Doctests
  went from 37 passing / 206 ignored to 242 passing / 0 ignored.
- CI gained a 24-configuration feature matrix, MSRV verification, generated-code
  drift detection, and rustdoc broken-link denial.

### Schema deltas

- Schema version unchanged: **v202607.0.0**. No enum membership or codelist
  changes in this release.
- Upstream **v202607.1.0** remains un-adopted; see the 0.8.0 notes below for the
  review. Regenerating against it is still a separate follow-up.

## [0.8.0] — 2026-07-26

This release adds a uniform, feature-independent introspection and strict-parsing
surface to every generated BO4E enum, in response to downstream feedback from the
`mako` project.

### Added

- **`Bo4eStrict` trait + `strict` module** (`versioned` feature) — recursive
  strict decoding. `value.ensure_known_enums()` walks a decoded BO/COM/`AnyBo`
  and returns `Err(strict::StrictError)` listing the JSON-path of **every** enum
  field that fell through to `Unknown`, anywhere in the tree (e.g.
  `["zaehler[1].zaehlertyp"]`). `unknown_enum_paths()` returns them directly.
  One call replaces the hand-written `record.field == T::Unknown` re-checks a
  strict ingest boundary needs. Implemented for every generated BO, COM, enum,
  and `AnyBo`. **Not sealed** — downstream wrappers can implement it to extend
  the recursive check. Re-exported from `rubo4e::prelude`.
- **`Bo4eEnum` trait** (`versioned` feature) — implemented by every generated
  BO4E enum, giving a uniform surface for code that is generic over the enum type
  (e.g. proving a SQL `CHECK` list covers `T::VARIANTS`). Re-exported from
  `rubo4e::prelude`. Sealed — cannot be implemented downstream.
- **Feature-independent `Display` + `AsRef<str>` on every enum**, yielding the
  canonical BO4E wire string via `as_wire`. Previously these required `strum`.
- **Per-enum introspection, available WITHOUT the `strum` feature.** Every enum
  now exposes:
  - `const VARIANTS: &'static [Self]` — the known variants, excluding the
    `Unknown` catch-all, in schema declaration order.
  - `const COUNT: usize` — a stable per-version variant count. Replaces the
    hand-maintained magic-number guards downstream projects had to pin.
  - `fn iter_known() -> impl Iterator<Item = Self> + Clone` — previously gated on
    `strum`, now always available.
- **Strict enum parsing.** Every enum now exposes:
  - `fn from_wire(s: &str) -> Result<Self, UnknownVariant>` — the opt-in strict
    counterpart to the lenient `serde` / `FromStr` path. Returns `Err` for typos,
    legacy codes, and values from a newer schema (including the literal
    `"UNKNOWN"`) instead of silently mapping them to `Unknown`.
  - `fn as_wire(&self) -> &'static str` — the canonical BO4E wire string.
  - `const fn is_known(&self) -> bool` / `const fn is_unknown(&self) -> bool` —
    detect a value that fell through to `Unknown` after a lenient decode, in one
    call at the ingest boundary.
- **`error::UnknownVariant`** — the error returned by `from_wire`, with the
  offending value. Converts into `garde::Error` under the `validate` feature.
  Re-exported from `rubo4e::prelude`.
- **Type- and variant-level interop documentation** generated directly into the
  affected enums:
  - `Zaehlertyp::IntelligentesMesssystem` and `Geraetetyp::IntelligentesMessystem`
    now carry cross-referencing notes about the upstream `Messsystem`/`Messystem`
    spelling divergence.
  - `BdewArtikelnummer` documents its provenance and coverage signal.
  - `Gasqualitaet` documents the H2-blend forward-compatibility story.
  - `Rechnungstyp` documents the sanctioned representation for correction/reversal
    invoices.

### Changed *(breaking)*

- Enum `Display` and `AsRef<str>` are now hand-written (always on) rather than
  derived from `strum`. Behaviour is identical (canonical wire string); the
  `strum` derive set is reduced to `EnumString`, `EnumIter`, `IntoStaticStr`.
- The sqlx `Encode` impl for enums no longer has a separate `strum` fast-path —
  it always encodes via `as_wire()` (one fewer allocation, no `strum` needed).
- The proptest `Arbitrary` impls for generated enums no longer require the
  `strum` feature (they now sample from `VARIANTS`). This only affects the
  crate's own `#[cfg(test)]` builds.

### Notes / no-ops

- **Reactive-energy units already present.** `Mengeneinheit` already includes
  `Kvarh`, `Kvar`, `Var`, and `Varh` in v202607 — no change needed. Downstream
  code mapping `KVARH → KWH` / `KVAR → KW` can map directly to the reactive
  variants instead.
- **Mandatory-vs-optional fields.** AHB-mandatory status is a
  message/process-context property and is *not* present in the BO4E JSON Schema
  (`bo/*.json` carry no `required` array beyond `_typ`), so it cannot be derived
  here. For ergonomic, diffable construction, enable the `builder` feature
  (`typed-builder`, `setter(into)`); see `examples/builder.rs`.

### Schema deltas

- Schema version unchanged: **v202607.0.0**. No enum membership or codelist
  changes in this release — all changes above are library API additions.
- Reviewed upstream **v202607.1.0** (BO4E-Schemas): it removes two *unreferenced*
  enums (`Mengenoperator`, `Lokationstyp`), trims `Messgroesse`, and remodels
  `Zeitreihe`; it does **not** touch `Gasqualitaet`, `Zaehlertyp`, `Geraetetyp`,
  `Mengeneinheit`, `Rechnungstyp`, or `BdewArtikelnummer`. All the enum
  observations above therefore still hold against the newest upstream patch.
  Regenerating against v202607.1.0 is tracked as a separate follow-up.

---

Older history (0.1.0 – 0.7.0) is available in the git log. Notable prior
milestones: identifier types with BDEW check digits (`MaloId`, `MeloId`, `SrId`,
`TrId`, `AkivId`, `BilanzkreisId`, `TranchennummerId`, …), the `versioned` schema
modules, the `time`-crate version-bound relaxation in 0.7.0, and the
`decimal` / `time` typed-field backends.
