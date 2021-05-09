use crate::Synth;

/**
Low level access
 */
impl Synth {
    /*
      /**
        Create and start voices using a preset. The id passed as
        argument will be used as the voice group id.
         */
    FLUIDSYNTH_API int fluid_synth_start(fluid_synth_t* synth, unsigned int id,
                         fluid_preset_t* preset, int audio_chan,
                         int midi_chan, int key, int vel);

      /** Stop the voices in the voice group defined by id. */
    FLUIDSYNTH_API int fluid_synth_stop(fluid_synth_t* synth, unsigned int id);
    */
}
