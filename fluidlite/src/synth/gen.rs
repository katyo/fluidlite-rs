use crate::{ffi, Chan, Status, Synth};
use num_derive::FromPrimitive;

/**
Generator (effect) numbers

See also _Soundfont 2.01 specifications section 8.1.3_
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum GenParam {
    /** Sample start address offset (0-32767) */
    StartAddrOfs = ffi::fluid_gen_type_GEN_STARTADDROFS,
    /**< Sample end address offset (-32767-0) */
    EndAddrOfs = ffi::fluid_gen_type_GEN_ENDADDROFS,
    /**< Sample loop start address offset (-32767-32767) */
    StartLoopAddOfs = ffi::fluid_gen_type_GEN_STARTLOOPADDROFS,
    /**< Sample loop end address offset (-32767-32767) */
    EndLoopAddrOfs = ffi::fluid_gen_type_GEN_ENDLOOPADDROFS,
    /** Sample start address coarse offset (X 32768) */
    StartAddrCoarseOfs = ffi::fluid_gen_type_GEN_STARTADDRCOARSEOFS,
    /** Modulation LFO to pitch */
    ModLfoToPitch = ffi::fluid_gen_type_GEN_MODLFOTOPITCH,
    /** Vibrato LFO to pitch */
    VibLfoToPitch = ffi::fluid_gen_type_GEN_VIBLFOTOPITCH,
    /** Modulation envelope to pitch */
    ModEnvToPitch = ffi::fluid_gen_type_GEN_MODENVTOPITCH,
    /** Filter cutoff */
    FilterFc = ffi::fluid_gen_type_GEN_FILTERFC,
    /** Filter Q */
    FilterQ = ffi::fluid_gen_type_GEN_FILTERQ,
    /** Modulation LFO to filter cutoff */
    ModLfoToFilterFc = ffi::fluid_gen_type_GEN_MODLFOTOFILTERFC,
    /** Modulation envelope to filter cutoff */
    ModEnvToFilterFc = ffi::fluid_gen_type_GEN_MODENVTOFILTERFC,
    /** Sample end address coarse offset (X 32768) */
    EndAddrCoarseOfs = ffi::fluid_gen_type_GEN_ENDADDRCOARSEOFS,
    /** Modulation LFO to volume */
    ModLfoToVol = ffi::fluid_gen_type_GEN_MODLFOTOVOL,
    /** Chorus send amount */
    ChorussEnd = ffi::fluid_gen_type_GEN_CHORUSSEND,
    /** Reverb send amount */
    ReverbsEnd = ffi::fluid_gen_type_GEN_REVERBSEND,
    /** Stereo panning */
    Pan = ffi::fluid_gen_type_GEN_PAN,
    /** Modulation LFO delay */
    ModLfoDelay = ffi::fluid_gen_type_GEN_MODLFODELAY,
    /** Modulation LFO frequency */
    ModLfoFreq = ffi::fluid_gen_type_GEN_MODLFOFREQ,
    /** Vibrato LFO delay */
    Viblfodelay = ffi::fluid_gen_type_GEN_VIBLFODELAY,
    /** Vibrato LFO frequency */
    VibLfoFreq = ffi::fluid_gen_type_GEN_VIBLFOFREQ,
    /** Modulation envelope delay */
    ModEnvDelay = ffi::fluid_gen_type_GEN_MODENVDELAY,
    /** Modulation envelope attack */
    ModEnvAttack = ffi::fluid_gen_type_GEN_MODENVATTACK,
    /** Modulation envelope hold */
    ModEnvHold = ffi::fluid_gen_type_GEN_MODENVHOLD,
    /** Modulation envelope decay */
    ModEnvDecay = ffi::fluid_gen_type_GEN_MODENVDECAY,
    /** Modulation envelope sustain */
    ModEnvSustain = ffi::fluid_gen_type_GEN_MODENVSUSTAIN,
    /** Modulation envelope release */
    ModEnvRelease = ffi::fluid_gen_type_GEN_MODENVRELEASE,
    /** Key to modulation envelope hold */
    KeyToModEnvHold = ffi::fluid_gen_type_GEN_KEYTOMODENVHOLD,
    /** Key to modulation envelope decay */
    KeyToModEnvDecay = ffi::fluid_gen_type_GEN_KEYTOMODENVDECAY,
    /** Volume envelope delay */
    VolEnvDelay = ffi::fluid_gen_type_GEN_VOLENVDELAY,
    /** Volume envelope attack */
    VolEnvAttack = ffi::fluid_gen_type_GEN_VOLENVATTACK,
    /** Volume envelope hold */
    VolEnvHold = ffi::fluid_gen_type_GEN_VOLENVHOLD,
    /** Volume envelope decay */
    VolEnvDecay = ffi::fluid_gen_type_GEN_VOLENVDECAY,
    /** Volume envelope sustain */
    VolEnvSustain = ffi::fluid_gen_type_GEN_VOLENVSUSTAIN,
    /** Volume envelope release */
    VolEnvRelease = ffi::fluid_gen_type_GEN_VOLENVRELEASE,
    /** Key to volume envelope hold */
    KeyToVolEnvHold = ffi::fluid_gen_type_GEN_KEYTOVOLENVHOLD,
    /** Key to volume envelope decay */
    KeyToVolEnvDecay = ffi::fluid_gen_type_GEN_KEYTOVOLENVDECAY,
    /** Instrument ID (shouldn't be set by user) */
    Instrument = ffi::fluid_gen_type_GEN_INSTRUMENT,
    /** MIDI note range */
    Keyrange = ffi::fluid_gen_type_GEN_KEYRANGE,
    /** MIDI velocity range */
    Velrange = ffi::fluid_gen_type_GEN_VELRANGE,
    /** Sample start loop address coarse offset (X 32768) */
    Startloopaddrcoarseofs = ffi::fluid_gen_type_GEN_STARTLOOPADDRCOARSEOFS,
    /** Fixed MIDI note number */
    Keynum = ffi::fluid_gen_type_GEN_KEYNUM,
    /** Fixed MIDI velocity value */
    Velocity = ffi::fluid_gen_type_GEN_VELOCITY,
    /** Initial volume attenuation */
    Attenuation = ffi::fluid_gen_type_GEN_ATTENUATION,
    /** Sample end loop address coarse offset (X 32768) */
    EndLoopAddrCoarseOfs = ffi::fluid_gen_type_GEN_ENDLOOPADDRCOARSEOFS,
    /** Coarse tuning */
    CoarseTune = ffi::fluid_gen_type_GEN_COARSETUNE,
    /** Fine tuning */
    FineTune = ffi::fluid_gen_type_GEN_FINETUNE,
    /** Sample ID (shouldn't be set by user) */
    SampleId = ffi::fluid_gen_type_GEN_SAMPLEID,
    /** Sample mode flags */
    SampleMode = ffi::fluid_gen_type_GEN_SAMPLEMODE,
    /** Scale tuning */
    ScaleTune = ffi::fluid_gen_type_GEN_SCALETUNE,
    /** Exclusive class number */
    ExclusiveClass = ffi::fluid_gen_type_GEN_EXCLUSIVECLASS,
    /** Sample root note override */
    OverrideRootKey = ffi::fluid_gen_type_GEN_OVERRIDEROOTKEY,

    /** Pitch (NOTE: Not a real SoundFont generator)

    The initial pitch is not a "standard" generator. It is not
    mentioned in the list of generator in the SF2 specifications. It
    is used, however, as the destination for the default pitch wheel
    modulator.
     */
    Pitch = ffi::fluid_gen_type_GEN_PITCH,
}

/**
Generator interface
 */
impl Synth {
    /**
    Change the value of a generator. This function allows to control
    all synthesis parameters in real-time. The changes are additive,
    i.e. they add up to the existing parameter value. This function is
    similar to sending an NRPN message to the synthesizer. The
    function accepts a float as the value of the parameter. The
    parameter numbers and ranges are described in the SoundFont 2.01
    specification, paragraph 8.1.3, page 48.
     */
    pub fn set_gen(&self, chan: Chan, param: GenParam, value: f32) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_set_gen(self.handle, chan as _, param as _, value) })
    }

    /**
    Retreive the value of a generator. This function returns the value
    set by a previous call 'set_gen()' or by an NRPN message.

    Returns the value of the generator.
     */
    pub fn get_gen(&self, chan: Chan, param: GenParam) -> f32 {
        unsafe { ffi::fluid_synth_get_gen(self.handle, chan as _, param as _) }
    }
}
