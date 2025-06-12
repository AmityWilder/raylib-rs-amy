use super::*;

// Audio device management functions

/// Initialize audio device and context
#[inline]
pub fn init_audio_device() {
    unsafe {
        sys::InitAudioDevice();
    }
}

/// Close the audio device and context
#[inline]
pub fn close_audio_device() {
    unsafe {
        sys::CloseAudioDevice()
    }
}

/// Check if audio device has been initialized successfully
#[inline]
pub fn is_audio_device_ready() -> bool {
    unsafe {
        sys::IsAudioDeviceReady()
    }
}

/// Set master volume (listener)
#[inline]
pub fn set_master_volume(
    volume: f32,
) {
    unsafe {
        sys::SetMasterVolume(
            volume,
        );
    }
}

/// Get master volume (listener)
#[inline]
pub fn get_master_volume() -> f32 {
    unsafe {
        sys::GetMasterVolume()
    }
}

// Wave/Sound loading/unloading functions

/// Load wave data from file
#[inline]
pub fn load_wave(
    file_name: &CStr,
) -> sys::Wave {
    unsafe {
        sys::LoadWave(
            file_name,
        )
    }
}

/// Load wave from memory buffer, fileType refers to extension: i.e. ".wav"
/// WARNING: File extension must be provided in lower-case
#[inline]
pub fn load_wave_from_memory(
    file_type: &CStr,
    file_data: &[u8],
) -> sys::Wave {
    unsafe {
        sys::LoadWaveFromMemory(
            file_type.as_ptr(),
            file_data.as_ptr(),
            file_data.len().try_into().unwrap(),
        )
    }
}

/// Checks if wave data is valid (data loaded and parameters)
#[inline]
pub fn is_wave_valid(
    wave: sys::Wave,
) -> bool {
    unsafe {
        sys::IsWaveValid(
            wave,
        )
    }
}

/// Load sound from file
///
/// NOTE: The entire file is loaded to memory to be played (no-streaming)
#[inline]
pub fn load_sound(
    file_name: &CStr,
) -> sys::Sound {
    unsafe {
        sys::LoadSound(
            file_name,
        )
    }
}

/// Load sound from wave data
///
/// NOTE: Wave data must be unallocated manually
#[inline]
pub fn load_sound_from_wave(
    wave: sys::Wave,
) -> sys::Sound {
    unsafe {
        sys::LoadSoundFromWave(
            wave,
        )
    }
}

/// Create a new sound that shares the same sample data as the source sound, does not own the sound data
///
/// NOTE: Wave data must be unallocated manually and will be shared across all clones
#[inline]
pub fn load_sound_alias(
    source: sys::Sound,
) -> sys::Sound {
    unsafe {
        sys::LoadSoundAlias(
            source,
        )
    }
}

/// Checks if a sound is valid (data loaded and buffers initialized)
#[inline]
pub fn is_sound_valid(
    sound: sys::Sound,
) -> bool {
    unsafe {
        sys::IsSoundValid(
            sound,
        )
    }
}

/// Update sound buffer with new data
#[inline]
pub fn update_sound(
    sound: sys::Sound,
    data: &[u8],
    sample_count: usize,
) {
    unsafe {
        sys::UpdateSound(
            sound,
            data.as_ptr().cast(),
            sample_count.try_into(),
        );
    }
}

/// Unload wave data
#[inline]
pub fn unload_wave(
    wave: sys::Wave,
) {
    unsafe {
        sys::UnloadWave(
            wave,
        );
    }
}

/// Unload sound
#[inline]
pub fn unload_sound(
    sound: sys::Sound,
) {
    unsafe {
        sys::UnloadSound(
            sound,
        );
    }
}

/// Unload a sound alias (does not deallocate sample data)
#[inline]
pub fn unload_sound_alias(
    alias: sys::Sound,
) {
    unsafe {
        sys::UnloadSoundAlias(
            alias,
        );
    }
}

/// Export wave data to file, returns true on success
#[inline]
pub fn export_wave(
    wave: sys::Wave,
    file_name: &CStr,
) -> bool {
    unsafe {
        sys::ExportWave(
            wave,
            file_name.as_ptr(),
        )
    }
}

/// Export wave sample data to code (.h), returns true on success
#[inline]
pub fn export_wave_as_code(
    wave: sys::Wave,
    file_name: &CStr,
) -> Result<(), ()> {
    match unsafe {
        sys::ExportWaveAsCode(
            wave,
            file_name.as_ptr(),
        )
    } {
        true => Ok(()),
        false => Err(()),
    }
}

// Wave/Sound management functions

/// Play a sound
#[inline]
pub fn play_sound(
    sound: sys::Sound,
) {
    unsafe {
        sys::PlaySound(
            sound,
        );
    }
}

/// Stop playing a sound
#[inline]
pub fn stop_sound(
    sound: sys::Sound,
) {
    unsafe {
        sys::StopSound(
            sound,
        );
    }
}

/// Pause a sound
#[inline]
pub fn pause_sound(
    sound: sys::Sound,
) {
    unsafe {
        sys::PauseSound(
            sound
        );
    }
}

/// Resume a paused sound
#[inline]
pub fn resume_sound(
    sound: sys::Sound,
) {
    unsafe {
        sys::ResumeSound(
            sound
        );
    }
}

/// Check if a sound is currently playing
#[inline]
pub fn is_sound_playing(
    sound: sys::Sound,
) -> bool {
    unsafe {
        sys::IsSoundPlaying(
            sound,
        )
    }
}

/// Set volume for a sound (1.0 is max level)
#[inline]
pub fn set_sound_volume(
    sound: sys::Sound,
    volume: f32,
) {
    unsafe {
        sys::SetSoundVolume(
            sound,
            volume,
        );
    }
}

/// Set pitch for a sound (1.0 is base level)
#[inline]
pub fn set_sound_pitch(
    sound: sys::Sound,
    pitch: f32,
) {
    unsafe {
        sys::SetSoundPitch(
            sound,
            pitch,
        );
    }
}

/// Set pan for a sound (0.5 is center)
#[inline]
pub fn set_sound_pan(
    sound: sys::Sound,
    pan: f32,
) {
    unsafe {
        sys::SetSoundPan(
            sound,
            pan,
        );
    }
}

/// Copy a wave to a new wave
#[inline]
pub fn wave_copy(
    wave: sys::Wave,
) -> sys::Wave {
    unsafe {
        sys::WaveCopy(
            wave,
        )
    }
}

/// Crop a wave to defined frames range
#[inline]
pub fn wave_crop(
    wave: &mut sys::Wave,
    init_frame: u32,
    final_frame: u32,
) {
    unsafe {
        sys::WaveCrop(
            wave,
            init_frame.try_into().unwrap(),
            final_frame.try_into().unwrap(),
        );
    }
}

/// Convert wave data to desired format
#[inline]
pub fn wave_format(
    wave: &mut sys::Wave,
    sample_rate: u32,
    sample_size: u32,
    channels: u32,
) {
    unsafe {
        sys::WaveFormat(
            wave,
            sample_rate.try_into().unwrap(),
            sample_size.try_into().unwrap(),
            channels.try_into().unwrap(),
        );
    }
}

pub struct WaveSamples(NonNull<[f32]>);

impl Deref for WaveSamples {
    type Target = [f32];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            self.0.as_ref()
        }
    }
}

impl DerefMut for WaveSamples {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            self.0.as_mut()
        }
    }
}

impl WaveSamples {
    unsafe fn new(data: *mut f32, len: u32) -> Option<Self> {
        if !data.is_null() {
            Some(Self(unsafe {
                NonNull::new_unchecked(std::ptr::slice_from_raw_parts_mut(data, len.try_into().unwrap()))
            }))
        } else {
            None
        }
    }
}

/// Load samples data from wave as a 32bit float data array
///
/// NOTE 1: Returned sample values are normalized to range [-1..1]
///
/// NOTE 2: Sample data allocated should be freed with UnloadWaveSamples()
#[inline]
pub fn load_wave_samples(
    wave: sys::Wave,
) -> WaveSamples {
    unsafe {
        WaveSamples::new(
            sys::LoadWaveSamples(
                wave,
            ),
            wave.frameCount*wave.channels,
        )
    }
}

/// Unload samples data loaded with LoadWaveSamples()
#[inline]
pub fn unload_wave_samples(
    samples: WaveSamples,
) {
    unsafe {
        sys::UnloadWaveSamples(
            samples.0.as_ptr(),
        )
    }
}

// Music management functions

/// Load music stream from file
#[inline]
pub fn load_music_stream(
    file_name: &CStr,
) -> sys::Music {
    unsafe {
        sys::LoadMusicStream(
            file_name.as_ptr()
        )
    }
}

/// Load music stream from memory buffer, `file_type` refers to extension: i.e. ".wav"
///
/// WARNING: File extension must be provided in lower-case
#[inline]
pub fn load_music_stream_from_memory(
    file_type: &CStr,
    data: &[u8],
) -> sys::Music {
    unsafe {
        sys::LoadMusicStreamFromMemory(
            file_type.as_ptr(),
            data.as_ptr(),
            data.len().try_into().unwrap(),
        )
    }
}

/// Checks if a music stream is valid (context and buffers initialized)
#[inline]
pub fn is_music_valid(
    music: sys::Music,
) -> bool {
    unsafe {
        sys::IsMusicValid(
            music,
        )
    }
}

/// Unload music stream
#[inline]
pub fn unload_music_stream(
    music: sys::Music,
) {
    unsafe {
        sys::UnloadMusicStream(
            music,
        );
    }
}

/// Start music playing (open stream) from beginning
#[inline]
pub fn play_music_stream(
    music: sys::Music,
) {
    unsafe {
        sys::PlayMusicStream(
            music,
        );
    }
}

/// Check if music is playing
#[inline]
pub fn is_music_stream_playing(
    music: sys::Music,
) -> bool {
    unsafe {
        sys::IsMusicStreamPlaying(
            music
        )
    }
}

/// Updates buffers for music streaming
///
/// Update (re-fill) music buffers if data already processed
#[inline]
pub fn update_music_stream(
    music: sys::Music,
) {
    unsafe {
        sys::UpdateMusicStream(
            music,
        );
    }
}

/// Stop music playing (close stream)
#[inline]
pub fn stop_music_stream(
    music: sys::Music,
) {
    unsafe {
        sys::StopMusicStream(
            music,
        );
    }
}

/// Pause music playing
#[inline]
pub fn pause_music_stream(
    music: sys::Music,
) {
    unsafe {
        sys::PauseMusicStream(
            music,
        );
    }
}

/// Resume playing paused music
#[inline]
pub fn resume_music_stream(
    music: sys::Music,
) {
    unsafe {
        sys::ResumeMusicStream(
            music,
        );
    }
}

/// Seek music to a position (in seconds)
#[inline]
pub fn seek_music_stream(
    music: sys::Music,
    position: f32,
) {
    unsafe {
        sys::SeekMusicStream(
            music,
            position,
        );
    }
}

/// Set volume for music (1.0 is max level)
#[inline]
pub fn set_music_volume(
    music: sys::Music,
    volume: f32,
) {
    unsafe {
        sys::SetMusicVolume(
            music,
            volume,
        );
    }
}

/// Set pitch for a music (1.0 is base level)
#[inline]
pub fn set_music_pitch(
    music: sys::Music,
    pitch: f32,
) {
    unsafe {
        sys::SetMusicPitch(
            music,
            pitch,
        );
    }
}

/// Set pan for a music (0.5 is center)
#[inline]
pub fn set_music_pan(
    music: sys::Music,
    pan: f32,
) {
    unsafe {
        sys::SetMusicPan(
            music,
            pan,
        );
    }
}

/// Get music time length (in seconds)
#[inline]
pub fn get_music_time_length(
    music: sys::Music,
) -> f32 {
    unsafe {
        sys::GetMusicTimeLength(
            music,
        )
    }
}

/// Get music time length
#[inline]
pub fn get_music_duration_length(
    music: sys::Music,
) -> Duration {
    Duration::from_secs_f32(get_music_time_length(music))
}

/// Get current music time played (in seconds)
#[inline]
pub fn get_music_time_played(
    music: sys::Music,
) -> f32 {
    unsafe {
        sys::GetMusicTimePlayed(
            music,
        )
    }
}

/// Get current music time played
#[inline]
pub fn get_music_duration_played(
    music: sys::Music,
) -> Duration {
    Duration::from_secs_f32(get_music_time_played(music))
}

// AudioStream management functions

/// Load audio stream (to stream raw audio pcm data)
#[inline]
pub fn load_audio_stream(
    sample_rate: u32,
    sample_size: u32,
    channels: u32,
) -> sys::AudioStream {
    unsafe {
        sys::LoadAudioStream(
            sample_rate,
            sample_size,
            channels,
        )
    }
}

/// Checks if an audio stream is valid (buffers initialized)
#[inline]
pub fn is_audio_stream_valid(
    stream: sys::AudioStream,
) -> bool {
    unsafe {
        sys::IsAudioStreamValid(
            stream,
        )
    }
}

/// Unload audio stream and free memory
#[inline]
pub fn unload_audio_stream(
    stream: sys::AudioStream,
) {
    unsafe {
        sys::UnloadAudioStream(
            stream,
        );
    }
}

/// Update audio stream buffers with data
///
/// NOTE 1: Only updates one buffer of the stream source: dequeue -> update -> queue
///
/// NOTE 2: To dequeue a buffer it needs to be processed: [`is_audio_stream_processed()`]
#[inline]
pub fn update_audio_stream(
    stream: sys::AudioStream,
    data: &[u8],
    frame_count: usize,
) {
    unsafe {
        sys::UpdateAudioStream(
            stream,
            data.as_ptr().cast(),
            frame_count.try_into().unwrap(),
        );
    }
}

/// Check if any audio stream buffers requires refill
#[inline]
pub fn is_audio_stream_processed(
    stream: sys::AudioStream,
) -> bool {
    unsafe {
        sys::IsAudioStreamProcessed(
            stream,
        )
    }
}

/// Play audio stream
#[inline]
pub fn play_audio_stream(
    stream: sys::AudioStream,
) {
    unsafe {
        sys::PlayAudioStream(
            stream,
        );
    }
}

/// Pause audio stream
#[inline]
pub fn pause_audio_stream(
    stream: sys::AudioStream,
) {
    unsafe {
        sys::PauseAudioStream(
            stream,
        );
    }
}

/// Resume audio stream
#[inline]
pub fn resume_audio_stream(
    stream: sys::AudioStream,
) {
    unsafe {
        sys::ResumeAudioStream(
            stream,
        );
    }
}

/// Check if audio stream is playing
#[inline]
pub fn is_audio_stream_playing(
    stream: sys::AudioStream,
) -> bool {
    unsafe {
        sys::IsAudioStreamPlaying(
            stream,
        )
    }
}

/// Stop audio stream
#[inline]
pub fn stop_audio_stream(
    stream: sys::AudioStream,
) {
    unsafe {
        sys::StopAudioStream(
            stream,
        );
    }
}

/// Set volume for audio stream (1.0 is max level)
#[inline]
pub fn set_audio_stream_volume(
    stream: sys::AudioStream,
    volume: f32,
) {
    unsafe {
        sys::SetAudioStreamVolume(
            stream,
            volume,
        );
    }
}

/// Set pitch for audio stream (1.0 is base level)
#[inline]
pub fn set_audio_stream_pitch(
    stream: sys::AudioStream,
    pitch: f32,
) {
    unsafe {
        sys::SetAudioStreamPitch(
            stream,
            pitch,
        );
    }
}

/// Set pan for audio stream (0.5 is centered)
#[inline]
pub fn set_audio_stream_pan(
    stream: sys::AudioStream,
    pan: f32,
) {
    unsafe {
        sys::SetAudioStreamPan(
            stream,
            pan,
        );
    }
}

/// Default size for new audio streams
#[inline]
pub fn set_audio_stream_buffer_size_default(
    size: usize,
) {
    unsafe {
        sys::SetAudioStreamBufferSizeDefault(
            size.try_into().unwrap(),
        );
    }
}

/// Audio thread callback to request new data
#[inline]
pub fn set_audio_stream_callback(
    stream: sys::AudioStream,
    callback: sys::AudioCallback,
) {
    unsafe {
        sys::SetAudioStreamCallback(
            stream,
            callback,
        );
    }
}

/// Attach audio stream processor to stream, receives frames x 2 samples as 'float' (stereo)
///
/// Contrary to buffers, the order of processors is important
///
/// The new processor must be added at the end. As there aren't supposed to be a lot of processors attached to
/// a given stream, we iterate through the list to find the end. That way we don't need a pointer to the last element
#[inline]
pub fn attach_audio_stream_processor(
    stream: sys::AudioStream,
    processor: sys::AudioCallback,
) {
    unsafe {
        sys::AttachAudioStreamProcessor(
            stream,
            processor,
        );
    }
}

/// Detach audio stream processor from stream (Remove processor from audio stream)
#[inline]
pub fn detach_audio_stream_processor(
    stream: sys::AudioStream,
    processor: sys::AudioCallback,
) {
    unsafe {
        sys::DetachAudioStreamProcessor(
            stream,
            processor,
        );
    }
}

/// Attach audio stream processor to the entire audio pipeline, receives frames x 2 samples as 'float' (stereo)
///
// Order of processors is important

// Works the same way as {Attach,Detach}AudioStreamProcessor() functions, except
// these two work on the already mixed output just before sending it to the sound hardware
#[inline]
pub fn attach_audio_mixed_processor(
    processor: sys::AudioCallback,
) {
    unsafe {
        sys::AttachAudioMixedProcessor(
            processor,
        );
    }
}

/// Detach audio stream processor from the entire audio pipeline (Remove processor from audio pipeline)
#[inline]
pub fn detach_audio_mixed_processor(
    processor: sys::AudioCallback,
) {
    unsafe {
        sys::DetachAudioMixedProcessor(
            processor,
        );
    }
}
