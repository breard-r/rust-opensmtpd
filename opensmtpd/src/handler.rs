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
    ($self: ident, $obj: ident, $version: expr, $kind: expr, $entry: ident, $output: ident, $session_ctx: ident, $filter_ctx: ident) => {{
        if $self.version == $version
            && $self.kind == $kind
            && $self.subsystem == $obj.subsystem
            && $self.events.contains(&$obj.event)
        {
            ($self.action)($output, $entry, $session_ctx, $filter_ctx)?;
        }
        Ok(())
    }};
}

type Callback<S, F> = fn(&mut dyn FilterOutput, &Entry, &mut S, &mut F) -> Result<(), String>;

#[derive(Clone)]
pub struct Handler<S, F> {
    version: Version,
    pub(crate) kind: Kind,
    pub(crate) subsystem: Subsystem,
    pub(crate) events: HashSet<Event>,
    action: Callback<S, F>,
}

impl<S, F> Handler<S, F> {
    pub fn new(
        version: Version,
        kind: Kind,
        subsystem: Subsystem,
        events: &[Event],
        action: Callback<S, F>,
    ) -> Self {
        Handler {
            version,
            kind,
            subsystem,
            events: events.iter().cloned().collect(),
            action,
        }
    }

    pub fn send(
        &self,
        entry: &Entry,
        output: &mut dyn FilterOutput,
        session_ctx: &mut S,
        filter_ctx: &mut F,
    ) -> Result<(), Error> {
        match entry {
            Entry::V1Report(report) => handle!(
                self,
                report,
                Version::V1,
                Kind::Report,
                entry,
                output,
                session_ctx,
                filter_ctx
            ),
            Entry::V1Filter(filter) => handle!(
                self,
                filter,
                Version::V1,
                Kind::Filter,
                entry,
                output,
                session_ctx,
                filter_ctx
            ),
        }
    }
}
