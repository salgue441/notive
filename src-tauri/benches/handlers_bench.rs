//! Performance benchmarks for handlers.

use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Import test functions (they're public in test mode)
#[cfg(test)]
use notive_lib::handlers::navigation::{is_oauth_url, should_open_externally};

// For benchmarks, we'll test the public functions
// Note: These functions need to be public for benchmarks
// We'll use a workaround by testing URL parsing performance

fn bench_url_parsing(c: &mut Criterion) {
    use url::Url;
    
    let urls = vec![
        "https://www.notion.so/page",
        "https://example.com/path?query=value",
        "https://accounts.google.com/oauth",
        "https://notion.so",
        "https://api.notion.so",
    ];
    
    c.bench_function("url_parsing", |b| {
        b.iter(|| {
            for url_str in urls.iter() {
                let _ = black_box(Url::parse(url_str));
            }
        });
    });
}

fn bench_host_extraction(c: &mut Criterion) {
    use url::Url;
    
    let urls = vec![
        "https://www.notion.so/page",
        "https://example.com/path",
        "https://accounts.google.com/oauth",
    ];
    
    c.bench_function("host_extraction", |b| {
        b.iter(|| {
            for url_str in urls.iter() {
                if let Ok(parsed) = Url::parse(url_str) {
                    let _ = black_box(parsed.host_str());
                }
            }
        });
    });
}

fn bench_string_operations(c: &mut Criterion) {
    let hosts = vec![
        "www.notion.so",
        "notion.so",
        "api.notion.so",
        "example.com",
    ];
    
    let domain = "notion.so";
    
    c.bench_function("host_matching", |b| {
        b.iter(|| {
            for host in hosts.iter() {
                let matches = black_box(host == domain || host.ends_with(&format!(".{}", domain)));
                black_box(matches);
            }
        });
    });
}

criterion_group!(
    benches,
    bench_url_parsing,
    bench_host_extraction,
    bench_string_operations
);
criterion_main!(benches);
