use super::*;

/// `charset[0]` is 'A', so class 1 decodes to 'A', class 2 to 'B', and class 4 (len + 1) is the
/// appended space.
fn charset() -> Vec<char> {
    vec!['A', 'B', 'C']
}

/// A one-hot probability sequence: `classes[t]` wins at timestep `t`.
fn logits(classes: &[usize]) -> NdTensor<f32, 3> {
    let n_classes = charset().len() + 2;
    let mut data = vec![0.0f32; classes.len() * n_classes];
    for (t, c) in classes.iter().enumerate() {
        data[t * n_classes + c] = 1.0;
    }
    NdTensor::from_data([1, classes.len(), n_classes], data)
}

fn decode(classes: &[usize]) -> String {
    let l = logits(classes);
    ctc_greedy(l.view(), 0, &charset()).text
}

#[test]
fn a_class_maps_one_past_the_blank() {
    assert_eq!(decode(&[1, 2, 3]), "ABC");
}

#[test]
fn the_blank_emits_nothing() {
    assert_eq!(decode(&[0, 0, 0]), "");
}

/// The whole point of CTC: a character held across timesteps is one character.
#[test]
fn a_repeated_class_collapses_to_one_character() {
    assert_eq!(decode(&[1, 1, 1, 2, 2]), "AB");
}

/// ...but a blank between two runs of the same class means two characters.
#[test]
fn a_blank_separates_a_doubled_letter() {
    assert_eq!(decode(&[1, 1, 0, 1, 1]), "AA");
}

#[test]
fn the_last_class_is_the_appended_space() {
    // charset.len() + 1 == 4.
    assert_eq!(decode(&[1, 4, 2]), "A B");
}

#[test]
fn leading_and_trailing_space_is_trimmed() {
    assert_eq!(decode(&[4, 1, 4]), "A");
}

#[test]
fn confidence_is_the_mean_of_the_emitting_steps() {
    let n_classes = charset().len() + 2;
    // Two emissions at 0.8 and 0.6, plus a blank that must not count.
    let mut data = vec![0.0f32; 3 * n_classes];
    data[1] = 0.8;
    data[n_classes] = 0.9; // blank wins at t=1
    data[2 * n_classes + 2] = 0.6;
    let l = NdTensor::from_data([1, 3, n_classes], data);
    let out = ctc_greedy(l.view(), 0, &charset());
    assert_eq!(out.text, "AB");
    assert!((out.confidence - 0.7).abs() < 1e-6, "confidence {}", out.confidence);
}

#[test]
fn an_empty_result_has_zero_confidence() {
    let out = ctc_greedy(logits(&[0, 0]).view(), 0, &charset());
    assert_eq!(out.confidence, 0.0);
}

/// An out-of-range class must be dropped, not panic. Only reachable on a charset/model mismatch,
/// which is exactly when a panic would be worst.
#[test]
fn a_class_beyond_the_charset_is_ignored() {
    let n_classes = charset().len() + 2;
    let mut data = vec![0.0f32; n_classes];
    data[n_classes - 1] = 1.0;
    let l = NdTensor::from_data([1, 1, n_classes], data);
    // The last valid class is the space, so this decodes to a space and then trims to empty.
    assert_eq!(ctc_greedy(l.view(), 0, &charset()).text, "");
}
