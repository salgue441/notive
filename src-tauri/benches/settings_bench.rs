//! Performance benchmarks for settings operations.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use notive_lib::config::{ShortcutSettings, UpdateChannel, UserSettings};
use serde_json;

fn bench_settings_load_save_cycle(c: &mut Criterion) {
    let settings = UserSettings::default();
    let json = serde_json::to_string(&settings).unwrap();
    
    c.bench_function("settings_load_save_cycle", |b| {
        b.iter(|| {
            let loaded: UserSettings = serde_json::from_str(black_box(&json)).unwrap();
            let saved = serde_json::to_string(black_box(&loaded)).unwrap();
            black_box(saved)
        });
    });
}

fn bench_settings_with_large_css(c: &mut Criterion) {
    let mut settings = UserSettings::default();
    // Simulate large custom CSS (10KB)
    settings.custom_css = "body { color: red; }\n".repeat(500);
    settings.custom_css_enabled = true;
    
    c.bench_function("settings_with_large_css_serialize", |b| {
        b.iter(|| {
            serde_json::to_string(black_box(&settings)).unwrap()
        });
    });
    
    let json = serde_json::to_string(&settings).unwrap();
    c.bench_function("settings_with_large_css_deserialize", |b| {
        b.iter(|| {
            serde_json::from_str::<UserSettings>(black_box(&json)).unwrap()
        });
    });
}

fn bench_settings_comparison(c: &mut Criterion) {
    let settings1 = UserSettings::default();
    let settings2 = UserSettings::default();
    let mut settings3 = UserSettings::default();
    settings3.zoom_level = 1.5;
    
    c.bench_function("settings_comparison_equal", |b| {
        b.iter(|| {
            black_box(&settings1).zoom_level == black_box(&settings2).zoom_level
        });
    });
    
    c.bench_function("settings_comparison_different", |b| {
        b.iter(|| {
            black_box(&settings1).zoom_level != black_box(&settings3).zoom_level
        });
    });
}

fn bench_shortcut_settings_variations(c: &mut Criterion) {
    let mut group = c.benchmark_group("shortcut_settings");
    
    for count in [1, 10, 100].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(count),
            count,
            |b, &count| {
                let shortcuts: Vec<ShortcutSettings> = (0..count)
                    .map(|i| {
                        let mut s = ShortcutSettings::default();
                        s.toggle_window = format!("Alt+{}", i);
                        s
                    })
                    .collect();
                
                b.iter(|| {
                    for shortcut in shortcuts.iter() {
                        black_box(serde_json::to_string(shortcut).unwrap());
                    }
                });
            },
        );
    }
    group.finish();
}

fn bench_update_channel_operations(c: &mut Criterion) {
    let channels = vec![
        UpdateChannel::Stable,
        UpdateChannel::Beta,
        UpdateChannel::Nightly,
    ];
    
    c.bench_function("update_channel_serialize_all", |b| {
        b.iter(|| {
            for channel in channels.iter() {
                black_box(serde_json::to_string(channel).unwrap());
            }
        });
    });
}

criterion_group!(
    benches,
    bench_settings_load_save_cycle,
    bench_settings_with_large_css,
    bench_settings_comparison,
    bench_shortcut_settings_variations,
    bench_update_channel_operations
);
criterion_main!(benches);
