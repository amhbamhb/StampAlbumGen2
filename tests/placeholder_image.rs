use stamp_album_gen2::placeholder_image::generate_placeholder_image;
use std::path::Path;

#[test]
fn test_generate_placeholder_image() {
    let test_path = Path::new("test_placeholder.png");
    let result = generate_placeholder_image(100, 75, test_path);
    assert!(result.is_ok());

    assert!(test_path.exists());

    std::fs::remove_file(test_path).unwrap();
}
