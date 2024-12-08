use crate::protocol::STREAM_CONTINUE;
use crate::protocol::STREAM_END;
use crate::protocol::STREAM_START;

#[derive(Debug, Clone)]
pub enum MultiplexerEvent<'a> {
    StreamStarted { stream_id: u64, length: u16 },
    DataPointer { stream_id: u64, data: &'a [u8] },
    StreamEnded { stream_id: u64 },
    Error(String),
}


struct CurrentStream {
    stream_id: u64,
    length: u16,
    recv_count: u32,
}

enum MultiplexerState {
    Idle,
    Receiving(CurrentStream),
}

pub struct Multiplexer {
    state: MultiplexerState,
    buffer: Vec<u8>, // Buffer to hold incomplete data
}

impl Multiplexer {
    pub fn new() -> Multiplexer {
        Multiplexer {
            state: MultiplexerState::Idle,
            buffer: Vec::new(),
        }
    }

    pub fn handle_data<F>(&mut self, data: &[u8], mut callback: F) -> usize
    where
        F: FnMut(MultiplexerEvent),
    {
		let mut i = 0;
		while i < data.len() {
			let data = &data[i..];
			let mut next_state = None;
			match &mut self.state {
				MultiplexerState::Idle => {
					if data.len() < 10 {
						return data.len();
					}
					let raw_id = u64::from_le_bytes(data[0..8].try_into().unwrap());
                    let stage = (raw_id & 0x0F) as u8;
                    let stream_id = raw_id >> 4;
                    let length = u16::from_le_bytes(data[8..10].try_into().unwrap());
					i += 10;

					match stage {
                        STREAM_START => {
                            callback(MultiplexerEvent::StreamStarted {
                                stream_id,
                                length,
                            });
							next_state = Some(MultiplexerState::Receiving(CurrentStream {
								stream_id,
								length,
								recv_count: 0,
							}));
                        }
                        STREAM_END => {
                            callback(MultiplexerEvent::StreamEnded { stream_id });
                        }
                        STREAM_CONTINUE => {}
                        _ => {
                            callback(MultiplexerEvent::Error(format!("Invalid stage: {}", stage)));
                        }
                    }
				}
				MultiplexerState::Receiving(stream_info) => {
					let payload_left = (stream_info.length as usize).saturating_sub(stream_info.recv_count as usize);
                    let payload_len = payload_left.min(data.len());
                    let payload = &data[..payload_len];
					callback(MultiplexerEvent::DataPointer {
						stream_id: stream_info.stream_id,
						data: payload,
					});
					stream_info.recv_count += payload.len() as u32;
					i += payload_len;
					if stream_info.recv_count as u16 >= stream_info.length {
						next_state = Some(MultiplexerState::Idle);
						callback(MultiplexerEvent::StreamEnded {
							stream_id: stream_info.stream_id,
						});
					}
				}
			}
			if let Some(state) = next_state {
				self.state = state;
			}
		}
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

	// fn run_test(multiplexer: &mut Multiplexer, input: &[u8]) -> Vec<MultiplexerEvent<'static>> {
    //     let mut events = Vec::new();
        
    //     // We'll create a callback that pushes events to the `events` vector.
    //     // To allow a static lifetime, we can clone the data or just ensure the data we pass in tests is 'static.
    //     let mut callback = |evt: MultiplexerEvent<'_>| {
    //         // Since the DataPointer event borrows data from input,
    //         // we might need to clone it for testing if required.
    //         match evt {
    //             MultiplexerEvent::DataPointer { stream_id, data } => {
    //                 // Clone the data so it can live beyond this callback
    //                 events.push(MultiplexerEvent::DataPointer {
    //                     stream_id,
    //                     data: Box::leak(data.to_vec().into_boxed_slice()),
    //                 });
    //             }
    //             other => events.push(other),
    //         }
    //     };

    //     multiplexer.handle_data(input, &mut callback);
    //     events
    // }

    #[test]
    fn test_stream_start_and_data() {
        let mut multiplexer = Multiplexer::new();
        // Simulated data for a stream start
        let data = [
            0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Stream ID and stage (STREAM_START)
            0x05, 0x00,                                     // Length of the stream
            0x41, 0x42,                                     // Payload data
        ];
		let mut count = 0;
        multiplexer.handle_data(&data, |event| {
            match event {
				MultiplexerEvent::StreamStarted { stream_id, length } => {
					assert_eq!(stream_id, 1);
					assert_eq!(length, 5);
				},
				MultiplexerEvent::DataPointer { stream_id, data } => {
					assert_eq!(stream_id, 1);
					assert_eq!(data, b"AB");
				},
				MultiplexerEvent::StreamEnded { stream_id } => {},
				MultiplexerEvent::Error(_) => {},
			}
			count += 1;
        });
 
		assert_eq!(count, 2);
    }

    // #[test]
    // fn test_incomplete_header() {
    //     let mut multiplexer = Multiplexer::new(10);
    //     let mut events = Vec::new();

    //     // Simulated data with an incomplete header
    //     let data_part1 = [0x11, 0x00, 0x00, 0x00, 0x00];
    //     let data_part2 = [0x00, 0x00, 0x00, 0x05, 0x00, 0x41, 0x42];

    //     multiplexer.handle_data(&data_part1, |event| {
    //         events.push(event);
    //     });

    //     // No events should be triggered yet
    //     assert!(events.is_empty());

    //     multiplexer.handle_data(&data_part2, |event| {
    //         events.push(event);
    //     });

    //     assert!(matches!(events[0], MultiplexerEvent::StreamStarted { stream_id, length } if stream_id == 1 && length == 5));
    //     assert!(matches!(events[1], MultiplexerEvent::DataPointer { stream_id, data } if stream_id == 1 && data == b"AB"));
    // }

    // #[test]
    // fn test_stream_end() {
    //     let mut multiplexer = Multiplexer::new(10);
    //     let mut events = Vec::new();

    //     // Simulated data for stream start and end
    //     let data = [
    //         0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Stream ID and stage (STREAM_START)
    //         0x05, 0x00,                                     // Length of the stream
    //         0x41, 0x42,                                     // Payload data
    //         0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Stream ID and stage (STREAM_END)
    //     ];

    //     multiplexer.handle_data(&data, |event| {
    //         events.push(event);
    //     });

    //     assert!(matches!(events[0], MultiplexerEvent::StreamStarted { stream_id, length } if stream_id == 1 && length == 5));
    //     assert!(matches!(events[1], MultiplexerEvent::DataPointer { stream_id, data } if stream_id == 1 && data == b"AB"));
    //     assert!(matches!(events[2], MultiplexerEvent::StreamEnded { stream_id } if stream_id == 1));
    // }

    // #[test]
    // fn test_error_on_invalid_stage() {
    //     let mut multiplexer = Multiplexer::new(10);
    //     let mut events = Vec::new();

    //     // Simulated data with an invalid stage
    //     let data = [
    //         0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Invalid stage
    //         0x05, 0x00,                                     // Length
    //     ];

    //     multiplexer.handle_data(&data, |event| {
    //         events.push(event);
    //     });

    //     assert!(matches!(events[0], MultiplexerEvent::Error(_)));
    // }
}
