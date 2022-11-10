//! Extension traits for `Stream` implementing various operators.
//!
//! A collection of functions taking typed `Stream` objects as input and producing new `Stream`
//! objects as output. Many of the operators provide simple, composable functionality. Some of the
//! operators are more complicated, for use with advanced timely dataflow features.
//!
//! The [`Operator`](generic::operator) trait provides general
//! operators whose behavior can be supplied using closures accepting input and output handles.
//! Most of the operators in this module are defined using these two general operators.

pub use self::branch::{Branch, BranchWhen};
pub use self::broadcast::Broadcast;
pub use self::capture::Capture;
pub use self::concat::{Concat, Concatenate};
pub use self::delay::Delay;
pub use self::enterleave::{Enter, EnterAt, Leave};
pub use self::exchange::Exchange;
pub use self::feedback::{ConnectLoop, Feedback, LoopVariable};
pub use self::filter::Filter;
pub use self::input::Input;
pub use self::inspect::{Inspect, InspectCore};
pub use self::map::Map;
pub use self::ok_err::OkErr;
pub use self::partition::Partition;
pub use self::probe::Probe;
pub use self::result::ResultStream;
pub use self::to_stream::{Event, ToStream, ToStreamAsync, ToStreamCore};
pub use self::unordered_input::{UnorderedInput, UnorderedInputCore};

pub use self::generic::Operator;
pub use self::generic::{FrontierNotificator, Notificator};

pub use self::count::Accumulate;
pub use self::reclock::Reclock;

pub mod branch;
pub mod broadcast;
pub mod capture;
pub mod concat;
pub mod delay;
pub mod enterleave;
pub mod exchange;
pub mod feedback;
pub mod filter;
pub mod flow_controlled;
pub mod input;
pub mod inspect;
pub mod map;
pub mod ok_err;
pub mod partition;
pub mod probe;
pub mod rc;
pub mod result;
pub mod to_stream;
pub mod unordered_input;

pub mod aggregation;
pub mod generic;

pub mod count;
pub mod reclock;

// keep "mint" module-private
mod capability;
pub use self::capability::{
    ActivateCapability, Capability, CapabilityRef, CapabilitySet, DowngradeError,
};
