use stamp_album_gen2::native_pdf::{document::Bounds, stamp_box::StampBoxGeometry};

fn physical_bounds(width: f64, height: f64) -> Bounds {
    Bounds {
        left: 40.0,
        bottom: 50.0,
        right: 40.0 + width,
        top: 50.0 + height,
    }
}

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-9, "{actual} != {expected}");
}

#[test]
fn total_additions_are_split_per_edge_not_applied_twice() -> Result<(), String> {
    let geometry =
        StampBoxGeometry::from_total_additions(physical_bounds(22.0, 25.0), 6.0, 6.0, 0.0)?;
    let frame = geometry.frame_bounds();
    let physical = geometry.physical_bounds();
    assert_close(frame.right - frame.left, 28.0);
    assert_close(frame.top - frame.bottom, 31.0);
    assert_close(geometry.horizontal_clearance_mm(), 3.0);
    assert_close(geometry.vertical_clearance_mm(), 3.0);
    assert_close(physical.left - frame.left, 3.0);
    assert_close(frame.right - physical.right, 3.0);
    assert_close(physical.bottom - frame.bottom, 3.0);
    assert_close(frame.top - physical.top, 3.0);
    assert_close(physical.right - physical.left, 22.0);
    assert_close(physical.top - physical.bottom, 25.0);
    Ok(())
}

#[test]
fn physical_dimensions_are_not_capped_or_rotated_for_any_item_format() -> Result<(), String> {
    for (width, height) in [
        (25.0, 40.0),
        (22.5, 25.0),
        (44.0, 90.0),
        (125.0, 66.0),
        (40.0, 25.0),
        (80.0, 68.0),
        (103.0, 51.0),
        (51.0, 26.0),
    ] {
        let geometry =
            StampBoxGeometry::from_total_additions(physical_bounds(width, height), 3.0, 5.0, 0.5)?;
        let physical = geometry.physical_bounds();
        let frame = geometry.frame_bounds();
        assert_close(physical.right - physical.left, width);
        assert_close(physical.top - physical.bottom, height);
        assert_close(frame.right - frame.left, width + 3.5);
        assert_close(frame.top - frame.bottom, height + 5.5);
        assert_close(geometry.horizontal_clearance_mm(), 1.75);
        assert_close(geometry.vertical_clearance_mm(), 2.75);
    }
    Ok(())
}

#[test]
fn zero_clearance_and_signed_adjustments_preserve_total_addition_semantics() -> Result<(), String> {
    for (extra_w, extra_h, adjustment, clearance_w, clearance_h) in [
        (0.0, 0.0, 0.0, 0.0, 0.0),
        (3.0, 5.0, -1.0, 1.0, 2.0),
        (-1.0, 1.0, 2.0, 0.5, 1.5),
        (1.0, 1.0, -1.0, 0.0, 0.0),
    ] {
        let geometry = StampBoxGeometry::from_total_additions(
            physical_bounds(22.0, 25.0),
            extra_w,
            extra_h,
            adjustment,
        )?;
        assert_close(geometry.horizontal_clearance_mm(), clearance_w);
        assert_close(geometry.vertical_clearance_mm(), clearance_h);
    }
    Ok(())
}

#[test]
fn invalid_sizes_and_frames_that_would_shrink_known_stamps_fail_explicitly() {
    for (width, height) in [(0.0, 25.0), (-1.0, 25.0), (22.0, 0.0), (22.0, f64::NAN)] {
        assert!(
            StampBoxGeometry::from_total_additions(physical_bounds(width, height), 3.0, 3.0, 0.0,)
                .is_err()
        );
    }
    for (w, h, adjustment) in [
        (-1.0, 3.0, 0.0),
        (3.0, -1.0, 0.0),
        (3.0, 3.0, -4.0),
        (f64::NAN, 3.0, 0.0),
        (3.0, f64::INFINITY, 0.0),
        (3.0, 3.0, f64::NEG_INFINITY),
        (f64::MAX, 3.0, 0.0),
    ] {
        let error =
            StampBoxGeometry::from_total_additions(physical_bounds(22.0, 25.0), w, h, adjustment)
                .expect_err("invalid mounting geometry must fail");
        assert!(error.contains("native_pdf.stamp_box"), "{error}");
    }
}

#[test]
fn coordinate_and_extent_overflow_are_rejected_before_rendering() {
    for physical in [
        Bounds {
            left: f64::NEG_INFINITY,
            ..physical_bounds(22.0, 25.0)
        },
        Bounds {
            bottom: f64::NAN,
            ..physical_bounds(22.0, 25.0)
        },
        Bounds {
            left: -f64::MAX,
            right: f64::MAX,
            ..physical_bounds(22.0, 25.0)
        },
        Bounds {
            left: 7e307,
            right: 8e307,
            ..physical_bounds(22.0, 25.0)
        },
        Bounds {
            left: -3.5e307,
            right: 3.5e307,
            ..physical_bounds(22.0, 25.0)
        },
    ] {
        assert!(StampBoxGeometry::from_total_additions(physical, 0.0, 0.0, 0.0).is_err());
    }
}
