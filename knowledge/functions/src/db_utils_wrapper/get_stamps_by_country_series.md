---
type: Rust Function
title: get_stamps_by_country_series
resource: src/db_utils_wrapper.rs#L177-L198
generated:
  by: okf-rs/0.4.0
---

# Signature

`pub fn get_stamps_by_country_series( conn: &Connection, country: &str, series: &str, ) -> Result< Vec<( i32, String, String, i32, String, i32, String, String, String, String, String, String, )>, >`