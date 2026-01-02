use lru_cache::LruCache;

/// Test 1: Basic put and get operations
#[test]
fn test_basic_put_and_get() {
    let mut cache: LruCache<i32, String> = LruCache::new(3);
    
    cache.put(1, "one".to_string());
    cache.put(2, "two".to_string());
    cache.put(3, "three".to_string());
    
    assert_eq!(cache.get(&1), Some("one".to_string()));
    assert_eq!(cache.get(&2), Some("two".to_string()));
    assert_eq!(cache.get(&3), Some("three".to_string()));
}

/// Test 2: Get returns None for non-existent key
#[test]
fn test_get_missing_key_returns_none() {
    let mut cache: LruCache<i32, i32> = LruCache::new(2);
    
    cache.put(1, 100);
    
    assert_eq!(cache.get(&1), Some(100));
    assert_eq!(cache.get(&999), None); // Key doesn't exist
    assert_eq!(cache.get(&0), None);   // Never inserted
}

/// Test 3: LRU eviction when capacity is exceeded
#[test]
fn test_eviction_when_capacity_exceeded() {
    let mut cache: LruCache<i32, &str> = LruCache::new(2);
    
    cache.put(1, "one");
    cache.put(2, "two");
    // Cache is now full: [2] <-> [1] (head to tail)
    
    cache.put(3, "three");
    // Key 1 should be evicted (LRU): [3] <-> [2]
    
    assert_eq!(cache.get(&1), None);           // Evicted
    assert_eq!(cache.get(&2), Some("two"));    // Still present
    assert_eq!(cache.get(&3), Some("three"));  // Just added
}

/// Test 4: Updating existing key moves it to head (most recently used)
#[test]
fn test_update_existing_key() {
    let mut cache: LruCache<i32, &str> = LruCache::new(2);
    
    cache.put(1, "one");
    cache.put(2, "two");
    // Order: [2] <-> [1] (head to tail)
    
    // Update key 1 - should move to head and update value
    cache.put(1, "ONE_UPDATED");
    // Order: [1] <-> [2] (head to tail)
    
    // Now add key 3 - should evict key 2 (now LRU)
    cache.put(3, "three");
    
    assert_eq!(cache.get(&1), Some("ONE_UPDATED")); // Updated and present
    assert_eq!(cache.get(&2), None);                 // Evicted
    assert_eq!(cache.get(&3), Some("three"));        // Just added
}

/// Test 5: Get operation moves accessed item to head (affects eviction order)
#[test]
fn test_get_updates_recency() {
    let mut cache: LruCache<i32, &str> = LruCache::new(3);
    
    cache.put(1, "one");
    cache.put(2, "two");
    cache.put(3, "three");
    // Order: [3] <-> [2] <-> [1] (head to tail, 1 is LRU)
    
    // Access key 1 - moves it to head
    let _ = cache.get(&1);
    // Order: [1] <-> [3] <-> [2] (head to tail, 2 is now LRU)
    
    // Add key 4 - should evict key 2 (LRU)
    cache.put(4, "four");
    
    assert_eq!(cache.get(&1), Some("one"));    // Accessed, still present
    assert_eq!(cache.get(&2), None);            // Evicted (was LRU after get(1))
    assert_eq!(cache.get(&3), Some("three"));  // Still present
    assert_eq!(cache.get(&4), Some("four"));   // Just added
}

