use super::sys;

// Audio device management functions

/// Initialize audio device and context
#[inline]
pub fn InitAudioDevice();

/// Close the audio device and context
#[inline]
pub fn CloseAudioDevice();

/// Check if audio device has been initialized successfully
#[inline]
pub fn IsAudioDeviceReady() -> bool;

/// Set master volume (listener)
#[inline]
pub fn SetMasterVolume(
    volume: f32,
);

/// Get master volume (listener)
#[inline]
pub fn GetMasterVolume() -> f32;

// Wave/Sound loading/unloading functions

/// Load wave data from file
#[inline]
pub fn LoadWave(
    fileName: *const ::std::os::raw::c_char,
) -> Wave;

/// Load wave from memory buffer, fileType refers to extension: i.e. ".wav"
/// WARNING: File extension must be provided in lower-case
#[inline]
pub fn LoadWaveFromMemory(
    fileType: *const ::std::os::raw::c_char,
    fileData: *const u8,
    dataSize: i32,
) -> Wave;

/// Checks if wave data is valid (data loaded and parameters)
#[inline]
pub fn IsWaveValid(
    wave: Wave,
) -> bool;

/// Load sound from file
///
/// NOTE: The entire file is loaded to memory to be played (no-streaming)
#[inline]
pub fn LoadSound(
    fileName: *const ::std::os::raw::c_char,
) -> Sound;

/// Load sound from wave data
///
/// NOTE: Wave data must be unallocated manually
#[inline]
pub fn LoadSoundFromWave(
    wave: Wave,
) -> Sound;

/// Create a new sound that shares the same sample data as the source sound, does not own the sound data
///
/// NOTE: Wave data must be unallocated manually and will be shared across all clones
#[inline]
pub fn LoadSoundAlias(
    source: Sound,
) -> Sound;

/// Checks if a sound is valid (data loaded and buffers initialized)
#[inline]
pub fn IsSoundValid(
    sound: Sound,
) -> bool;

/// Update sound buffer with new data
#[inline]
pub fn UpdateSound(
    sound: Sound,
    data: *const ::std::os::raw::c_void,
    sampleCount: i32,
);

/// Unload wave data
#[inline]
pub fn UnloadWave(
    wave: Wave,
);

/// Unload sound
#[inline]
pub fn UnloadSound(
    sound: Sound,
);

/// Unload a sound alias (does not deallocate sample data)
#[inline]
pub fn UnloadSoundAlias(
    alias: Sound,
);

/// Export wave data to file, returns true on success
#[inline]
pub fn ExportWave(
    wave: Wave,
    fileName: *const ::std::os::raw::c_char,
) -> bool;

/// Export wave sample data to code (.h), returns true on success
#[inline]
pub fn ExportWaveAsCode(
    wave: Wave,
    fileName: *const ::std::os::raw::c_char,
) -> bool;

// Wave/Sound management functions

/// Play a sound
#[inline]
pub fn PlaySound(
    sound: Sound,
);

/// Stop playing a sound
#[inline]
pub fn StopSound(
    sound: Sound,
);

/// Pause a sound
#[inline]
pub fn PauseSound(
    sound: Sound,
);

/// Resume a paused sound
#[inline]
pub fn ResumeSound(
    sound: Sound,
);

/// Check if a sound is currently playing
#[inline]
pub fn IsSoundPlaying(
    sound: Sound,
) -> bool;

/// Set volume for a sound (1.0 is max level)
#[inline]
pub fn SetSoundVolume(
    sound: Sound,
    volume: f32,
);

/// Set pitch for a sound (1.0 is base level)
#[inline]
pub fn SetSoundPitch(
    sound: Sound,
    pitch: f32,
);

/// Set pan for a sound (0.5 is center)
#[inline]
pub fn SetSoundPan(
    sound: Sound,
    pan: f32,
);

/// Copy a wave to a new wave
#[inline]
pub fn WaveCopy(
    wave: Wave,
) -> Wave;

/// Crop a wave to defined frames range
#[inline]
pub fn WaveCrop(
    wave: *mut Wave,
    initFrame: i32,
    finalFrame: i32,
);

/// Convert wave data to desired format
#[inline]
pub fn WaveFormat(
    wave: *mut Wave,
    sampleRate: i32,
    sampleSize: i32,
    channels: i32,
);

/// Load samples data from wave as a 32bit float data array
///
/// NOTE 1: Returned sample values are normalized to range [-1..1]
///
/// NOTE 2: Sample data allocated should be freed with UnloadWaveSamples()
#[inline]
pub fn LoadWaveSamples(
    wave: Wave,
) -> *mut f32;

/// Unload samples data loaded with LoadWaveSamples()
#[inline]
pub fn UnloadWaveSamples(
    samples: *mut f32,
);

// Music management functions

/// Load music stream from file
#[inline]
pub fn LoadMusicStream(
    fileName: *const ::std::os::raw::c_char,
) -> Music;

/// Load music stream from memory buffer, fileType refers to extension: i.e. ".wav"
///
/// WARNING: File extension must be provided in lower-case
#[inline]
pub fn LoadMusicStreamFromMemory(
    fileType: *const ::std::os::raw::c_char,
    data: *const u8,
    dataSize: i32,
) -> Music;

/// Checks if a music stream is valid (context and buffers initialized)
#[inline]
pub fn IsMusicValid(
    music: Music,
) -> bool;

/// Unload music stream
#[inline]
pub fn UnloadMusicStream(
    music: Music,
);

/// Start music playing (open stream) from beginning
#[inline]
pub fn PlayMusicStream(
    music: Music,
);

/// Check if music is playing
#[inline]
pub fn IsMusicStreamPlaying(
    music: Music,
) -> bool;

/// Updates buffers for music streaming
///
/// Update (re-fill) music buffers if data already processed
#[inline]
pub fn UpdateMusicStream(
    music: Music,
);

/// Stop music playing (close stream)
#[inline]
pub fn StopMusicStream(
    music: Music,
);

/// Pause music playing
#[inline]
pub fn PauseMusicStream(
    music: Music,
);

/// Resume playing paused music
#[inline]
pub fn ResumeMusicStream(
    music: Music,
);

/// Seek music to a position (in seconds)
#[inline]
pub fn SeekMusicStream(
    music: Music,
    position: f32,
);

/// Set volume for music (1.0 is max level)
#[inline]
pub fn SetMusicVolume(
    music: Music,
    volume: f32,
);

/// Set pitch for a music (1.0 is base level)
#[inline]
pub fn SetMusicPitch(
    music: Music,
    pitch: f32,
);

/// Set pan for a music (0.5 is center)
#[inline]
pub fn SetMusicPan(
    music: Music,
    pan: f32,
);

/// Get music time length (in seconds)
#[inline]
pub fn GetMusicTimeLength(
    music: Music,
) -> f32;

/// Get current music time played (in seconds)
#[inline]
pub fn GetMusicTimePlayed(
    music: Music,
) -> f32;

// AudioStream management functions

/// Load audio stream (to stream raw audio pcm data)
#[inline]
pub fn LoadAudioStream(
    sampleRate: u32,
    sampleSize: u32,
    channels: u32,
) -> AudioStream;

/// Checks if an audio stream is valid (buffers initialized)
#[inline]
pub fn IsAudioStreamValid(
    stream: AudioStream,
) -> bool;

/// Unload audio stream and free memory
#[inline]
pub fn UnloadAudioStream(
    stream: AudioStream,
);

/// Update audio stream buffers with data
///
/// NOTE 1: Only updates one buffer of the stream source: dequeue -> update -> queue
///
/// NOTE 2: To dequeue a buffer it needs to be processed: IsAudioStreamProcessed()
#[inline]
pub fn UpdateAudioStream(
    stream: AudioStream,
    data: *const ::std::os::raw::c_void,
    frameCount: i32,
);

/// Check if any audio stream buffers requires refill
#[inline]
pub fn IsAudioStreamProcessed(
    stream: AudioStream,
) -> bool;

/// Play audio stream
#[inline]
pub fn PlayAudioStream(
    stream: AudioStream,
);

/// Pause audio stream
#[inline]
pub fn PauseAudioStream(
    stream: AudioStream,
);

/// Resume audio stream
#[inline]
pub fn ResumeAudioStream(
    stream: AudioStream,
);

/// Check if audio stream is playing
#[inline]
pub fn IsAudioStreamPlaying(
    stream: AudioStream,
) -> bool;

/// Stop audio stream
#[inline]
pub fn StopAudioStream(
    stream: AudioStream,
);

/// Set volume for audio stream (1.0 is max level)
#[inline]
pub fn SetAudioStreamVolume(
    stream: AudioStream,
    volume: f32,
);

/// Set pitch for audio stream (1.0 is base level)
#[inline]
pub fn SetAudioStreamPitch(
    stream: AudioStream,
    pitch: f32,
);

/// Set pan for audio stream (0.5 is centered)
#[inline]
pub fn SetAudioStreamPan(
    stream: AudioStream,
    pan: f32,
);

/// Default size for new audio streams
#[inline]
pub fn SetAudioStreamBufferSizeDefault(
    size: i32,
);

/// Audio thread callback to request new data
#[inline]
pub fn SetAudioStreamCallback(
    stream: AudioStream,
    callback: AudioCallback,
);

/// Attach audio stream processor to stream, receives frames x 2 samples as 'float' (stereo)
///
/// Contrary to buffers, the order of processors is important
///
/// The new processor must be added at the end. As there aren't supposed to be a lot of processors attached to
/// a given stream, we iterate through the list to find the end. That way we don't need a pointer to the last element
#[inline]
pub fn AttachAudioStreamProcessor(
    stream: AudioStream,
    processor: AudioCallback,
);

/// Detach audio stream processor from stream (Remove processor from audio stream)
#[inline]
pub fn DetachAudioStreamProcessor(
    stream: AudioStream,
    processor: AudioCallback,
);

/// Attach audio stream processor to the entire audio pipeline, receives frames x 2 samples as 'float' (stereo)
///
// Order of processors is important

// Works the same way as {Attach,Detach}AudioStreamProcessor() functions, except
// these two work on the already mixed output just before sending it to the sound hardware
#[inline]
pub fn AttachAudioMixedProcessor(
    processor: AudioCallback,
);

/// Detach audio stream processor from the entire audio pipeline (Remove processor from audio pipeline)
#[inline]
pub fn DetachAudioMixedProcessor(
    processor: AudioCallback,
);
