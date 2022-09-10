use crate::{ext::*, *};
use serde::de::DeserializeOwned;
use std::str::FromStr;

/// Provides binder extension methods for a [Configuration](trait.Configuration.html).
pub trait ConfigurationBinder {
    /// Creates and returns a structure bound to the configuration.
    fn get_as<T: DeserializeOwned>(&self) -> T;

    /// Binds the configuration to the specified instance.
    ///
    /// # Arguments
    ///
    /// * `instance` - The instance to bind the configuration to
    fn bind<T: DeserializeOwned>(&self, instance: &mut T);

    /// Binds the specified configuration section to the provided instance.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the configuration section to bind
    /// * `instance` - The instance to bind the configuration to
    fn bind_at<T: DeserializeOwned>(&self, key: impl AsRef<str>, instance: &mut T);

    /// Gets a typed value from the configuration.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the value to retrieve
    fn get_value<T: FromStr>(&self, key: impl AsRef<str>) -> Result<Option<T>, T::Err>;

    /// Gets an optional, typed value from the configuration.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the value to retrieve
    fn get_value_or_default<T: FromStr + Default>(&self, key: impl AsRef<str>) -> Result<T, T::Err>;
}

impl ConfigurationBinder for dyn Configuration {
    fn get_as<T: DeserializeOwned>(&self) -> T {
        from_config::<T>(self).unwrap()
    }

    fn bind<T: DeserializeOwned>(&self, instance: &mut T) {
        bind_config(self, instance).unwrap()
    }

    fn bind_at<T: DeserializeOwned>(&self, key: impl AsRef<str>, instance: &mut T) {
        let section = self.section(key.as_ref());

        if section.exists() {
            bind_config(section.as_config(), instance).unwrap()
        }
    }

    fn get_value<T: FromStr>(&self, key: impl AsRef<str>) -> Result<Option<T>, T::Err> {
        let section = self.section(key.as_ref());
        let value = if section.exists() {
            Some(T::from_str(section.value())?)
        } else {
            None
        };

        Ok(value)
    }

    fn get_value_or_default<T: FromStr + Default>(&self, key: impl AsRef<str>) -> Result<T, T::Err> {
        let section = self.section(key.as_ref());
        let value = if section.exists() {
            T::from_str(section.value())?
        } else {
            T::default()
        };

        Ok(value)
    }
}

impl<C: AsRef<dyn Configuration>> ConfigurationBinder for C {
    fn get_as<T: DeserializeOwned>(&self) -> T {
        from_config::<T>(self.as_ref()).unwrap()
    }

    fn bind<T: DeserializeOwned>(&self, instance: &mut T) {
        bind_config(self.as_ref(), instance).unwrap()
    }

    fn bind_at<T: DeserializeOwned>(&self, key: impl AsRef<str>, instance: &mut T) {
        let section = self.as_ref().section(key.as_ref());

        if section.exists() {
            bind_config(section.as_config(), instance).unwrap()
        }
    }

    fn get_value<T: FromStr>(&self, key: impl AsRef<str>) -> Result<Option<T>, T::Err> {
        let section = self.as_ref().section(key.as_ref());
        let value = if section.exists() {
            Some(T::from_str(section.value())?)
        } else {
            None
        };

        Ok(value)
    }

    fn get_value_or_default<T: FromStr + Default>(&self, key: impl AsRef<str>) -> Result<T, T::Err> {
        let section = self.as_ref().section(key.as_ref());
        let value = if section.exists() {
            T::from_str(section.value())?
        } else {
            T::default()
        };

        Ok(value)
    }
}
