//! Shared state for Dora↔UI communication
//!
//! Replaces channel-based communication with direct shared memory access.
//! Uses dirty tracking to minimize UI updates - only redraw when data changes.

use parking_lot::RwLock;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::data::{AudioData, ChatMessage, LogEntry};

/// Dirty-trackable collection for any data type
pub struct DirtyVec<T> {
    data: RwLock<Vec<T>>,
    dirty: AtomicBool,
    max_size: usize,
}

impl<T: Clone> DirtyVec<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            data: RwLock::new(Vec::new()),
            dirty: AtomicBool::new(false),
            max_size,
        }
    }

    /// Push item, mark dirty, enforce max size
    pub fn push(&self, item: T) {
        let mut data = self.data.write();
        data.push(item);
        if data.len() > self.max_size {
            data.remove(0);
        }
        self.dirty.store(true, Ordering::Release);
    }

    /// Read all data if dirty, clearing dirty flag
    pub fn read_if_dirty(&self) -> Option<Vec<T>> {
        if self.dirty.swap(false, Ordering::AcqRel) {
            Some(self.data.read().clone())
        } else {
            None
        }
    }

    /// Read all data unconditionally
    pub fn read_all(&self) -> Vec<T> {
        self.data.read().clone()
    }

    /// Clear all data
    pub fn clear(&self) {
        self.data.write().clear();
        self.dirty.store(true, Ordering::Release);
    }

    /// Check if dirty without consuming
    pub fn is_dirty(&self) -> bool {
        self.dirty.load(Ordering::Acquire)
    }
}

/// Dirty-trackable single value
pub struct DirtyValue<T> {
    data: RwLock<T>,
    dirty: AtomicBool,
}

impl<T: Clone + Default> DirtyValue<T> {
    pub fn new(initial: T) -> Self {
        Self {
            data: RwLock::new(initial),
            dirty: AtomicBool::new(false),
        }
    }

    /// Set value and mark dirty
    pub fn set(&self, value: T) {
        *self.data.write() = value;
        self.dirty.store(true, Ordering::Release);
    }

    /// Read value if dirty, clearing dirty flag
    pub fn read_if_dirty(&self) -> Option<T> {
        if self.dirty.swap(false, Ordering::AcqRel) {
            Some(self.data.read().clone())
        } else {
            None
        }
    }

    /// Read value unconditionally
    pub fn read(&self) -> T {
        self.data.read().clone()
    }
}

impl<T: Default> Default for DirtyValue<T> {
    fn default() -> Self {
        Self {
            data: RwLock::new(T::default()),
            dirty: AtomicBool::new(false),
        }
    }
}

/// Chat state with streaming message consolidation
pub struct ChatState {
    messages: RwLock<Vec<ChatMessage>>,
    dirty: AtomicBool,
    max_messages: usize,
}

impl ChatState {
    pub fn new(max_messages: usize) -> Self {
        Self {
            messages: RwLock::new(Vec::new()),
            dirty: AtomicBool::new(false),
            max_messages,
        }
    }

    /// Push message with automatic streaming consolidation
    ///
    /// If message is streaming, ACCUMULATES content to existing streaming message from same sender/session.
    /// If message is complete, finalizes any existing streaming message.
    pub fn push(&self, msg: ChatMessage) {
        let mut messages = self.messages.write();

        // Find existing streaming message from same sender + session
        // IMPORTANT: Only match if BOTH have valid session_ids (not None)
        // to prevent incorrectly merging messages from different participants
        let existing_idx = messages.iter().position(|m| {
            m.sender == msg.sender
                && m.is_streaming
                && m.session_id.is_some()
                && m.session_id == msg.session_id
        });

        if let Some(idx) = existing_idx {
            // ACCUMULATE content for streaming messages (append, not replace)
            messages[idx].content.push_str(&msg.content);
            if !msg.is_streaming {
                // Finalize: mark as complete
                messages[idx].is_streaming = false;
                messages[idx].timestamp = msg.timestamp;
            }
        } else {
            // New message
            messages.push(msg);

            // Enforce max size
            if messages.len() > self.max_messages {
                messages.remove(0);
            }
        }

        self.dirty.store(true, Ordering::Release);
    }

    /// Read all messages if dirty
    pub fn read_if_dirty(&self) -> Option<Vec<ChatMessage>> {
        if self.dirty.swap(false, Ordering::AcqRel) {
            Some(self.messages.read().clone())
        } else {
            None
        }
    }

    /// Read all messages unconditionally
    pub fn read_all(&self) -> Vec<ChatMessage> {
        self.messages.read().clone()
    }

    /// Clear all messages
    pub fn clear(&self) {
        self.messages.write().clear();
        self.dirty.store(true, Ordering::Release);
    }

    /// Get message count
    pub fn len(&self) -> usize {
        self.messages.read().len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.messages.read().is_empty()
    }
}

/// Audio state - ring buffer for audio chunks
/// Unlike other states, audio is consumed (drained) not just read
pub struct AudioState {
    chunks: RwLock<VecDeque<AudioData>>,
    max_chunks: usize,
}

impl AudioState {
    pub fn new(max_chunks: usize) -> Self {
        Self {
            chunks: RwLock::new(VecDeque::new()),
            max_chunks,
        }
    }

    /// Push audio chunk (producer - bridge thread)
    pub fn push(&self, chunk: AudioData) {
        let mut chunks = self.chunks.write();
        chunks.push_back(chunk);
        // Bound to prevent memory growth
        while chunks.len() > self.max_chunks {
            chunks.pop_front();
        }
    }

    /// Drain all available chunks (consumer - audio thread)
    pub fn drain(&self) -> Vec<AudioData> {
        self.chunks.write().drain(..).collect()
    }

    /// Drain up to N chunks
    pub fn drain_n(&self, n: usize) -> Vec<AudioData> {
        let mut chunks = self.chunks.write();
        let drain_count = n.min(chunks.len());
        chunks.drain(..drain_count).collect()
    }

    /// Check if audio available
    pub fn has_audio(&self) -> bool {
        !self.chunks.read().is_empty()
    }

    /// Get pending chunk count
    pub fn len(&self) -> usize {
        self.chunks.read().len()
    }

    /// Clear all pending audio
    pub fn clear(&self) {
        self.chunks.write().clear();
    }
}

/// Dora connection status
#[derive(Debug, Clone, Default)]
pub struct DoraStatus {
    /// List of connected bridge node IDs
    pub active_bridges: Vec<String>,
    /// Last error message if any
    pub last_error: Option<String>,
}

/// Unified shared state for all Dora↔UI communication
///
/// This replaces the channel-based architecture with direct shared memory.
/// Bridges write to state, UI reads from state on a single timer.
pub struct SharedDoraState {
    /// Chat messages (with streaming consolidation)
    pub chat: ChatState,

    /// Audio chunks (ring buffer, consumed by audio player)
    pub audio: AudioState,

    /// Log entries
    pub logs: DirtyVec<LogEntry>,

    /// Connection/dataflow status
    pub status: DirtyValue<DoraStatus>,
}

impl SharedDoraState {
    /// Create new shared state with default capacities
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            chat: ChatState::new(500),   // 500 max chat messages
            audio: AudioState::new(100), // 100 max pending audio chunks
            logs: DirtyVec::new(1000),   // 1000 max log entries
            status: DirtyValue::default(),
        })
    }

    /// Create with custom capacities
    pub fn with_capacities(max_chat: usize, max_audio_chunks: usize, max_logs: usize) -> Arc<Self> {
        Arc::new(Self {
            chat: ChatState::new(max_chat),
            audio: AudioState::new(max_audio_chunks),
            logs: DirtyVec::new(max_logs),
            status: DirtyValue::default(),
        })
    }

    /// Clear all state (on dataflow stop/reset)
    pub fn clear_all(&self) {
        self.chat.clear();
        self.audio.clear();
        self.logs.clear();
        self.status.set(DoraStatus::default());
    }

    /// Add active bridge
    pub fn add_bridge(&self, bridge_id: String) {
        let mut status = self.status.read();
        if !status.active_bridges.contains(&bridge_id) {
            status.active_bridges.push(bridge_id);
            self.status.set(status);
        }
    }

    /// Remove active bridge
    pub fn remove_bridge(&self, bridge_id: &str) {
        let mut status = self.status.read();
        status.active_bridges.retain(|b| b != bridge_id);
        self.status.set(status);
    }

    /// Set error status
    pub fn set_error(&self, error: Option<String>) {
        let mut status = self.status.read();
        status.last_error = error;
        self.status.set(status);
    }
}

impl Default for SharedDoraState {
    fn default() -> Self {
        Self {
            chat: ChatState::new(500),
            audio: AudioState::new(100),
            logs: DirtyVec::new(1000),
            status: DirtyValue::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::MessageRole;

    #[test]
    fn test_dirty_vec() {
        let vec: DirtyVec<i32> = DirtyVec::new(5);

        // Initially not dirty
        assert!(vec.read_if_dirty().is_none());

        // Push makes dirty
        vec.push(1);
        vec.push(2);

        // Read clears dirty
        let data = vec.read_if_dirty().unwrap();
        assert_eq!(data, vec![1, 2]);

        // Now not dirty
        assert!(vec.read_if_dirty().is_none());

        // Max size enforcement
        for i in 0..10 {
            vec.push(i);
        }
        let data = vec.read_all();
        assert_eq!(data.len(), 5); // Max size
        assert_eq!(data, vec![5, 6, 7, 8, 9]); // Oldest removed
    }

    #[test]
    fn test_chat_streaming_consolidation() {
        let chat = ChatState::new(100);

        // First streaming chunk
        chat.push(ChatMessage {
            content: "Hello".to_string(),
            sender: "Bot".to_string(),
            role: MessageRole::Assistant,
            timestamp: 1000,
            is_streaming: true,
            session_id: Some("s1".to_string()),
        });

        // Second streaming chunk - should ACCUMULATE, not replace
        chat.push(ChatMessage {
            content: ", world".to_string(),
            sender: "Bot".to_string(),
            role: MessageRole::Assistant,
            timestamp: 1001,
            is_streaming: true,
            session_id: Some("s1".to_string()),
        });

        // Should still be one message with accumulated content
        let messages = chat.read_all();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].content, "Hello, world"); // Accumulated!
        assert!(messages[0].is_streaming);

        // Finalize with final chunk
        chat.push(ChatMessage {
            content: "!".to_string(),
            sender: "Bot".to_string(),
            role: MessageRole::Assistant,
            timestamp: 1002,
            is_streaming: false,
            session_id: Some("s1".to_string()),
        });

        let messages = chat.read_all();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].content, "Hello, world!"); // Full accumulated content
        assert!(!messages[0].is_streaming);
    }

    #[test]
    fn test_chat_multi_participant_isolation() {
        let chat = ChatState::new(100);

        // Two participants streaming concurrently with different session_ids
        chat.push(ChatMessage {
            content: "Hello from ".to_string(),
            sender: "Tutor".to_string(),
            role: MessageRole::Assistant,
            timestamp: 1000,
            is_streaming: true,
            session_id: Some("session_tutor".to_string()),
        });

        chat.push(ChatMessage {
            content: "Hi from ".to_string(),
            sender: "Student".to_string(),
            role: MessageRole::Assistant,
            timestamp: 1001,
            is_streaming: true,
            session_id: Some("session_student".to_string()),
        });

        // Continue streaming - each should accumulate separately
        chat.push(ChatMessage {
            content: "tutor!".to_string(),
            sender: "Tutor".to_string(),
            role: MessageRole::Assistant,
            timestamp: 1002,
            is_streaming: false,
            session_id: Some("session_tutor".to_string()),
        });

        chat.push(ChatMessage {
            content: "student!".to_string(),
            sender: "Student".to_string(),
            role: MessageRole::Assistant,
            timestamp: 1003,
            is_streaming: false,
            session_id: Some("session_student".to_string()),
        });

        // Should have 2 separate messages, properly accumulated
        let messages = chat.read_all();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].content, "Hello from tutor!");
        assert_eq!(messages[0].sender, "Tutor");
        assert_eq!(messages[1].content, "Hi from student!");
        assert_eq!(messages[1].sender, "Student");
    }

    #[test]
    fn test_chat_no_session_id_creates_new_message() {
        let chat = ChatState::new(100);

        // Messages without session_id should NOT be consolidated
        chat.push(ChatMessage {
            content: "First".to_string(),
            sender: "Bot".to_string(),
            role: MessageRole::Assistant,
            timestamp: 1000,
            is_streaming: true,
            session_id: None, // No session_id
        });

        chat.push(ChatMessage {
            content: "Second".to_string(),
            sender: "Bot".to_string(),
            role: MessageRole::Assistant,
            timestamp: 1001,
            is_streaming: true,
            session_id: None, // No session_id
        });

        // Should be 2 separate messages (not consolidated)
        let messages = chat.read_all();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].content, "First");
        assert_eq!(messages[1].content, "Second");
    }

    #[test]
    fn test_audio_drain() {
        let audio = AudioState::new(10);

        audio.push(AudioData {
            samples: vec![0.1, 0.2],
            sample_rate: 44100,
            channels: 1,
            participant_id: None,
            question_id: None,
        });
        audio.push(AudioData {
            samples: vec![0.3, 0.4],
            sample_rate: 44100,
            channels: 1,
            participant_id: None,
            question_id: None,
        });

        assert_eq!(audio.len(), 2);

        let chunks = audio.drain();
        assert_eq!(chunks.len(), 2);
        assert_eq!(audio.len(), 0);
    }
}
