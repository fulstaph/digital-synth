const TWO_PI: f32 = std::f32::consts::TAU;

#[derive(Debug, Clone)]
pub struct SineGenerator {
    frequency_hz: f32,
    amplitude: f32,
    sample_rate: f32,
    phase: f32,
}

impl SineGenerator {
    pub fn new(frequency_hz: f32, amplitude: f32, sample_rate: f32) -> Self {
        Self {
            frequency_hz,
            amplitude,
            sample_rate,
            phase: 0.0,
        }
    }

    pub fn fill_mono(&mut self, output: &mut [f32]) {
        if self.frequency_hz <= 0.0 || self.sample_rate <= 0.0 {
            output.fill(0.0);
            return;
        }

        let phase_increment = TWO_PI * self.frequency_hz / self.sample_rate;

        for sample in output {
            *sample = self.phase.sin() * self.amplitude;
            self.phase = (self.phase + phase_increment).rem_euclid(TWO_PI);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fills_requested_number_of_samples() {
        let mut generator = SineGenerator::new(440.0, 0.2, 44_100.0);
        let mut output = [0.0; 128];

        generator.fill_mono(&mut output);

        assert_eq!(output.len(), 128);
        assert!(output.iter().any(|sample| *sample != 0.0));
    }

    #[test]
    fn keeps_samples_within_amplitude_bounds() {
        let mut generator = SineGenerator::new(440.0, 0.2, 44_100.0);
        let mut output = [0.0; 512];

        generator.fill_mono(&mut output);

        for sample in output {
            assert!(
                (-0.2..=0.2).contains(&sample),
                "sample {sample} exceeded amplitude bounds"
            );
        }
    }

    #[test]
    fn keeps_phase_continuous_across_buffers() {
        let mut uninterrupted = SineGenerator::new(440.0, 0.2, 44_100.0);
        let mut uninterrupted_output = [0.0; 128];
        uninterrupted.fill_mono(&mut uninterrupted_output);

        let mut chunked = SineGenerator::new(440.0, 0.2, 44_100.0);
        let mut first_chunk = [0.0; 64];
        let mut second_chunk = [0.0; 64];
        chunked.fill_mono(&mut first_chunk);
        chunked.fill_mono(&mut second_chunk);

        assert_eq!(&uninterrupted_output[..64], &first_chunk);
        assert_eq!(&uninterrupted_output[64..], &second_chunk);
    }

    #[test]
    fn writes_silence_for_non_positive_frequencies() {
        let mut zero_frequency = SineGenerator::new(0.0, 0.2, 44_100.0);
        let mut negative_frequency = SineGenerator::new(-440.0, 0.2, 44_100.0);
        let mut zero_output = [1.0; 32];
        let mut negative_output = [1.0; 32];

        zero_frequency.fill_mono(&mut zero_output);
        negative_frequency.fill_mono(&mut negative_output);

        assert_eq!(zero_output, [0.0; 32]);
        assert_eq!(negative_output, [0.0; 32]);
    }
}
