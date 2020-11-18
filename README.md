# kteach-r

COMPOSABLE SAMPLEs PLAYER
A sample player can play multiple sample and mix them. Can apply effect at the sample. Mixers players
and effects are user configurable via GUI. Can record loop. Can analyze samples BPM ecc ecc. Can play
sample in sync. Can receive midi command.

## Name

The name kteach-r pronunced /kiː/ /ˈtɛk/ /ɑː/ is composed by k teach and r.

teach cause I like techno music, r cause of rust.

k cause the name kteach-r is a tribute to a character of [Battlestar Galactica](https://en.wikipedia.org/wiki/Battlestar_Galactica_(2004_TV_series)

## What it can do

* 1 kind of sample/track player (play, pause, loop, pitch, tempo)
* several kinds of effects (volume, echo, ...)
* 1 kind of mixer (2 source 1 output)
* 1 kind of recorder
* everything above composable via GUI
* control via MIDI
* file browser and music collection (via a separate and standalone bin)
* core buildable on wasm and embedded, (so it will possible to use kteach-r on standalone
    controllers)
* 3 modalities editing, browse, performance, build


tutti i messaggi durante performance arrivano via midi, se qlks viene fatto dall gui viene tradotto
in midi. build mode descrive come le varie source sono collegate tra di loro e viene scritto in un
file di config toml (se per esempio voglio un effetto splittabile tra due source colelgero le source
ad un mixer e queste ad un effetto poi durante performance con midi decido come dirigere il
segnale) in build mode viene anche definito tutta la midi mapp.
A GUI is provided but it should be possible to run kteach against other GUI for example when
embedded.

## Non goal

E' principalmente un esperimento quindi per ora aperto verso qualsiasi direzione.

## Architecture

loop {
  state = get state
  /// next1 = next(audio source, state)
  /// next2 = next(audio source, state)
  /// next3 = next(audio source, state)
  /// next is {sample: (left, right), destination: impl trait Destination}
  /// audio source impl trait origin
  /// a node usally implement both Origin and Destination
  /// output is a buffer that implement only destination and each x time
  /// must be filled with new data
  /// at each round of the loop the samples flow from origin to destinations
  /// if destination is output the increments the buffer, there are exactly
  /// x round of loop in a second. For each round something must be written on the
  /// output, for silence output will be filled with 0s
  /// State define how the nodes (Destination and Sources) are connected. And
  /// how each destination or source is setted. State is filled in the real time
  /// thread from the GUI and MIDI threads.
  /// Initial sources are filled in the real time thread from the decoder
  /// Construct the state tree at each round could be wasteful
  /// visto che si va sullo stereo si puo usare due thread
  /// essendo le operazioni sequenziali si puo implementare una pipeline?>? in 
  /// realta se state cambia la pipelin va a farsi fottere quindi non so quanto co
  /// nvenga.
  /// di sicuro studiare fearless_simd
  /// allinizio convertire qualsiasi tipo di signal a due segnali mono con precisa
  /// bitrate inizialmente solo un bitrate e' supportato! nel senso che quando
  /// lanci app scegli il bitrate e tutti i sample caricati sono convertiti a quel
  /// bitrate
  nexts = map(sources, x => next(x, state))

```
                             ____  ____     
                          /\/    \/    \/\/\                                   .
                         /                  \                                 .
                        /                    \                               /
  (ON SAMPLE LOAD)*    / input => decoder =>  \                             / 
  (ON GUI EVENT)  *   /  input => GUI     =>   | => audio engine -> output .
. (ON MIDI EVENT) *  /   input => MIDI    =>  / .                         . 
 \                  /       ^-------^            \                       /
  \                /            ^                 \                     /
   .THIS IS A WAVE*             r                  .---> REAL-TIME <---.
     ------------               e                   -------------------
                                a
                                l
                                 
                                t
                                i
                                m
                                e
```
## GUI

The GUI is done with [crochet](https://github.com/raphlinus/crochet) cause I want a stable UI toolkit for rust and I think that crochet is going in the right direction so I hope that using it will help crochet to grow. Also this
is a side project and there aren't time constraints. It is also WASM "ready".

## Decoder

[Symphonia](https://github.com/pdeljanov/Symphonia) is the decoder. Cause is pure rust and it seems
the most completely option. It is also WASM ready.

## Reproduce audio
[cpal](https://docs.rs/cpal/0.13.1/cpal/) is used to play audio cause it seems the defacto standard in rust.

## Goals and non Goals

## Milestones

- [ ] sample player (pause and play)
- [ ] 2-channel mixer
- [ ] sample player position
- [ ] sample player (play from)
- [ ] sample player speed
- [ ] sample player speed with same pitch
- [ ] sample player waveform
- [ ] sample player sync
- [ ] n-channel mixer
- [ ] effects (at least a delay and a reverb and a low/high pass)
- [ ] sample recorder
- [ ] MIDI signal
- [ ] sample analyzer (BPM and key and autogain)
- [ ] file browser

## Useful crates

* https://github.com/RustAudio/rodio
    Decode MP3, WAV, Vorbis, Flac. Playback with [cpal](https://github.com/RustAudio/cpal)
* https://github.com/RustAudio/dasp
    Set of library to work with digital audio signal. Dasp libraries require no dynamic allocations1
    and have no dependencies.
* https://github.com/RustAudio/wmidi
    Midi encoding and decoding library suitable for real-time execution.
* https://github.com/RustAudio/pitch_calc
    A library for musical pitch conversions!
* https://github.com/RustAudio/time_calc
    A library for music/DSP time conversions!
* https://github.com/RustAudio/dsp-chain
    A library for chaining together multiple audio dsp processors/generators
* https://github.com/Windfisch/rust-assert-no-alloc
    This crate provides a custom allocator that allows to temporarily disable memory (de)allocations
    for a thread.
* https://github.com/reedrosenbluth/oscen
    It contains a collection of components frequently used in sound synthesis such as oscillators,
    filters, and envelope generators. It lets you connect (or patch) the output of one module into the input of another.
* https://docs.rs/puremp3/0.1.0/puremp3/
    mp3 decoder
* https://crates.io/crates/assert_no_alloc
    TODO
* https://crates.io/crates/ringbuf
    TODO
* https://docs.rs/bbqueue/0.4.10/bbqueue/
    TODO
* https://github.com/padenot/audio_thread_priority
    TODO
 
