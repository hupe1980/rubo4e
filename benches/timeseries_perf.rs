//! What a coverage audit costs at production volume.
//!
//! A year of quarter-hourly readings is 35 040 entries, and a Netzbetreiber
//! runs one of those per metering point per settlement run. The audit is a
//! `O(n log n)` sort over one `Vec<Range<OffsetDateTime>>`, so the numbers here
//! are the claim in the docs made checkable — and the shapes are the ones that
//! actually arrive: clean, gappy, duplicated, and unsorted.
//!
//! ```text
//! cargo bench --bench timeseries_perf --features versioned,time,decimal
//! ```

use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use rubo4e::current::{Lastgang, Menge, Mengeneinheit, Messwertstatus, Zeitraum, Zeitreihenwert};
use rubo4e::timeseries::Bo4eTimeSeries;
use rust_decimal::Decimal;
use std::hint::black_box;
use time::{Duration, OffsetDateTime};

/// 2026-01-01 00:00 +01:00 — the start of a German settlement year.
fn origin() -> OffsetDateTime {
    time::macros::datetime!(2026-01-01 00:00 +01:00)
}

fn slot(start: OffsetDateTime, minutes: i64, kw: i64) -> Zeitreihenwert {
    Zeitreihenwert {
        wert: Some(Decimal::from(kw)),
        status: Some(Messwertstatus::Abgelesen),
        zeitraum: Some(Zeitraum::from_instants(
            start,
            start + Duration::minutes(minutes),
        )),
        ..Default::default()
    }
}

fn lastgang(werte: Vec<Zeitreihenwert>) -> Lastgang {
    Lastgang {
        messgroesse: Some(Mengeneinheit::Kw),
        werte: Some(werte),
        ..Lastgang::new(Menge {
            wert: Some(Decimal::from(15)),
            einheit: Some(Mengeneinheit::Minute),
            ..Default::default()
        })
    }
}

/// A clean quarter-hourly profile of `n` slots — the best case, and the one a
/// conforming producer sends.
fn clean(n: i64) -> Lastgang {
    let t0 = origin();
    lastgang(
        (0..n)
            .map(|i| slot(t0 + Duration::minutes(15 * i), 15, 400 + i % 200))
            .collect(),
    )
}

/// Every twelfth slot missing — a communication outage of three hours a day.
fn gappy(n: i64) -> Lastgang {
    let t0 = origin();
    lastgang(
        (0..n)
            .filter(|i| i % 12 != 0)
            .map(|i| slot(t0 + Duration::minutes(15 * i), 15, 400 + i % 200))
            .collect(),
    )
}

/// Every slot delivered twice — the shape that silently doubles a billed
/// quantity, and the reason the sweep tracks overlaps at all.
fn duplicated(n: i64) -> Lastgang {
    let t0 = origin();
    let mut werte = Vec::with_capacity(n as usize * 2);
    for i in 0..n {
        let s = slot(t0 + Duration::minutes(15 * i), 15, 400 + i % 200);
        werte.push(s.clone());
        werte.push(s);
    }
    lastgang(werte)
}

/// Descending order — the worst case for the sort, and legal: the schema
/// imposes no order on `werte`.
fn reversed(n: i64) -> Lastgang {
    let t0 = origin();
    lastgang(
        (0..n)
            .rev()
            .map(|i| slot(t0 + Duration::minutes(15 * i), 15, 400 + i % 200))
            .collect(),
    )
}

/// `audit` across the shapes and the sizes that occur: a day, a month, a year.
fn audit(c: &mut Criterion) {
    let mut group = c.benchmark_group("timeseries/audit");
    for (shape, build) in [
        ("clean", clean as fn(i64) -> Lastgang),
        ("gappy", gappy),
        ("duplicated", duplicated),
        ("reversed", reversed),
    ] {
        for slots in [96_i64, 2_976, 35_040] {
            let lg = build(slots);
            group.throughput(Throughput::Elements(lg.werte().len() as u64));
            group.bench_with_input(BenchmarkId::new(shape, slots), &lg, |b, lg| {
                b.iter(|| black_box(lg.audit()))
            });
        }
    }
    group.finish();
}

/// `audit_over` adds the clipping pass, and is what a settlement run actually
/// calls — the period is known, and a series missing its last day must not read
/// as complete.
fn audit_over(c: &mut Criterion) {
    let mut group = c.benchmark_group("timeseries/audit_over");
    let year = origin()..origin() + Duration::days(365);
    for slots in [96_i64, 2_976, 35_040] {
        let lg = clean(slots);
        group.throughput(Throughput::Elements(slots as u64));
        group.bench_with_input(BenchmarkId::from_parameter(slots), &lg, |b, lg| {
            b.iter(|| black_box(lg.audit_over(year.clone())))
        });
    }
    group.finish();
}

/// The two aggregates. `integrate` parses every interval a second time and runs
/// `Decimal` arithmetic per slot; `sum` touches neither, which is the cost
/// difference between billing a load profile and billing a meter reading.
fn aggregate(c: &mut Criterion) {
    let mut group = c.benchmark_group("timeseries/aggregate");
    for slots in [96_i64, 2_976, 35_040] {
        let lg = clean(slots);
        group.throughput(Throughput::Elements(slots as u64));
        group.bench_with_input(BenchmarkId::new("integrate", slots), &lg, |b, lg| {
            b.iter(|| black_box(lg.integrate()))
        });
        group.bench_with_input(BenchmarkId::new("sum", slots), &lg, |b, lg| {
            b.iter(|| black_box(lg.sum()))
        });
    }
    group.finish();
}

/// Building the series, so the audit numbers above can be read against the cost
/// of the allocation they are measured on.
fn construct(c: &mut Criterion) {
    let mut group = c.benchmark_group("timeseries/construct");
    for slots in [96_i64, 2_976, 35_040] {
        group.throughput(Throughput::Elements(slots as u64));
        group.bench_with_input(BenchmarkId::from_parameter(slots), &slots, |b, &slots| {
            b.iter_batched(|| slots, |n| black_box(clean(n)), BatchSize::SmallInput)
        });
    }
    group.finish();
}

criterion_group!(timeseries_perf, audit, audit_over, aggregate, construct);
criterion_main!(timeseries_perf);
