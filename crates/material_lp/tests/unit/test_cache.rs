#[cfg(test)]
mod tests {
    use std::time::Duration;

    use material_lp::cache::Cache;
    use material_lp::resource::CelestialResource;

    fn celestial_resource_1() -> (CelestialResource, f64) {
        (
            CelestialResource { 
                key: "test_key".into(),
                planet_id: 1,
                resource_type_id: 1,
                init_output: 1.0,
                richness_index: 1,
                richness_value: 1
            }, 
            1.0
        )
    }

    fn celestial_resource_2() -> (CelestialResource, f64) {
        (
            CelestialResource { 
                key: "test_key".into(),
                planet_id: 1,
                resource_type_id: 1,
                init_output: 1.0,
                richness_index: 1,
                richness_value: 1
            }, 
            2.0
        )
    }

    #[test]
    fn test_cache_set_and_get() {
        let cache = Cache::new(Duration::from_secs(60));
        let key = "test_key".to_string();
        let celestial_resources = vec![celestial_resource_1()];
        let value = Ok(celestial_resources);

        cache.set(key.clone(), value.clone());
        let cached_value = cache.get(&key);

        assert_eq!(cached_value, Some(value));
    }

    #[test]
    fn test_cache_expired() {
        let cache = Cache::new(Duration::from_secs(1));
        let key = "test_key".to_string();
        let celestial_resources = vec![celestial_resource_1()];
        let value = Ok(celestial_resources);

        cache.set(key.clone(), value.clone());
        std::thread::sleep(Duration::from_secs(2));
        let cached_value = cache.get(&key);

        assert_eq!(cached_value, None);
    }

    #[test]
    fn test_cache_not_expired() {
        let cache = Cache::new(Duration::from_secs(60));
        let key = "test_key".to_string();
        let celestial_resources = vec![celestial_resource_1()];
        let value = Ok(celestial_resources);

        cache.set(key.clone(), value.clone());
        std::thread::sleep(Duration::from_secs(1));
        let cached_value = cache.get(&key);

        assert_eq!(cached_value, Some(value));
    }

    #[test]
    fn test_cache_overwrite() {
        let cache = Cache::new(Duration::from_secs(60));
        let key = "test_key".to_string();
        let celestial_resources_1 = vec![celestial_resource_1()];
        let celestial_resources_2 = vec![celestial_resource_2()];
        let value1 = Ok(celestial_resources_1);
        let value2 = Ok(celestial_resources_2);

        cache.set(key.clone(), value1.clone());
        cache.set(key.clone(), value2.clone());
        let cached_value = cache.get(&key);

        assert_eq!(cached_value, Some(value2));
    }

    #[test]
    fn test_cache_error_value() {
        let cache = Cache::new(Duration::from_secs(60));
        let key = "test_key".to_string();
        let value = Err("error".to_string());

        cache.set(key.clone(), value.clone());
        let cached_value = cache.get(&key);

        assert_eq!(cached_value, Some(value));
    }
}