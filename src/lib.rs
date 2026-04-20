use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

const MODE_DRAW: u32 = 2;
const MODE_CONTROL: u32 = 3;
const DRAW_VERSION_RAW: u32 = 0;
const DRAW_VERSION_RLE: u32 = 1;
const VERSION_HEADER: u32 = 0;
const VERSION_X: u32 = 1;
const VERSION_Y: u32 = 2;
const VERSION_LEN: u32 = 3;
const VERSION_FRAME: u32 = 4;
const VERSION_DELTA: u32 = 5;

const MENTAL_TYPE_BIT: u32 = 0x0100;
const FRAME_WORD_FLAG: u32 = 0x0800;
const LEGACY_RLE_RUN_FLAG: u32 = 0x0400;
const DELTA_WORD_FLAG: u32 = 0x0200;
const PROFILE_12_FLAG: u32 = 0x0002;

const PROFILE_COMPASS_8: u8 = 0;
const PROFILE_D6_12: u8 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct EncodeOptions {
    use_rle: bool,
    frame_index: Option<u8>,
    delta_ms: Option<u8>,
}

impl Default for EncodeOptions {
    fn default() -> Self {
        Self {
            use_rle: true,
            frame_index: None,
            delta_ms: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MentalStrokePayload {
    move_x: u8,
    move_y: u8,
    directions: Vec<u8>,
    form_class: u8,
    symmetry: u8,
    direction_profile: u8,
    spatial_frequency: u8,
    drift_speed: u8,
    frame_index: Option<u8>,
    delta_ms: u8,
}

fn py_value_error(message: impl Into<String>) -> PyErr {
    PyValueError::new_err(message.into())
}

fn build_word(mode: u32, version: u32, payload: u32) -> u32 {
    ((mode & 0x3) << 18) | ((version & 0x3) << 16) | (payload & 0x0FFF)
}

fn extract_fields(word: u32) -> (u32, u32, u32) {
    let mode = (word >> 18) & 0x3;
    let version = (word >> 16) & 0x3;
    let payload = word & 0x0FFF;
    (mode, version, payload)
}

fn wire_control_version(version: u32) -> u32 {
    match version {
        VERSION_FRAME | VERSION_DELTA => VERSION_LEN,
        _ => version,
    }
}

fn direction_modulus(profile: u8) -> u8 {
    if profile == PROFILE_D6_12 {
        12
    } else {
        8
    }
}

fn validate_stroke(stroke: &MentalStrokePayload) -> PyResult<()> {
    if stroke.form_class > 3 {
        return Err(py_value_error(format!(
            "form_class must be in [0,3], got {}",
            stroke.form_class
        )));
    }
    if stroke.symmetry > 3 {
        return Err(py_value_error(format!(
            "symmetry must be in [0,3], got {}",
            stroke.symmetry
        )));
    }
    if stroke.direction_profile > 1 {
        return Err(py_value_error(format!(
            "direction_profile must be 0 or 1, got {}",
            stroke.direction_profile
        )));
    }
    if stroke.spatial_frequency > 7 {
        return Err(py_value_error(format!(
            "spatial_frequency must be in [0,7], got {}",
            stroke.spatial_frequency
        )));
    }
    if stroke.drift_speed > 3 {
        return Err(py_value_error(format!(
            "drift_speed must be in [0,3], got {}",
            stroke.drift_speed
        )));
    }
    let modulus = direction_modulus(stroke.direction_profile);
    for &direction in &stroke.directions {
        if direction >= modulus {
            return Err(py_value_error(format!(
                "direction must be in [0,{}] for profile {}, got {}",
                modulus - 1,
                stroke.direction_profile,
                direction
            )));
        }
    }
    Ok(())
}

fn header_payload(stroke: &MentalStrokePayload) -> u32 {
    let form_bits = ((stroke.form_class as u32) & 0x3) << 10;
    let sym = (stroke.symmetry as u32) & 0x3;
    let sym_bits = ((sym >> 1) & 0x1) << 9 | ((sym & 0x1) << 7);
    let freq_bits = ((stroke.spatial_frequency as u32) & 0x7) << 4;
    let speed_bits = ((stroke.drift_speed as u32) & 0x3) << 2;
    let profile_bits = if stroke.direction_profile == PROFILE_D6_12 {
        PROFILE_12_FLAG
    } else {
        0
    };
    MENTAL_TYPE_BIT | form_bits | sym_bits | freq_bits | speed_bits | profile_bits
}

fn parse_header_payload(payload: u32) -> (u8, u8, u8, u8, u8) {
    let form_class = ((payload >> 10) & 0x3) as u8;
    let sym_hi = ((payload >> 9) & 0x1) as u8;
    let sym_lo = ((payload >> 7) & 0x1) as u8;
    let symmetry = (sym_hi << 1) | sym_lo;
    let direction_profile = if (payload & PROFILE_12_FLAG) != 0 {
        PROFILE_D6_12
    } else {
        PROFILE_COMPASS_8
    };
    let spatial_frequency = ((payload >> 4) & 0x7) as u8;
    let drift_speed = ((payload >> 2) & 0x3) as u8;
    (
        form_class,
        symmetry,
        direction_profile,
        spatial_frequency,
        drift_speed,
    )
}

fn draw_payload(direction: u8, profile: u8) -> u32 {
    if profile == PROFILE_D6_12 {
        MENTAL_TYPE_BIT | (((direction as u32) & 0xF) << 4)
    } else {
        MENTAL_TYPE_BIT | (((direction as u32) & 0x7) << 5)
    }
}

fn frame_payload(frame_index: u8) -> u32 {
    MENTAL_TYPE_BIT | FRAME_WORD_FLAG | (frame_index as u32)
}

fn delta_payload(delta_ms: u8) -> u32 {
    MENTAL_TYPE_BIT | DELTA_WORD_FLAG | (delta_ms as u32)
}

fn is_frame_word(mode: u32, version: u32, payload: u32) -> bool {
    mode == MODE_CONTROL
        && version == wire_control_version(VERSION_FRAME)
        && (payload & MENTAL_TYPE_BIT) != 0
        && (payload & FRAME_WORD_FLAG) != 0
}

fn is_delta_word(mode: u32, version: u32, payload: u32) -> bool {
    mode == MODE_CONTROL
        && version == wire_control_version(VERSION_DELTA)
        && (payload & MENTAL_TYPE_BIT) != 0
        && (payload & DELTA_WORD_FLAG) != 0
}

fn run_length_encode(directions: &[u8], max_run: usize) -> Vec<(u8, usize)> {
    if directions.is_empty() {
        return Vec::new();
    }
    let mut runs = Vec::new();
    let mut current = directions[0];
    let mut length = 1usize;
    for &direction in directions.iter().skip(1) {
        if direction == current && length < max_run {
            length += 1;
            continue;
        }
        runs.push((current, length));
        current = direction;
        length = 1;
    }
    runs.push((current, length));
    runs
}

fn parse_u8(item: &Bound<'_, PyAny>, key: &str) -> PyResult<u8> {
    let value = item
        .downcast::<PyDict>()?
        .get_item(key)?
        .ok_or_else(|| py_value_error(format!("missing key: {key}")))?;
    value.extract()
}

fn parse_u8_vec(item: &Bound<'_, PyAny>, key: &str) -> PyResult<Vec<u8>> {
    let value = item
        .downcast::<PyDict>()?
        .get_item(key)?
        .ok_or_else(|| py_value_error(format!("missing key: {key}")))?;
    value.extract()
}

fn parse_optional_u8(item: &Bound<'_, PyAny>, key: &str) -> PyResult<Option<u8>> {
    let dict = item.downcast::<PyDict>()?;
    match dict.get_item(key)? {
        Some(value) => {
            if value.is_none() {
                Ok(None)
            } else {
                Ok(Some(value.extract()?))
            }
        }
        None => Ok(None),
    }
}

fn parse_stroke_payload(item: &Bound<'_, PyAny>) -> PyResult<MentalStrokePayload> {
    let stroke = MentalStrokePayload {
        move_x: parse_u8(item, "move_x")?,
        move_y: parse_u8(item, "move_y")?,
        directions: parse_u8_vec(item, "directions")?,
        form_class: parse_u8(item, "form_class")?,
        symmetry: parse_u8(item, "symmetry")?,
        direction_profile: parse_u8(item, "direction_profile")?,
        spatial_frequency: parse_u8(item, "spatial_frequency")?,
        drift_speed: parse_u8(item, "drift_speed")?,
        frame_index: parse_optional_u8(item, "frame_index")?,
        delta_ms: parse_u8(item, "delta_ms")?,
    };
    validate_stroke(&stroke)?;
    Ok(stroke)
}

fn parse_encode_options(metadata: Option<&Bound<'_, PyDict>>) -> PyResult<EncodeOptions> {
    let mut options = EncodeOptions::default();
    let Some(metadata) = metadata else {
        return Ok(options);
    };
    if let Some(value) = metadata.get_item("use_rle")? {
        options.use_rle = value.extract()?;
    }
    if let Some(value) = metadata.get_item("frame_index")? {
        if !value.is_none() {
            options.frame_index = Some(value.extract()?);
        }
    }
    if let Some(value) = metadata.get_item("delta_ms")? {
        if !value.is_none() {
            options.delta_ms = Some(value.extract()?);
        }
    }
    Ok(options)
}

fn pack_mental_raw(strokes: &[MentalStrokePayload], options: EncodeOptions) -> PyResult<Vec<u32>> {
    let mut words = Vec::new();
    for stroke in strokes {
        let mut frame_index = stroke.frame_index;
        if frame_index.is_none() {
            frame_index = options.frame_index;
        }
        let mut delta_ms = stroke.delta_ms;
        if delta_ms == 0 {
            if let Some(global_delta) = options.delta_ms {
                delta_ms = global_delta;
            }
        }
        words.push(build_word(MODE_CONTROL, VERSION_HEADER, header_payload(stroke)));
        words.push(build_word(MODE_CONTROL, VERSION_X, MENTAL_TYPE_BIT | stroke.move_x as u32));
        words.push(build_word(MODE_CONTROL, VERSION_Y, MENTAL_TYPE_BIT | stroke.move_y as u32));
        words.push(build_word(
            MODE_CONTROL,
            VERSION_LEN,
            MENTAL_TYPE_BIT | stroke.directions.len() as u32,
        ));
        if let Some(frame) = frame_index {
            words.push(build_word(
                MODE_CONTROL,
                wire_control_version(VERSION_FRAME),
                frame_payload(frame),
            ));
        }
        words.push(build_word(
            MODE_CONTROL,
            wire_control_version(VERSION_DELTA),
            delta_payload(delta_ms),
        ));
        for &direction in &stroke.directions {
            words.push(build_word(
                MODE_DRAW,
                DRAW_VERSION_RAW,
                draw_payload(direction, stroke.direction_profile),
            ));
        }
    }
    Ok(words)
}

fn pack_mental_rle(strokes: &[MentalStrokePayload], options: EncodeOptions) -> PyResult<Vec<u32>> {
    let mut words = Vec::new();
    for stroke in strokes {
        let max_run = if stroke.direction_profile == PROFILE_D6_12 { 16 } else { 32 };
        let runs = run_length_encode(&stroke.directions, max_run);
        if runs.len() > 255 {
            return Err(py_value_error(
                "stroke too long for v1 RLE run-count field (max 255 runs)",
            ));
        }
        let mut frame_index = stroke.frame_index;
        if frame_index.is_none() {
            frame_index = options.frame_index;
        }
        let mut delta_ms = stroke.delta_ms;
        if delta_ms == 0 {
            if let Some(global_delta) = options.delta_ms {
                delta_ms = global_delta;
            }
        }
        words.push(build_word(MODE_CONTROL, VERSION_HEADER, header_payload(stroke)));
        words.push(build_word(MODE_CONTROL, VERSION_X, MENTAL_TYPE_BIT | stroke.move_x as u32));
        words.push(build_word(MODE_CONTROL, VERSION_Y, MENTAL_TYPE_BIT | stroke.move_y as u32));
        words.push(build_word(
            MODE_CONTROL,
            VERSION_LEN,
            MENTAL_TYPE_BIT | runs.len() as u32,
        ));
        if let Some(frame) = frame_index {
            words.push(build_word(
                MODE_CONTROL,
                wire_control_version(VERSION_FRAME),
                frame_payload(frame),
            ));
        }
        words.push(build_word(
            MODE_CONTROL,
            wire_control_version(VERSION_DELTA),
            delta_payload(delta_ms),
        ));
        for (direction, run_length) in runs {
            let payload = if stroke.direction_profile == PROFILE_D6_12 {
                MENTAL_TYPE_BIT | (((direction as u32) & 0xF) << 4) | (((run_length - 1) as u32) & 0x0F)
            } else {
                MENTAL_TYPE_BIT | (((direction as u32) & 0x7) << 5) | (((run_length - 1) as u32) & 0x1F)
            };
            words.push(build_word(MODE_DRAW, DRAW_VERSION_RLE, payload));
        }
    }
    Ok(words)
}

fn pack_mental_payloads(strokes: &[MentalStrokePayload], options: EncodeOptions) -> PyResult<Vec<u32>> {
    if options.use_rle {
        pack_mental_rle(strokes, options)
    } else {
        pack_mental_raw(strokes, options)
    }
}

fn unpack_mental_payloads(words: &[u32]) -> (Option<String>, Vec<Option<u8>>, Vec<u8>, Vec<MentalStrokePayload>) {
    if words.is_empty() {
        return (None, Vec::new(), Vec::new(), Vec::new());
    }

    let mut strokes = Vec::new();
    let mut frame_indices = Vec::new();
    let mut delta_values = Vec::new();
    let mut encodings = Vec::new();
    let mut i = 0usize;

    while i < words.len() {
        let (mode, version, payload) = extract_fields(words[i]);
        if mode != MODE_CONTROL || version != VERSION_HEADER || (payload & MENTAL_TYPE_BIT) == 0 {
            i += 1;
            continue;
        }
        if i + 3 >= words.len() {
            break;
        }

        let (form_class, symmetry, direction_profile, spatial_frequency, drift_speed) =
            parse_header_payload(payload);

        let (mx_mode, mx_version, mx_payload) = extract_fields(words[i + 1]);
        let (my_mode, my_version, my_payload) = extract_fields(words[i + 2]);
        let (ln_mode, ln_version, ln_payload) = extract_fields(words[i + 3]);
        if mx_mode != MODE_CONTROL
            || mx_version != VERSION_X
            || (mx_payload & MENTAL_TYPE_BIT) == 0
            || my_mode != MODE_CONTROL
            || my_version != VERSION_Y
            || (my_payload & MENTAL_TYPE_BIT) == 0
            || ln_mode != MODE_CONTROL
            || ln_version != VERSION_LEN
            || (ln_payload & MENTAL_TYPE_BIT) == 0
        {
            i += 1;
            continue;
        }

        let move_x = (mx_payload & 0xFF) as u8;
        let move_y = (my_payload & 0xFF) as u8;
        let unit_count = (ln_payload & 0xFF) as usize;

        let mut cursor = i + 4;
        let mut frame_index: Option<u8> = None;
        let mut delta_ms = 0u8;
        let mut seen_frame = false;
        let mut seen_delta = false;
        while cursor < words.len() {
            let (c_mode, c_version, c_payload) = extract_fields(words[cursor]);
            if !seen_frame && is_frame_word(c_mode, c_version, c_payload) {
                frame_index = Some((c_payload & 0xFF) as u8);
                seen_frame = true;
                cursor += 1;
                continue;
            }
            if !seen_delta && is_delta_word(c_mode, c_version, c_payload) {
                delta_ms = (c_payload & 0xFF) as u8;
                seen_delta = true;
                cursor += 1;
                continue;
            }
            break;
        }

        let available = unit_count.min(words.len().saturating_sub(cursor));
        let mut directions = Vec::new();
        let mut consumed_units = 0usize;
        let mut encoding = "raw";

        for j in 0..available {
            let (d_mode, d_version, d_payload) = extract_fields(words[cursor + j]);
            if d_mode != MODE_DRAW || (d_payload & MENTAL_TYPE_BIT) == 0 {
                break;
            }
            let is_rle = d_version == DRAW_VERSION_RLE || (d_payload & LEGACY_RLE_RUN_FLAG) != 0;
            if is_rle {
                encoding = "rle";
                let (direction, run_length) = if direction_profile == PROFILE_D6_12 {
                    let direction = ((d_payload >> 4) & 0xF) as u8;
                    if direction >= 12 {
                        break;
                    }
                    (direction, ((d_payload & 0x0F) + 1) as usize)
                } else {
                    (((d_payload >> 5) & 0x7) as u8, ((d_payload & 0x1F) + 1) as usize)
                };
                directions.extend(std::iter::repeat(direction).take(run_length));
            } else {
                let direction = if direction_profile == PROFILE_D6_12 {
                    let direction = ((d_payload >> 4) & 0xF) as u8;
                    if direction >= 12 {
                        break;
                    }
                    direction
                } else {
                    ((d_payload >> 5) & 0x7) as u8
                };
                directions.push(direction);
            }
            consumed_units += 1;
        }

        let stroke = MentalStrokePayload {
            move_x,
            move_y,
            directions,
            form_class,
            symmetry,
            direction_profile,
            spatial_frequency,
            drift_speed,
            frame_index,
            delta_ms,
        };
        strokes.push(stroke);
        frame_indices.push(frame_index);
        delta_values.push(delta_ms);
        encodings.push(encoding.to_string());
        i = cursor + consumed_units;
    }

    let encoding = if encodings.is_empty() {
        None
    } else if encodings.iter().all(|value| value == &encodings[0]) {
        Some(encodings[0].clone())
    } else {
        Some("mixed".to_string())
    };

    (encoding, frame_indices, delta_values, strokes)
}

#[pyfunction]
fn pack_mental_strokes_payload(
    strokes: &Bound<'_, PyList>,
    metadata: Option<&Bound<'_, PyDict>>,
) -> PyResult<Vec<u32>> {
    let mut payloads = Vec::with_capacity(strokes.len());
    for item in strokes.iter() {
        payloads.push(parse_stroke_payload(&item)?);
    }
    let options = parse_encode_options(metadata)?;
    pack_mental_payloads(&payloads, options)
}

#[pyfunction]
fn unpack_mental_words_payload(
    py: Python<'_>,
    words: Vec<u32>,
) -> PyResult<(Py<PyDict>, Py<PyList>)> {
    let (encoding, frame_indices, delta_values, strokes) = unpack_mental_payloads(&words);
    let metadata = PyDict::new(py);
    metadata.set_item("stroke_count", strokes.len())?;
    if frame_indices.iter().any(|value| value.is_some()) {
        let values: Vec<Option<u8>> = frame_indices;
        metadata.set_item("frame_indices", values)?;
    }
    if delta_values.iter().any(|value| *value != 0) {
        metadata.set_item("delta_ms", delta_values)?;
    }
    if let Some(encoding) = encoding {
        metadata.set_item("encoding", encoding)?;
    }

    let payloads = PyList::empty(py);
    for stroke in strokes {
        let item = PyDict::new(py);
        item.set_item("move_x", stroke.move_x)?;
        item.set_item("move_y", stroke.move_y)?;
        item.set_item("directions", stroke.directions)?;
        item.set_item("form_class", stroke.form_class)?;
        item.set_item("symmetry", stroke.symmetry)?;
        item.set_item("direction_profile", stroke.direction_profile)?;
        item.set_item("spatial_frequency", stroke.spatial_frequency)?;
        item.set_item("drift_speed", stroke.drift_speed)?;
        item.set_item("frame_index", stroke.frame_index)?;
        item.set_item("delta_ms", stroke.delta_ms)?;
        payloads.append(item)?;
    }
    Ok((metadata.unbind(), payloads.unbind()))
}

#[pyfunction]
fn backend_info(py: Python<'_>) -> PyResult<Py<PyDict>> {
    let info = PyDict::new(py);
    info.set_item("backend", "rust")?;
    info.set_item("native", true)?;
    info.set_item("fallback_used", false)?;
    info.set_item("module_name", "zpe_mental_codec")?;
    info.set_item("crate_name", env!("CARGO_PKG_NAME"))?;
    info.set_item("version", env!("CARGO_PKG_VERSION"))?;
    Ok(info.unbind())
}

#[pymodule]
fn zpe_mental_codec(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pack_mental_strokes_payload, m)?)?;
    m.add_function(wrap_pyfunction!(unpack_mental_words_payload, m)?)?;
    m.add_function(wrap_pyfunction!(backend_info, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_strokes() -> Vec<MentalStrokePayload> {
        vec![
            MentalStrokePayload {
                move_x: 10,
                move_y: 10,
                directions: vec![0, 2, 4, 6],
                form_class: 2,
                symmetry: 2,
                direction_profile: PROFILE_COMPASS_8,
                spatial_frequency: 4,
                drift_speed: 1,
                frame_index: Some(3),
                delta_ms: 20,
            },
            MentalStrokePayload {
                move_x: 10,
                move_y: 10,
                directions: vec![0, 2, 4, 6, 8, 10],
                form_class: 2,
                symmetry: 3,
                direction_profile: PROFILE_D6_12,
                spatial_frequency: 4,
                drift_speed: 1,
                frame_index: Some(3),
                delta_ms: 40,
            },
        ]
    }

    #[test]
    fn rle_roundtrip_preserves_profiles_and_metadata() {
        let strokes = sample_strokes();
        let words = pack_mental_payloads(&strokes, EncodeOptions::default()).unwrap();
        let (encoding, frame_indices, delta_values, decoded) = unpack_mental_payloads(&words);
        assert_eq!(encoding.as_deref(), Some("rle"));
        assert_eq!(frame_indices, vec![Some(3), Some(3)]);
        assert_eq!(delta_values, vec![20, 40]);
        assert_eq!(decoded, strokes);
    }

    #[test]
    fn raw_roundtrip_preserves_profiles_and_metadata() {
        let strokes = sample_strokes();
        let words = pack_mental_payloads(
            &strokes,
            EncodeOptions {
                use_rle: false,
                ..EncodeOptions::default()
            },
        )
        .unwrap();
        let (encoding, _frame_indices, _delta_values, decoded) = unpack_mental_payloads(&words);
        assert_eq!(encoding.as_deref(), Some("raw"));
        assert_eq!(decoded, strokes);
    }
}
