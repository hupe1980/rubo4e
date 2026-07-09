use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use rubo4e::{
    json::Bo4eJsonExt,
    v202607::{Marktlokation, Vertrag},
};
use serde_json::Value;
use std::hint::black_box;

const MARKTLOKATION_MIN: &str = include_str!("../tests/golden/marktlokation_minimal.json");
const MARKTLOKATION_TYP: &str = include_str!("../tests/golden/marktlokation_typical.json");
const MARKTLOKATION_COMPAT_GO: &str = include_str!("../tests/compat/go/marktlokation.json");
const MARKTLOKATION_FILEBACKED_LARGE: &str =
    include_str!("../tests/perf/marktlokation_filebacked_large.json");
const VERTRAG_MIN: &str = include_str!("../tests/golden/vertrag_minimal.json");
const VERTRAG_TYP: &str = include_str!("../tests/golden/vertrag_typical.json");
const VERTRAG_COMPAT_GO: &str = include_str!("../tests/compat/go/vertrag.json");
const VERTRAG_FILEBACKED_LARGE: &str = include_str!("../tests/perf/vertrag_filebacked_large.json");

fn with_adversarial_extension_payload(base: &str) -> String {
    let mut value: Value = serde_json::from_str(base).expect("valid golden payload");
    let root = value
        .as_object_mut()
        .expect("golden payload root must be a JSON object");

    // Add many unknown extension keys with nested content to model larger,
    // high-entropy real-world payloads while staying below extension field cap.
    for idx in 0..48 {
        let key = format!("perf_adversarial_field_{idx:02}");
        let extension = serde_json::json!({
            "index": idx,
            "labels": ["a", "b", "c", "d", "e", "f", "g", "h"],
            "payload": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "nested": {
                "left": {"v": idx, "ok": true},
                "right": [{"k":"x"}, {"k":"y"}, {"k":"z"}]
            }
        });
        root.insert(key, extension);
    }

    serde_json::to_string(&value).expect("adversarial payload must serialize")
}

fn bench_deser_german<T: Bo4eJsonExt>(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    name: &str,
    payload: &str,
) {
    group.throughput(Throughput::Bytes(payload.len() as u64));

    group.bench_with_input(BenchmarkId::new("str", name), &payload, |b, p| {
        b.iter(|| {
            let parsed = T::from_json_german(black_box(p));
            black_box(parsed).expect("payload should deserialize");
        });
    });

    let raw = payload.as_bytes().to_vec();
    group.bench_with_input(BenchmarkId::new("bytes", name), &raw, |b, p| {
        b.iter_batched(
            || p.clone(),
            |bytes| {
                let parsed = T::from_json_german_bytes(black_box(bytes.as_slice()));
                black_box(parsed).expect("payload should deserialize");
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_deser_snake<T: Bo4eJsonExt>(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    name: &str,
    payload: &str,
) {
    group.throughput(Throughput::Bytes(payload.len() as u64));

    group.bench_with_input(BenchmarkId::new("str", name), &payload, |b, p| {
        b.iter(|| {
            let parsed = T::from_json_snake_case(black_box(p));
            black_box(parsed).expect("payload should deserialize");
        });
    });

    let raw = payload.as_bytes().to_vec();
    group.bench_with_input(BenchmarkId::new("bytes", name), &raw, |b, p| {
        b.iter_batched(
            || p.clone(),
            |bytes| {
                let parsed = T::from_json_snake_case_bytes(black_box(bytes.as_slice()));
                black_box(parsed).expect("payload should deserialize");
            },
            BatchSize::SmallInput,
        );
    });
}

fn deser_german(c: &mut Criterion) {
    let mut group = c.benchmark_group("deserialize/from_json_german");
    let ml_adv = with_adversarial_extension_payload(MARKTLOKATION_TYP);
    let vt_adv = with_adversarial_extension_payload(VERTRAG_TYP);

    bench_deser_german::<Marktlokation>(&mut group, "marktlokation_minimal", MARKTLOKATION_MIN);
    bench_deser_german::<Marktlokation>(&mut group, "marktlokation_typical", MARKTLOKATION_TYP);
    bench_deser_german::<Marktlokation>(
        &mut group,
        "marktlokation_filebacked",
        MARKTLOKATION_COMPAT_GO,
    );
    bench_deser_german::<Marktlokation>(
        &mut group,
        "marktlokation_filebacked_large",
        MARKTLOKATION_FILEBACKED_LARGE,
    );
    bench_deser_german::<Marktlokation>(&mut group, "marktlokation_adversarial", &ml_adv);
    bench_deser_german::<Vertrag>(&mut group, "vertrag_minimal", VERTRAG_MIN);
    bench_deser_german::<Vertrag>(&mut group, "vertrag_typical", VERTRAG_TYP);
    bench_deser_german::<Vertrag>(&mut group, "vertrag_filebacked", VERTRAG_COMPAT_GO);
    bench_deser_german::<Vertrag>(
        &mut group,
        "vertrag_filebacked_large",
        VERTRAG_FILEBACKED_LARGE,
    );
    bench_deser_german::<Vertrag>(&mut group, "vertrag_adversarial", &vt_adv);
    group.finish();
}

fn deser_snake(c: &mut Criterion) {
    let mut group = c.benchmark_group("deserialize/from_json_snake_case");

    let ml = Marktlokation::from_json_german(MARKTLOKATION_TYP).expect("valid golden payload");
    let vt = Vertrag::from_json_german(VERTRAG_TYP).expect("valid golden payload");
    let ml_filebacked = Marktlokation::from_json_german(MARKTLOKATION_COMPAT_GO)
        .expect("valid file-backed payload");
    let ml_filebacked_large = Marktlokation::from_json_german(MARKTLOKATION_FILEBACKED_LARGE)
        .expect("valid large file-backed payload");
    let vt_filebacked =
        Vertrag::from_json_german(VERTRAG_COMPAT_GO).expect("valid file-backed payload");
    let vt_filebacked_large = Vertrag::from_json_german(VERTRAG_FILEBACKED_LARGE)
        .expect("valid large file-backed payload");
    let ml_adv =
        Marktlokation::from_json_german(&with_adversarial_extension_payload(MARKTLOKATION_TYP))
            .expect("valid adversarial payload");
    let vt_adv = Vertrag::from_json_german(&with_adversarial_extension_payload(VERTRAG_TYP))
        .expect("valid adversarial payload");
    let ml_snake = ml.to_json_snake_case().expect("snake json generation");
    let vt_snake = vt.to_json_snake_case().expect("snake json generation");
    let ml_filebacked_snake = ml_filebacked
        .to_json_snake_case()
        .expect("file-backed snake json generation");
    let ml_filebacked_large_snake = ml_filebacked_large
        .to_json_snake_case()
        .expect("large file-backed snake json generation");
    let vt_filebacked_snake = vt_filebacked
        .to_json_snake_case()
        .expect("file-backed snake json generation");
    let vt_filebacked_large_snake = vt_filebacked_large
        .to_json_snake_case()
        .expect("large file-backed snake json generation");
    let ml_adv_snake = ml_adv
        .to_json_snake_case()
        .expect("adversarial snake json generation");
    let vt_adv_snake = vt_adv
        .to_json_snake_case()
        .expect("adversarial snake json generation");

    bench_deser_snake::<Marktlokation>(&mut group, "marktlokation_typical", &ml_snake);
    bench_deser_snake::<Marktlokation>(
        &mut group,
        "marktlokation_filebacked",
        &ml_filebacked_snake,
    );
    bench_deser_snake::<Marktlokation>(
        &mut group,
        "marktlokation_filebacked_large",
        &ml_filebacked_large_snake,
    );
    bench_deser_snake::<Marktlokation>(&mut group, "marktlokation_adversarial", &ml_adv_snake);
    bench_deser_snake::<Vertrag>(&mut group, "vertrag_typical", &vt_snake);
    bench_deser_snake::<Vertrag>(&mut group, "vertrag_filebacked", &vt_filebacked_snake);
    bench_deser_snake::<Vertrag>(
        &mut group,
        "vertrag_filebacked_large",
        &vt_filebacked_large_snake,
    );
    bench_deser_snake::<Vertrag>(&mut group, "vertrag_adversarial", &vt_adv_snake);
    group.finish();
}

fn ser(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialize");

    let ml = Marktlokation::from_json_german(MARKTLOKATION_TYP).expect("valid golden payload");
    group.throughput(Throughput::Bytes(MARKTLOKATION_TYP.len() as u64));
    group.bench_function("to_json_german/marktlokation_typical", |b| {
        b.iter(|| black_box(ml.to_json_german()).expect("serialization should succeed"));
    });
    group.bench_function("to_json_snake_case/marktlokation_typical", |b| {
        b.iter(|| black_box(ml.to_json_snake_case()).expect("serialization should succeed"));
    });
    group.bench_function("to_json_canonical/marktlokation_typical", |b| {
        b.iter(|| black_box(ml.to_json_canonical()).expect("serialization should succeed"));
    });

    let vt = Vertrag::from_json_german(VERTRAG_TYP).expect("valid golden payload");
    group.throughput(Throughput::Bytes(VERTRAG_TYP.len() as u64));
    group.bench_function("to_json_german/vertrag_typical", |b| {
        b.iter(|| black_box(vt.to_json_german()).expect("serialization should succeed"));
    });
    group.bench_function("to_json_snake_case/vertrag_typical", |b| {
        b.iter(|| black_box(vt.to_json_snake_case()).expect("serialization should succeed"));
    });
    group.bench_function("to_json_canonical/vertrag_typical", |b| {
        b.iter(|| black_box(vt.to_json_canonical()).expect("serialization should succeed"));
    });

    group.finish();
}

criterion_group!(json_perf, deser_german, deser_snake, ser);
criterion_main!(json_perf);
