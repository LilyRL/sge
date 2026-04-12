use std::{
    fs::File,
    io::{BufReader, Seek},
    path::Path,
    time::Duration,
};

pub use rodio;
use rodio::BitDepth;
use rodio::source::{AutomaticGainControlSettings, DitherAlgorithm, LimitSettings};
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Source, buffer::SamplesBuffer};
use sge_error_union::ErrorUnion;

sge_global::global!(AudioState, audio_state);
sge_macros::gen_ref_type!(Sound, SoundRef, sounds);

pub struct AudioState {
    handle: MixerDeviceSink,
}

pub struct Sound {
    buffer: SamplesBuffer,
}

impl Sound {
    pub fn play(&self) {
        get_audio_state().handle.mixer().add(self.buffer.clone());
    }

    pub fn play_ex(&self) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
        SoundBuilder {
            source: self.buffer.clone(),
        }
    }
}

#[derive(ErrorUnion, Debug)]
pub enum SoundLoadError {
    Io(std::io::Error),
    Decoding(rodio::decoder::DecoderError),
}

pub fn load_sound(path: impl AsRef<Path>) -> Result<SoundRef, SoundLoadError> {
    let file = File::open(path)?;
    let decoder = Decoder::try_from(file)?;
    Ok(sound_from_decoder(decoder))
}

pub fn load_sound_from_bytes(bytes: impl Into<Vec<u8>>) -> Result<SoundRef, SoundLoadError> {
    let cursor = std::io::Cursor::new(bytes.into());
    let decoder = Decoder::new(BufReader::new(cursor))?;
    Ok(sound_from_decoder(decoder))
}

fn sound_from_decoder(
    decoder: Decoder<impl std::io::Read + Send + Seek + Sync + 'static>,
) -> SoundRef {
    let buffer = decoder.record();
    Sound { buffer }.create()
}

pub fn play_sound(sound: SoundRef) {
    sound.get().play();
}

pub fn play_sound_ex(sound: SoundRef) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
    sound.get().play_ex()
}

#[derive(ErrorUnion, Debug)]
pub enum AudioInitError {
    Device(rodio::DeviceSinkError),
}

pub fn init() -> Result<(), AudioInitError> {
    let handle = DeviceSinkBuilder::open_default_sink()?;
    set_audio_state(AudioState { handle });
    init_sounds_storage();
    Ok(())
}

pub struct SoundBuilder<S: Source<Item = f32> + Send + 'static> {
    source: S,
}

impl<S: Source<Item = f32> + Send + 'static> SoundBuilder<S> {
    pub fn start(self) {
        get_audio_state().handle.mixer().add(self.source);
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn volume(self, factor: f32) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
        SoundBuilder {
            source: self.source.amplify(factor),
        }
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn volume_db(self, db: f32) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
        SoundBuilder {
            source: self.source.amplify_decibel(db),
        }
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn volume_normalized(
        self,
        value: f32,
    ) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
        SoundBuilder {
            source: self.source.amplify_normalized(value),
        }
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn speed(self, ratio: f32) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
        SoundBuilder {
            source: self.source.speed(ratio),
        }
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn fade_in(
        self,
        duration: Duration,
    ) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
        SoundBuilder {
            source: self.source.fade_in(duration),
        }
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn fade_out(
        self,
        duration: Duration,
    ) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
        SoundBuilder {
            source: self.source.fade_out(duration),
        }
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn repeat(self) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
        SoundBuilder {
            source: self.source.repeat_infinite(),
        }
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn delay(
        self,
        duration: Duration,
    ) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
        SoundBuilder {
            source: self.source.delay(duration),
        }
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn take(
        self,
        duration: Duration,
    ) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
        SoundBuilder {
            source: self.source.take_duration(duration),
        }
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn skip(
        self,
        duration: Duration,
    ) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
        SoundBuilder {
            source: self.source.skip_duration(duration),
        }
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn low_pass(self, freq: u32) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
        SoundBuilder {
            source: self.source.low_pass(freq),
        }
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn high_pass(self, freq: u32) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
        SoundBuilder {
            source: self.source.high_pass(freq),
        }
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn low_pass_q(
        self,
        freq: u32,
        q: f32,
    ) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
        SoundBuilder {
            source: self.source.low_pass_with_q(freq, q),
        }
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn high_pass_q(
        self,
        freq: u32,
        q: f32,
    ) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
        SoundBuilder {
            source: self.source.high_pass_with_q(freq, q),
        }
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn distortion(
        self,
        gain: f32,
        threshold: f32,
    ) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
        SoundBuilder {
            source: self.source.distortion(gain, threshold),
        }
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn linear_gain_ramp(
        self,
        duration: Duration,
        start: f32,
        end: f32,
        clamp_end: bool,
    ) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
        SoundBuilder {
            source: self
                .source
                .linear_gain_ramp(duration, start, end, clamp_end),
        }
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn limit(
        self,
        settings: LimitSettings,
    ) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
        SoundBuilder {
            source: self.source.limit(settings),
        }
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn agc(
        self,
        settings: AutomaticGainControlSettings,
    ) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
        SoundBuilder {
            source: self.source.automatic_gain_control(settings),
        }
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn dither(
        self,
        target_bits: BitDepth,
        algorithm: DitherAlgorithm,
    ) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
        SoundBuilder {
            source: self.source.dither(target_bits, algorithm),
        }
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn mix_with<O>(
        self,
        other: SoundBuilder<O>,
    ) -> SoundBuilder<impl Source<Item = f32> + Send + 'static>
    where
        O: Source<Item = f32> + Send + 'static,
    {
        SoundBuilder {
            source: self.source.mix(other.source),
        }
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn crossfade_with<O>(
        self,
        other: SoundBuilder<O>,
        duration: Duration,
    ) -> SoundBuilder<impl Source<Item = f32> + Send + 'static>
    where
        O: Source<Item = f32> + Send + 'static,
    {
        SoundBuilder {
            source: self.source.take_crossfade_with(other.source, duration),
        }
    }

    #[must_use = "Call .start() after adding effects to play the sound"]
    pub fn reverb(
        self,
        duration: Duration,
        amplitude: f32,
    ) -> SoundBuilder<impl Source<Item = f32> + Send + 'static> {
        SoundBuilder {
            source: self.source.buffered().reverb(duration, amplitude),
        }
    }
}

#[macro_export]
macro_rules! include_sound {
    ($path: literal) => {
        ::sge::prelude::load_sound_from_bytes(include_bytes!($path))
    };
}
