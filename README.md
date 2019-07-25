# `AHT10`

An embedded rust `no_std` driver for the AHT10 temperature and humidity sensor.

## Usage

Include [library](https://crates.io/crates/aht10) as a dependency in your Cargo.toml:

```
[dependencies.aht10]
version = "<version>"
```

To use the sensor, call `AHT10::new` with an embedded-hal i2c device:
```rust
extern crate aht10;

// Start the sensor.
let mut dev = AHT10::new(i2c_dev, embedded_hal::Delay).unwrap();
// Read humidity and temperature.
let (h, t) = dev.read().unwrap();
```

## Documentation

API documentation is generated on [docs.rs](https://docs.rs/aht10).

## License

Licensed under AGPL-3.0.

