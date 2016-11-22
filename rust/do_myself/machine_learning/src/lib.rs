mod machine_setting {
    extern crate leaf;
    use leaf::*;
    pub fn start() {
        let test: LayerConfig = LayerConfig::new("liner1", LinearConfig { output_size: 300 });
    }
}
#[cfg(test)]
mod tests {}
