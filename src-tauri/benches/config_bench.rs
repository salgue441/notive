//! Performance benchmarks for configuration module.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use notive_lib::config::{ShortcutSettings, UpdateChannel, UserSettings};
use serde_json;

fn bench_settings_serialize(c: &mut Criterion) {
    let settings = UserSettings::default();
    
    c.bench_function("settings_serialize", |b| {
        b.iter(|| {
            serde_json::to_string(black_box(&settings)).unwrap()
        });
    });
}

fn bench_settings_deserialize(c: &mut Criterion) {
    let settings = UserSettings::default();
    let json = serde_json::to_string(&settings).unwrap();
    
    c.bench_function("settings_deserialize", |b| {
        b.iter(|| {
            serde_json::from_str::<UserSettings>(black_box(&json)).unwrap()
        });
    });
}

fn bench_shortcut_settings_equality(c: &mut Criterion) {
    let shortcuts1 = ShortcutSettings::default();
    let shortcuts2 = ShortcutSettings::default();
    
    c.bench_function("shortcut_settings_equality", |b| {
        b.iter(|| {
            black_box(&shortcuts1) == black_box(&shortcuts2)
        });
    });
}

fn bench_update_channel_serialize(c: &mut Criterion) {
    let channel = UpdateChannel::Stable;
    
    c.bench_function("update_channel_serialize", |b| {
        b.iter(|| {
            serde_json::to_string(black_box(&channel)).unwrap()
        });
    });
}

criterion_group!(
    benches,
    bench_settings_serialize,
    bench_settings_deserialize,
    bench_shortcut_settings_equality,
    bench_update_channel_serialize
);
criterion_main!(benches);
