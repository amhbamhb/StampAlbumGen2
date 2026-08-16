---
type: Rust Function
title: collect_us1847_data
resource: src/us1847/collector.rs#L1622-L1635
generated:
  by: okf-rs/0.4.0
---

# Signature

`pub fn collect_us1847_data( _sequencer: &mut i32, _separator: &str, _site_name: &str, _images_directory: &str, _month_number: &dyn Fn(&str) -> String, _get_image: &dyn Fn(&str, bool) -> crate::types::ImageStatus, _base_url: &str, _start_year: i32, _end_year: i32, _overwrite_images: bool, ) -> Result<(), Box<dyn Error>>`