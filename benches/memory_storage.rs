use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::HashMap;
use telers::fsm::{
    storage::{Storage, StorageKey},
    MemoryStorage,
};
use tokio::runtime::Builder;

fn memory_storage_benchmark(c: &mut Criterion) {
    c.bench_with_input(
        BenchmarkId::new("set_state", "no display data"),
        &{
            let storage = MemoryStorage::default();
            let key = StorageKey::new(0, 1, 2);
            let value = "test";

            (storage, key, value)
        },
        |b, (storage, key, value)| {
            b.to_async(Builder::new_current_thread().build().unwrap())
                .iter(|| storage.set_state(&key, *value))
        },
    );
    c.bench_with_input(
        BenchmarkId::new("get_state", "no display data"),
        &{
            let storage = MemoryStorage::default();
            let key = StorageKey::new(0, 1, 2);

            (storage, key)
        },
        |b, (storage, key)| {
            b.to_async(Builder::new_current_thread().build().unwrap())
                .iter(|| storage.get_state(&key))
        },
    );
    c.bench_with_input(
        BenchmarkId::new("remove_state", "no display data"),
        &{
            let storage = MemoryStorage::default();
            let key = StorageKey::new(0, 1, 2);

            (storage, key)
        },
        |b, (storage, key)| {
            b.to_async(Builder::new_current_thread().build().unwrap())
                .iter(|| storage.remove_state(&key))
        },
    );
    c.bench_with_input(
        BenchmarkId::new("set_data", "no display data"),
        &{
            let storage = MemoryStorage::default();
            let key = StorageKey::new(0, 1, 2);
            let value = "test";

            let mut data = HashMap::new();
            data.insert("test", value);

            (storage, key, data)
        },
        |b, (storage, key, data)| {
            b.to_async(Builder::new_current_thread().build().unwrap())
                .iter(|| storage.set_data(&key, data.clone()))
        },
    );
    c.bench_with_input(
        BenchmarkId::new("get_data", "no display data"),
        &{
            let storage = MemoryStorage::default();
            let key = StorageKey::new(0, 1, 2);

            (storage, key)
        },
        |b, (storage, key)| {
            b.to_async(Builder::new_current_thread().build().unwrap())
                .iter(|| storage.get_data::<String>(&key))
        },
    );
    c.bench_with_input(
        BenchmarkId::new("remove_data", "no display data"),
        &{
            let storage = MemoryStorage::default();
            let key = StorageKey::new(0, 1, 2);

            (storage, key)
        },
        |b, (storage, key)| {
            b.to_async(Builder::new_current_thread().build().unwrap())
                .iter(|| storage.remove_data(&key))
        },
    );
}

criterion_group!(benches, memory_storage_benchmark);
criterion_main!(benches);
