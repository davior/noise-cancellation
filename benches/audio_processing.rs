use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_audio_processing(c: &mut Criterion) {
    c.bench_function("process_256_samples", |b| {
        b.iter(|| {
            let samples = black_box(vec![0.1f32; 256]);
            // Simulate DSP processing
            let _ = samples.iter().map(|s| s * 0.95).collect::<Vec<_>>();
        })
    });
}

criterion_group!(benches, bench_audio_processing);
criterion_main!(benches);
