use super::MidiMsg;

/// Passed to [`MidiMsg::from_midi_with_context`](crate::MidiMsg::from_midi_with_context) to allow
/// for the capture and used of captured context while reading from a MIDI stream.
///
/// This is used to allow for the formation of fully formed `MidiMsg`s when either a running
/// status is being employed, or when using 14-bit [`ControlChange`](crate::ControlChange) messages.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ReceiverContext {
    pub previous_channel_message: Option<MidiMsg>,
}