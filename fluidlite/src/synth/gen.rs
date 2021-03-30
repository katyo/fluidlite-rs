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
    StartAddrOfs = ffi::fluid_gen_type_GEN_STARTADDROFS as _,
    /**< Sample end address offset (-32767-0) */
    EndAddrOfs = ffi::fluid_gen_type_GEN_ENDADDROFS as _,
    /**< Sample loop start address offset (-32767-32767) */
    StartLoopAddOfs = ffi::fluid_gen_type_GEN_STARTLOOPADDROFS as _,
    /**< Sample loop end address offset (-32767-32767) */
    EndLoopAddrOfs = ffi::fluid_gen_type_GEN_ENDLOOPADDROFS as _,
    /** Sample start address coarse offset (X 32768) */
    StartAddrCoarseOfs = ffi::fluid_gen_type_GEN_STARTADDRCOARSEOFS as _,
    /** Modulation LFO to pitch */
    ModLfoToPitch = ffi::fluid_gen_type_GEN_MODLFOTOPITCH as _,
    /** Vibrato LFO to pitch */
    VibLfoToPitch = ffi::fluid_gen_type_GEN_VIBLFOTOPITCH as _,
    /** Modulation envelope to pitch */
    ModEnvToPitch = ffi::fluid_gen_type_GEN_MODENVTOPITCH as _,
    /** Filter cutoff */
    FilterFc = ffi::fluid_gen_type_GEN_FILTERFC as _,
    /** Filter Q */
    FilterQ = ffi::fluid_gen_type_GEN_FILTERQ as _,
    /** Modulation LFO to filter cutoff */
    ModLfoToFilterFc = ffi::fluid_gen_type_GEN_MODLFOTOFILTERFC as _,
    /** Modulation envelope to filter cutoff */
    ModEnvToFilterFc = ffi::fluid_gen_type_GEN_MODENVTOFILTERFC as _,
    /** Sample end address coarse offset (X 32768) */
    EndAddrCoarseOfs = ffi::fluid_gen_type_GEN_ENDADDRCOARSEOFS as _,
    /** Modulation LFO to volume */
    ModLfoToVol = ffi::fluid_gen_type_GEN_MODLFOTOVOL as _,
    /** Chorus send amount */
    ChorussEnd = ffi::fluid_gen_type_GEN_CHORUSSEND as _,
    /** Reverb send amount */
    ReverbsEnd = ffi::fluid_gen_type_GEN_REVERBSEND as _,
    /** Stereo panning */
    Pan = ffi::fluid_gen_type_GEN_PAN as _,
    /** Modulation LFO delay */
    ModLfoDelay = ffi::fluid_gen_type_GEN_MODLFODELAY as _,
    /** Modulation LFO frequency */
    ModLfoFreq = ffi::fluid_gen_type_GEN_MODLFOFREQ as _,
    /** Vibrato LFO delay */
    Viblfodelay = ffi::fluid_gen_type_GEN_VIBLFODELAY as _,
    /** Vibrato LFO frequency */
    VibLfoFreq = ffi::fluid_gen_type_GEN_VIBLFOFREQ as _,
    /** Modulation envelope delay */
    ModEnvDelay = ffi::fluid_gen_type_GEN_MODENVDELAY as _,
    /** Modulation envelope attack */
    ModEnvAttack = ffi::fluid_gen_type_GEN_MODENVATTACK as _,
    /** Modulation envelope hold */
    ModEnvHold = ffi::fluid_gen_type_GEN_MODENVHOLD as _,
    /** Modulation envelope decay */
    ModEnvDecay = ffi::fluid_gen_type_GEN_MODENVDECAY as _,
    /** Modulation envelope sustain */
    ModEnvSustain = ffi::fluid_gen_type_GEN_MODENVSUSTAIN as _,
    /** Modulation envelope release */
    ModEnvRelease = ffi::fluid_gen_type_GEN_MODENVRELEASE as _,
    /** Key to modulation envelope hold */
    KeyToModEnvHold = ffi::fluid_gen_type_GEN_KEYTOMODENVHOLD as _,
    /** Key to modulation envelope decay */
    KeyToModEnvDecay = ffi::fluid_gen_type_GEN_KEYTOMODENVDECAY as _,
    /** Volume envelope delay */
    VolEnvDelay = ffi::fluid_gen_type_GEN_VOLENVDELAY as _,
    /** Volume envelope attack */
    VolEnvAttack = ffi::fluid_gen_type_GEN_VOLENVATTACK as _,
    /** Volume envelope hold */
    VolEnvHold = ffi::fluid_gen_type_GEN_VOLENVHOLD as _,
    /** Volume envelope decay */
    VolEnvDecay = ffi::fluid_gen_type_GEN_VOLENVDECAY as _,
    /** Volume envelope sustain */
    VolEnvSustain = ffi::fluid_gen_type_GEN_VOLENVSUSTAIN as _,
    /** Volume envelope release */
    VolEnvRelease = ffi::fluid_gen_type_GEN_VOLENVRELEASE as _,
    /** Key to volume envelope hold */
    KeyToVolEnvHold = ffi::fluid_gen_type_GEN_KEYTOVOLENVHOLD as _,
    /** Key to volume envelope decay */
    KeyToVolEnvDecay = ffi::fluid_gen_type_GEN_KEYTOVOLENVDECAY as _,
    /** Instrument ID (shouldn't be set by user) */
    Instrument = ffi::fluid_gen_type_GEN_INSTRUMENT as _,
    /** MIDI note range */
    Keyrange = ffi::fluid_gen_type_GEN_KEYRANGE as _,
    /** MIDI velocity range */
    Velrange = ffi::fluid_gen_type_GEN_VELRANGE as _,
    /** Sample start loop address coarse offset (X 32768) */
    Startloopaddrcoarseofs = ffi::fluid_gen_type_GEN_STARTLOOPADDRCOARSEOFS as _,
    /** Fixed MIDI note number */
    Keynum = ffi::fluid_gen_type_GEN_KEYNUM as _,
    /** Fixed MIDI velocity value */
    Velocity = ffi::fluid_gen_type_GEN_VELOCITY as _,
    /** Initial volume attenuation */
    Attenuation = ffi::fluid_gen_type_GEN_ATTENUATION as _,
    /** Sample end loop address coarse offset (X 32768) */
    EndLoopAddrCoarseOfs = ffi::fluid_gen_type_GEN_ENDLOOPADDRCOARSEOFS as _,
    /** Coarse tuning */
    CoarseTune = ffi::fluid_gen_type_GEN_COARSETUNE as _,
    /** Fine tuning */
    FineTune = ffi::fluid_gen_type_GEN_FINETUNE as _,
    /** Sample ID (shouldn't be set by user) */
    SampleId = ffi::fluid_gen_type_GEN_SAMPLEID as _,
    /** Sample mode flags */
    SampleMode = ffi::fluid_gen_type_GEN_SAMPLEMODE as _,
    /** Scale tuning */
    ScaleTune = ffi::fluid_gen_type_GEN_SCALETUNE as _,
    /** Exclusive class number */
    ExclusiveClass = ffi::fluid_gen_type_GEN_EXCLUSIVECLASS as _,
    /** Sample root note override */
    OverrideRootKey = ffi::fluid_gen_type_GEN_OVERRIDEROOTKEY as _,

    /** Pitch (NOTE: Not a real SoundFont generator)

    The initial pitch is not a "standard" generator. It is not
    mentioned in the list of generator in the SF2 specifications. It
    is used, however, as the destination for the default pitch wheel
    modulator.
     */
    Pitch = ffi::fluid_gen_type_GEN_PITCH as _,
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
