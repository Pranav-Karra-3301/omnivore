# Add New Feature

Guide for implementing a new feature in Omnivore.

## Pre-Implementation Checklist

Before writing code:

1. [ ] Understand the existing architecture
2. [ ] Check if similar functionality exists
3. [ ] Identify which package(s) need changes
4. [ ] Consider error handling requirements
5. [ ] Plan the public API

## Implementation Steps

### 1. Create or Modify Types

Define necessary types in the appropriate module:

```rust
// In omnivore-core/src/your_module.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YourFeature {
    // Fields
}
```

### 2. Implement Core Logic

Add implementation with proper error handling:

```rust
impl YourFeature {
    pub fn new(config: FeatureConfig) -> Result<Self, FeatureError> {
        // Implementation with validation
    }

    pub async fn process(&self, input: Input) -> Result<Output, FeatureError> {
        // Async implementation
    }
}
```

### 3. Add Error Types

Define feature-specific errors:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FeatureError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Processing failed: {0}")]
    ProcessingError(String),
}
```

### 4. Write Tests

Add unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_creation() {
        let feature = YourFeature::new(Default::default());
        assert!(feature.is_ok());
    }

    #[tokio::test]
    async fn test_feature_processing() {
        let feature = YourFeature::new(Default::default()).unwrap();
        let result = feature.process(test_input()).await;
        assert!(result.is_ok());
    }
}
```

### 5. Add Documentation

Document the public API:

```rust
/// Brief description of the feature.
///
/// # Examples
///
/// ```
/// use omnivore_core::YourFeature;
///
/// let feature = YourFeature::new(config)?;
/// let result = feature.process(input).await?;
/// ```
///
/// # Errors
///
/// Returns `FeatureError::InvalidConfig` if...
```

### 6. Export from lib.rs

Make the feature publicly accessible:

```rust
// In omnivore-core/src/lib.rs
pub mod your_module;
pub use your_module::YourFeature;
```

## Post-Implementation Checklist

- [ ] All tests pass (`cargo test`)
- [ ] No Clippy warnings (`cargo clippy`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Documentation is complete
- [ ] No security issues introduced
