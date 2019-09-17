// Copyright (c) 2019 Rodolphe Bréard
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::entry::{Entry, Event, Kind, Subsystem, Version};
use crate::errors::Error;
use crate::output::FilterOutput;
use std::collections::HashSet;

macro_rules! handle {
    ($self: ident, $obj: ident, $version: expr, $kind: expr, $entry: ident, $output: ident) => {{
        if $self.version == $version
            && $self.kind == $kind
            && $self.subsystem == $obj.subsystem
            && $self.events.contains(&$obj.event)
        {
            ($self.action)($output, $entry)?;
        }
        Ok(())
    }};
}

type Callback = fn(&mut dyn FilterOutput, &Entry) -> Result<(), String>;

#[derive(Clone)]
pub struct Handler {
    version: Version,
    kind: Kind,
    subsystem: Subsystem,
    events: HashSet<Event>,
    action: Callback,
}

impl Handler {
    pub fn new(
        version: Version,
        kind: Kind,
        subsystem: Subsystem,
        events: &[Event],
        action: Callback,
    ) -> Self {
        Handler {
            version,
            kind,
            subsystem,
            events: events.iter().cloned().collect(),
            action,
        }
    }

    pub fn send(&self, entry: &Entry, output: &mut dyn FilterOutput) -> Result<(), Error> {
        match entry {
            Entry::V1Report(report) => {
                handle!(self, report, Version::V1, Kind::Report, entry, output)
            }
            Entry::V1Filter(filter) => {
                handle!(self, filter, Version::V1, Kind::Filter, entry, output)
            }
        }
    }
}
