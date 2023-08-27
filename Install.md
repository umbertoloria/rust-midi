The following command was necessary for me to use FluidSynth as another MIDI port (the default one seems not to be working fine).

  wget https://schristiancollins.com/generaluser.php -O FluidR3_GM.sf2

The following command opens a new MIDI port with FluidSynth.

  fluidsynth -a alsa -m alsa_seq -l -i -s -o audio.alsa.device=default FluidR3_GM.sf2

You're welcome :)

TODO: See timidity, maybe it has better audio (listen to https://www.youtube.com/watch?v=ywvrzXv36Os)
