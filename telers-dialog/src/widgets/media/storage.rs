//! Caching of Telegram `file_id`s for media sources

use std::{
    collections::{HashMap, VecDeque},
    sync::Mutex,
};

use super::{MediaContentType, MediaId};

type SourceKey = (Option<Box<str>>, Option<Box<str>>, MediaContentType);

pub trait MediaIdStorage: Send + Sync {
    fn get_media_id(
        &self,
        path: Option<&str>,
        url: Option<&str>,
        content_type: MediaContentType,
    ) -> Option<MediaId>;

    fn save_media_id(
        &self,
        path: Option<&str>,
        url: Option<&str>,
        content_type: MediaContentType,
        media_id: MediaId,
    );
}

fn source_key(
    path: Option<&str>,
    url: Option<&str>,
    content_type: MediaContentType,
) -> Option<SourceKey> {
    if path.is_none() && url.is_none() {
        return None;
    }
    Some((path.map(Into::into), url.map(Into::into), content_type))
}

pub struct InMemoryMediaIdStorage {
    inner: Mutex<Inner>,
    capacity: usize,
}

#[derive(Default)]
struct Inner {
    map: HashMap<SourceKey, MediaId>,
    order: VecDeque<SourceKey>,
}

impl InMemoryMediaIdStorage {
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Mutex::new(Inner::default()),
            capacity: capacity.max(1),
        }
    }
}

impl Default for InMemoryMediaIdStorage {
    fn default() -> Self {
        Self::new(1024)
    }
}

impl MediaIdStorage for InMemoryMediaIdStorage {
    fn get_media_id(
        &self,
        path: Option<&str>,
        url: Option<&str>,
        content_type: MediaContentType,
    ) -> Option<MediaId> {
        let key = source_key(path, url, content_type)?;
        let inner = self.inner.lock().expect("media id storage poisoned");
        inner.map.get(&key).cloned()
    }

    fn save_media_id(
        &self,
        path: Option<&str>,
        url: Option<&str>,
        content_type: MediaContentType,
        media_id: MediaId,
    ) {
        let Some(key) = source_key(path, url, content_type) else {
            return;
        };
        let mut inner = self.inner.lock().expect("media id storage poisoned");
        if inner.map.insert(key.clone(), media_id).is_none() {
            inner.order.push_back(key);
            if inner.order.len() > self.capacity {
                if let Some(evicted) = inner.order.pop_front() {
                    inner.map.remove(&evicted);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{InMemoryMediaIdStorage, MediaIdStorage};
    use crate::widgets::media::{MediaContentType, MediaId};

    #[test]
    fn save_then_get_roundtrips() {
        let storage = InMemoryMediaIdStorage::default();
        storage.save_media_id(
            None,
            Some("http://x/a.jpg"),
            MediaContentType::Photo,
            MediaId::new("file-a"),
        );

        assert_eq!(
            storage.get_media_id(None, Some("http://x/a.jpg"), MediaContentType::Photo),
            Some(MediaId::new("file-a"))
        );
    }

    #[test]
    fn get_returns_none_for_unknown_or_missing_source() {
        let storage = InMemoryMediaIdStorage::default();

        assert!(storage
            .get_media_id(None, Some("http://x/a.jpg"), MediaContentType::Photo)
            .is_none());

        storage.save_media_id(None, None, MediaContentType::Photo, MediaId::new("ignored"));
        assert!(storage
            .get_media_id(None, None, MediaContentType::Photo)
            .is_none());
    }

    #[test]
    fn evicts_oldest_past_capacity() {
        let storage = InMemoryMediaIdStorage::new(2);
        storage.save_media_id(
            None,
            Some("a"),
            MediaContentType::Photo,
            MediaId::new("id-a"),
        );
        storage.save_media_id(
            None,
            Some("b"),
            MediaContentType::Photo,
            MediaId::new("id-b"),
        );
        storage.save_media_id(
            None,
            Some("c"),
            MediaContentType::Photo,
            MediaId::new("id-c"),
        );

        assert!(storage
            .get_media_id(None, Some("a"), MediaContentType::Photo)
            .is_none());
        assert_eq!(
            storage.get_media_id(None, Some("b"), MediaContentType::Photo),
            Some(MediaId::new("id-b"))
        );
        assert_eq!(
            storage.get_media_id(None, Some("c"), MediaContentType::Photo),
            Some(MediaId::new("id-c"))
        );
    }
}
