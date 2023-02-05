use std::path::Path;

fn main() {
    let config = preamble_stitch::get_config_from_fs(Path::new("stitchconfig.yml")).unwrap();
    let files = preamble_stitch::read_files(&config.order).unwrap();
    let stitched = preamble_stitch::stitch_files(&files);
    let output_file = match &config.output_file {
        Some(path) => path,
        None => Path::new("output.sty"),
    };
    preamble_stitch::write_file(stitched, output_file).unwrap();
}
