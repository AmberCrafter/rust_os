use core::task::Poll;

use crate::{print, println};
use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::{task::AtomicWaker, Stream, StreamExt};
use pc_keyboard::{layouts, DecodedKey, Keyboard, ScancodeSet1};

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();
// static WAKER: OnceCell<AtomicWaker> = OnceCell::uninit();

pub fn init() {
    ScancodeStream::init();
    // Waker::new();
}

pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            // println!("Push scancode: {:?}", scancode);
            WAKER.wake();
        }
    } else {
        println!("WARNING: Scancode queue uninitialized");
    }
}

pub(crate) fn print_queue() {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        println!("queue length: {:?}", queue.len());
    }
}

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn init() {
        SCANCODE_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
    }
    pub fn new() -> Self {
        ScancodeStream { _private: () }
    }
}

// impl Stream for ScancodeStream {
//     type Item = u8;

//     fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context) -> core::task::Poll<Option<Self::Item>> {
//         let queue = SCANCODE_QUEUE.try_get().expect("not initialized");
//         println!("Stream queue length: {:?}", queue.len());
//         let scancode = queue.pop();
//         println!("scancode: {:?}", scancode);

//         if let Ok(scancode) = scancode {
//             return Poll::Ready(Some(scancode));
//         }

//         WAKER.register(cx.waker());

//         match scancode {
//             Ok(scancode) => {
//                 WAKER.take();
//                 Poll::Ready(Some(scancode))
//             },
//             Err(crossbeam_queue::PopError) => Poll::Pending
//         }
//     }
// }

use core::pin::Pin;
use core::task::Context;
impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("scancode queue not initialized");

        // fast path
        if let Ok(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Ok(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}

pub async fn print_keypresses() {
    ScancodeStream::init();
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(
        layouts::Us104Key,
        ScancodeSet1,
        pc_keyboard::HandleControl::Ignore,
    );
    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => print!("{}", character),
                    DecodedKey::RawKey(key) => print!("{:?}", key),
                }
            }
        }
    }
}
