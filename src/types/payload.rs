use std::ops::AddAssign;

/// Maximum number of messages a payload can contain.
const CAPACITY: usize = 16;

/// A small container of messages.
/// Messages should be small.
/// Add messages using the `+=` operator.
/// # Safety
/// The number of messages must not exceed [`CAPACITY`].
/// Adding a message beyond the capacity will trigger a panic.
#[derive(Default)]
pub struct Payload<Message: Clone + Copy> {
    messages: [Option<Message>; CAPACITY],
    count: usize,
}

impl<Message: Clone + Copy> Payload<Message> {
    pub fn new() -> Self {
        Self {
            messages: [None; CAPACITY],
            count: 0,
        }
    }

    /// Reset the payload to empty.
    pub fn reset(&mut self) {
        self.count = 0;
    }

    /// Number of messages in the payload.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Iterator over all messages in the payload.
    pub fn iter(&self) -> PayloadIter<Message> {
        PayloadIter {
            payload: &self,
            index: 0,
        }
    }
}

impl<Message: Clone + Copy> AddAssign<Message> for Payload<Message> {
    fn add_assign(&mut self, message: Message) {
        self.messages[self.count] = Some(message);
        self.count += 1;
    }
}

pub struct PayloadIter<'payload, Message: Clone + Copy> {
    payload: &'payload Payload<Message>,
    index: usize,
}

impl<'payload, Message: Clone + Copy> Iterator for PayloadIter<'payload, Message> {
    type Item = Message;

    fn next(&mut self) -> Option<Message> {
        if self.index < self.payload.count {
            let message = self.payload.messages[self.index].expect("Non-none message");
            self.index += 1;
            Some(message)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iter() {
        let mut payload: Payload<u32> = Payload::default();
        payload += 10;
        payload += 15;
        payload += 4;
        let mut iter = payload.iter();
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(15));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn reset() {
        let mut payload: Payload<u32> = Payload::default();
        payload += 3;
        payload += 2;
        payload += 1;
        payload.reset();
        assert_eq!(payload.len(), 0);
    }

    #[test]
    fn len() {
        let mut payload: Payload<u32> = Payload::default();
        payload += 10;
        payload += 15;
        payload += 4;
        assert_eq!(payload.len(), 3);
    }

    #[test]
    #[should_panic]
    fn panic_on_overflow() {
        let mut payload: Payload<u32> = Payload::default();
        for i in 0..CAPACITY {
            payload += i as u32;
        }
        payload += 0;
    }
}
