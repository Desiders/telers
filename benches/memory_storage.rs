use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use telers::fsm::{
    storage::{Storage, StorageKey},
    MemoryStorage,
};

fn memory_storage_benchmark(c: &mut Criterion) {
    c.bench_function("create", |b| b.iter(|| MemoryStorage::default()));
    c.bench_function("set_state", |b| {
        b.iter(|| async {
            let storage = black_box(MemoryStorage::default());
            let key = black_box(StorageKey::new(0, 1, 2));
            let value = black_box("test");

            storage.set_state(&key, value).await.unwrap();
        })
    });
    c.bench_function("get_state", |b| {
        b.iter(|| async {
            let storage = black_box(MemoryStorage::default());
            let key = black_box(StorageKey::new(0, 1, 2));

            storage.get_state(&key).await.unwrap();
        })
    });
    c.bench_function("get_state_with_set_state", |b| {
        b.iter(|| async {
            let storage = black_box(MemoryStorage::default());
            let key = black_box(StorageKey::new(0, 1, 2));
            let value = black_box("test");

            storage.set_state(&key, value).await.unwrap();
            storage.get_state(&key).await.unwrap();
        })
    });
    c.bench_function("remove_state", |b| {
        b.iter(|| async {
            let storage = black_box(MemoryStorage::default());
            let key = black_box(StorageKey::new(0, 1, 2));

            storage.remove_state(&key).await.unwrap();
        })
    });
    c.bench_function("remove_state_with_set_state", |b| {
        b.iter(|| async {
            let storage = black_box(MemoryStorage::default());
            let key = black_box(StorageKey::new(0, 1, 2));
            let value = black_box("test");

            storage.set_state(&key, value).await.unwrap();
            storage.remove_state(&key).await.unwrap();
        })
    });
    c.bench_function("set_data", |b| {
        b.iter(|| async {
            let storage = black_box(MemoryStorage::default());
            let key = black_box(StorageKey::new(0, 1, 2));
            let value = black_box("test");

            let mut data = HashMap::new();
            data.insert("test", value);

            storage.set_data(&key, data).await.unwrap();
        })
    });
    c.bench_function("get_data", |b| {
        b.iter(|| async {
            let storage = black_box(MemoryStorage::default());
            let key = black_box(StorageKey::new(0, 1, 2));

            storage.get_data::<String>(&key).await.unwrap();
        })
    });
    c.bench_function("get_data_with_set_data", |b| {
        b.iter(|| async {
            let storage = black_box(MemoryStorage::default());
            let key = black_box(StorageKey::new(0, 1, 2));
            let value = black_box("test");

            let mut data = HashMap::new();
            data.insert("test", value);

            storage.set_data(&key, data).await.unwrap();
            storage.get_data::<String>(&key).await.unwrap();
        })
    });
    c.bench_function("remove_data", |b| {
        b.iter(|| async {
            let storage = black_box(MemoryStorage::default());
            let key = black_box(StorageKey::new(0, 1, 2));

            storage.remove_data(&key).await.unwrap();
        })
    });
    c.bench_function("remove_data_with_set_data", |b| {
        b.iter(|| async {
            let storage = black_box(MemoryStorage::default());
            let key = black_box(StorageKey::new(0, 1, 2));
            let value = black_box("test");

            let mut data = HashMap::new();
            data.insert("test", value);

            storage.set_data(&key, data).await.unwrap();
            storage.remove_data(&key).await.unwrap();
        })
    });
}

criterion_group!(benches, memory_storage_benchmark);
criterion_main!(benches);
