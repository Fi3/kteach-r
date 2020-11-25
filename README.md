# kteach-r

COMPOSABLE SAMPLEs PLAYER

A sample player that can play multiple sample and mix them. Can apply effect at the samples.
Mixers players and effects are user configurable via GUI. Can record loop. Can analyze samples BPM ecc ecc.
Can play sample in sync. Can receive midi command.

## Name

The name kteach-r pronunced /kiː/ /ˈtɛk/ /ɑː/ is composed by k teach and r.

teach cause I like techno music, r cause of rust.

k cause the name kteach-r is a tribute to a character of [Battlestar Galactica](https://en.wikipedia.org/wiki/Battlestar_Galactica_2004_TV_series)

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

## Examples
kteach-r is composed by a [core](kteach-core/Cargo.toml) and [2 GUIs](#GUI). Example of how to use the core API without all the overhead of the graphical interfaces are in [examples](kteach-core/examples)
**parsing mp3 in debug mode is pretty slow, better to use** `cargo run --release`

## Non goal

kteach-r is mostly an experiment so for now every direction is a good one.

## Architecture
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

I want (at least at the beginning) develop 2 GUI for the core. This because:
* I want to do a comparison between the chosen libraries.
* Having 2 GUI help keep everything more modular.

After a very superficial research the two libraries that I want to try are:
* [crochet](https://github.com/raphlinus/crochet): the underlying concepts seems very interesting
* [iced](https://github.com/hecrj/iced): cause I like elm

## Decoder
[Symphonia](https://github.com/pdeljanov/Symphonia) is the decoder. Cause is pure rust and it seems
the most complete option. It is also WASM ready.

## Reproduce audio
[cpal](https://docs.rs/cpal/0.13.1/cpal/) is used to play audio cause it seems the defacto standard in rust.

## RealTime audio engine
[synthesizer-io-core](https://github.com/raphlinus/synthesizer-io) is used as RT audio engine.

## Midi
[midir](https://github.com/Boddlnagg/midir) is used in order to send and receive midi messages.
[wmidi](https://github.com/RustAudio/wmidi) is used to serialize and deserialize midi messages.


## Milestones

- [x] sample player (pause and play)
- [ ] multi players support (pause and play)
- [ ] 2-channel mixer
- [ ] sample player position
- [ ] sample player (play from)
- [ ] sample player speed
- [ ] sample player speed with same pitch
- [ ] sample player waveform
- [ ] sample player sync
- [ ] n-channel mixer
- [ ] effects (at least a delay and a reverb and a low/high pass)
- [ ] config file (midi and player/mixer/effect)
- [ ] sample analyzer (BPM and key and autogain)
- [ ] sample recorder
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
* https://github.com/BillyDM/iced_audio
    Audio widgets for iced
 
