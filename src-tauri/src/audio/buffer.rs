use std::collections::VecDeque;

/// Audio buffer for storing sample data
pub struct AudioBuffer {
    sample_rate: u32,
    channels: u16,
    data: VecDeque<f32>,
    max_size: usize,
    gain: f32,
}

impl AudioBuffer {
    /// Create a new audio buffer
    pub fn new(sample_rate: u32, channels: u16) -> Self {
        let max_size = (sample_rate as usize) * (channels as usize) * 10; // 10 seconds max
        Self {
            sample_rate,
            channels,
            data: VecDeque::with_capacity(max_size),
            max_size,
            gain: 1.0,
        }
    }

    /// Get the sample rate
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Get the number of channels
    pub fn channels(&self) -> u16 {
        self.channels
    }

    /// Get the current number of samples
    pub fn len(&self) -> usize {
        self.data.len() / (self.channels as usize)
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Push a stereo sample to the buffer
    pub fn push_stereo(&mut self, left: f32, right: f32) {
        if self.data.len() >= self.max_size {
            self.data.pop_front();
            self.data.pop_front();
        }
        self.data.push_back(left);
        self.data.push_back(right);
    }

    /// Push a mono sample to the buffer (will be duplicated to stereo)
    pub fn push_mono(&mut self, sample: f32) {
        self.push_stereo(sample, sample);
    }

    /// Push interleaved stereo samples
    pub fn push_interleaved(&mut self, left: f32, right: f32) {
        self.push_stereo(left, right);
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Read all samples as a Vec
    pub fn to_vec(&self) -> Vec<f32> {
        self.data.iter().map(|s| s * self.gain).collect()
    }

    /// Set the gain/volume multiplier
    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain.clamp(0.0, 2.0);
    }

    /// Get the current gain
    pub fn gain(&self) -> f32 {
        self.gain
    }

    /// Apply fade in/out
    pub fn apply_fade(&mut self, fade_in_samples: usize, fade_out_samples: usize) {
        let len = self.len();
        if len == 0 {
            return;
        }

        for i in 0..len {
            let mut multiplier = 1.0;

            // Fade in
            if i < fade_in_samples {
                multiplier *= i as f32 / fade_in_samples as f32;
            }

            // Fade out
            if i > len - fade_out_samples {
                let fade_out_pos = len - i;
                multiplier *= fade_out_pos as f32 / fade_out_samples as f32;
            }

            // Apply to both channels
            let idx = i * 2;
            if idx < self.data.len() {
                self.data[idx] *= multiplier;
            }
            if idx + 1 < self.data.len() {
                self.data[idx + 1] *= multiplier;
            }
        }
    }
}

impl Default for AudioBuffer {
    fn default() -> Self {
        Self::new(44100, 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_creation() {
        let buf = AudioBuffer::new(44100, 2);
        assert_eq!(buf.sample_rate(), 44100);
        assert_eq!(buf.channels(), 2);
        assert!(buf.is_empty());
    }

    #[test]
    fn test_buffer_push() {
        let mut buf = AudioBuffer::new(44100, 2);
        buf.push_stereo(0.5, 0.8);
        assert_eq!(buf.len(), 1);
    }
}
