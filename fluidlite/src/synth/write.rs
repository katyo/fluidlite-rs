use crate::{ffi, Synth, Status};

/**
Synthesizer plugin
 */
impl Synth {
    /**
    Generate a number of samples. This function expects two signed
    16bits buffers (left and right channel) that will be filled with
    samples.

    \param synth The synthesizer
    \param len The number of samples to generate
    \param lout The sample buffer for the left channel
    \param loff The offset, in samples, in the left buffer where the writing pointer starts
    \param lincr The increment, in samples, of the writing pointer in the left buffer
    \param rout The sample buffer for the right channel
    \param roff The offset, in samples, in the right buffer where the writing pointer starts
    \param rincr The increment, in samples, of the writing pointer in the right buffer
    \returns 0 if no error occured, non-zero otherwise
     */
    pub fn write_s16(&self,
FLUIDSYNTH_API int fluid_synth_write_s16(fluid_synth_t* synth, int len,
				       void* lout, int loff, int lincr,
				       void* rout, int roff, int rincr);


    /**
    Generate a number of samples. This function expects two floating
    point buffers (left and right channel) that will be filled with
    samples.

    \param synth The synthesizer
    \param len The number of samples to generate
    \param lout The sample buffer for the left channel
    \param loff The offset, in samples, in the left buffer where the writing pointer starts
    \param lincr The increment, in samples, of the writing pointer in the left buffer
    \param rout The sample buffer for the right channel
    \param roff The offset, in samples, in the right buffer where the writing pointer starts
    \param rincr The increment, in samples, of the writing pointer in the right buffer
    \returns 0 if no error occured, non-zero otherwise
     */

FLUIDSYNTH_API int fluid_synth_write_float(fluid_synth_t* synth, int len,
					 void* lout, int loff, int lincr,
					 void* rout, int roff, int rincr);

FLUIDSYNTH_API int fluid_synth_nwrite_float(fluid_synth_t* synth, int len,
					  float** left, float** right,
					  float** fx_left, float** fx_right);

    /**
    Generate a number of samples. This function implements the
    default interface defined in fluidsynth/audio.h. This function
    ignores the input buffers and expects at least two output
    buffer.

    \param synth The synthesizer
    \param len The number of samples to generate
    \param nin The number of input buffers
    \param in The array of input buffers
    \param nout The number of output buffers
    \param out The array of output buffers
    \returns 0 if no error occured, non-zero otherwise
     */

FLUIDSYNTH_API int fluid_synth_process(fluid_synth_t* synth, int len,
				     int nin, float** in,
				     int nout, float** out);
}
