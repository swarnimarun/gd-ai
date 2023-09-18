# godot ai

building simple ai tools for godot.

## Current state - WIP

only implemented whisper model support for transcription.

**Notes:** I don't recommend models larger than medium for now, the code is pretty flaky, also keep in mind the whisper transcription only works with 16kHz,
and single channel, we do support resampling but it's preferrable to not have to do it, for more accuracy and performance(resampling happens on CPU it can be quite slow, around 1 second for a few minute long sample). Also whisper only works properly with 30sec samples. Consider.

## Motivation

I wrote these things a while ago for Godot in GDNative porting it to gdext and making it open source, cause this might help someone else out there as well. Haha.


## Credits

burn and @Gadersd, I used some of the code from their examples as base for building model and other parts of the code structure for the burn interface layer,
and gdext cause writing the loader from scratch would be too much work.
And thanks to any open-source projects I use in the list above. Please check the cargo.toml for the lists.

Aside:
rubato also was very useful otherwise it would be a pita to get "resampling" working on windows. CoreAudio(mac) and Linux(some of the audio stacks) already support in built resampling. Sadly Godot doesn't provide external access to resampling code it has built-in, that would be quite good to have.

## Author: Me (Swarnim)

## Support

Currently consider just staring the project, I will accept contributions once I have completed atleast improving the overall infrastructure.
