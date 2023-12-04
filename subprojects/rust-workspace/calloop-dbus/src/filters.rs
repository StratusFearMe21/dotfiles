use dbus::{channel::Token, message::MatchRule, Message};

use log::trace;

use std::collections::BTreeMap;

pub struct Filters<F> {
    list: BTreeMap<dbus::channel::Token, (MatchRule<'static>, F)>,
    next_id: dbus::channel::Token,
}

impl<F> Default for Filters<F> {
    fn default() -> Self {
        Self {
            list: Default::default(),
            next_id: dbus::channel::Token(1),
        }
    }
}

impl<F> Filters<F> {
    pub fn add(&mut self, m: MatchRule<'static>, f: F) -> Token {
        let id = self.next_id;
        self.next_id.0 += 1;
        trace!("inserted token {} with value {:?}", id.0, &m);
        self.list.insert(id, (m, f));
        id
    }

    pub fn remove(&mut self, id: Token) -> Option<(MatchRule<'static>, F)> {
        trace!("removed token {}", id.0);
        self.list.remove(&id)
    }

    // pub fn get_matches(
    //     &mut self,
    //     m: &Message,
    // ) -> Option<(&dbus::channel::Token, &mut (MatchRule<'static>, F))> {
    //     self.list.iter_mut().find(|(_, v)| v.0.matches(m))
    // }
}
