use std::{net::ToSocketAddrs, time::Duration};

use log::{error, info, warn};
use mpd::{song::QueuePlace, Client, Id, Song, Stats, Status};

pub struct Mpd {
    connection: Client,
    status: Option<Status>,
    stats: Option<Stats>,
}

impl Mpd {
    pub fn new<T: ToSocketAddrs>(addr: T) -> Self {
        Self {
            connection: Client::connect(addr)
                .map_err(|err| error!("Failed to Connect to MPD Server: {err}"))
                .unwrap(),
            status: None,
            stats: None,
        }
    }

    pub fn update_loop(&mut self) {
        self.status = self.connection.status().map(Some).unwrap_or(None);
        self.stats = self.connection.stats().map(Some).unwrap_or(None);
    }

    pub fn set_volume(&mut self, volume: i8) {
        let _ = self
            .connection
            .volume(volume)
            .map_err(|err| warn!("Failed to set Volume: {err}"));
    }

    pub fn get_volume(&self) -> Option<i8> {
        if let Some(status) = &self.status {
            return Some(status.volume);
        }
        None
    }

    pub fn toggle_repeat(&mut self) {
        let repeat_state = self.get_repeat().unwrap_or(false);
        let single_state = self.get_single().unwrap_or(false);

        if repeat_state && single_state {
            let _ = self
                .connection
                .repeat(false)
                .map_err(|err| warn!("Failed to toggle Repeat, Set Repeat: {err}"));
            let _ = self
                .connection
                .single(false)
                .map_err(|err| warn!("Failed to toggle Repeat, Set Single: {err}"));
        } else if repeat_state {
            let _ = self
                .connection
                .single(true)
                .map_err(|err| warn!("Failed to toggle Repeat, Set Single: {err}"));
        } else {
            let _ = self
                .connection
                .repeat(true)
                .map_err(|err| warn!("Failed to toggle Repeat, Set Repeat: {err}"));
        }
    }

    pub fn get_repeat(&self) -> Option<bool> {
        if let Some(status) = &self.status {
            return Some(status.repeat);
        }
        None
    }

    pub fn get_single(&self) -> Option<bool> {
        if let Some(status) = &self.status {
            return Some(status.single);
        }
        None
    }

    pub fn toggle_shuffle(&mut self) {
        let _ = self
            .connection
            .repeat(!self.get_shuffle().unwrap_or(false))
            .map_err(|err| warn!("Failed to toggle Shuffle: {err}"));
    }

    pub fn get_shuffle(&self) -> Option<bool> {
        if let Some(status) = &self.status {
            return Some(status.random);
        }
        None
    }

    pub fn toggle_consume(&mut self) {
        let _ = self
            .connection
            .consume(!self.get_consume().unwrap_or(false))
            .map_err(|err| warn!("Failed to toggle Consume: {err}"));
    }

    pub fn get_consume(&self) -> Option<bool> {
        if let Some(status) = &self.status {
            return Some(status.consume);
        }
        None
    }

    pub fn forward(&mut self, duration: Duration) {
        if let Some((time, ..)) = self.get_time() {
            self.seek(time + duration)
        } else {
            info!("A song must be playing")
        };
    }

    pub fn rewind(&mut self, duration: Duration) {
        if let Some((time, ..)) = self.get_time() {
            self.seek(time - duration)
        } else {
            info!("A song must be playing")
        };
    }

    pub fn seek(&mut self, seek_position: Duration) {
        let _ = self
            .connection
            .rewind(seek_position)
            .map_err(|err| error!("Failed to Seek: {err}"));
    }

    pub fn get_time(&self) -> Option<(Duration, Duration)> {
        if let Some(status) = &self.status {
            return status.time;
        }
        None
    }

    pub fn get_queue(&mut self) -> Vec<Song> {
        if let Ok(queue) = self
            .connection
            .queue()
            .map_err(|err| error!("Failed to Get Queue: {err}"))
        {
            queue
        } else {
            vec![]
        }
    }

    pub fn get_all_songs(&mut self) -> Vec<Song> {
        if let Ok(list) = self
            .connection
            .listall()
            .map_err(|err| error!("Failed to Get Queue: {err}"))
        {
            list
        } else {
            vec![]
        }
    }

    pub fn next_song(&mut self) {
        let _ = self.connection.next();
    }

    pub fn prev_song(&mut self) {
        let _ = self.connection.prev();
    }

    pub fn toggle_play(&mut self) {
        if let Some(status) = &self.status {
            let _ = match status.state {
                mpd::State::Stop => self.connection.play(),
                mpd::State::Play | mpd::State::Pause => self.connection.toggle_pause(),
            };
        }
    }

    pub fn stop_playback(&mut self) {
        let _ = self.connection.stop();
    }

    pub fn delete_from_queue(&mut self, song_id: Id) {
        let _ = self.connection.delete(song_id);
    }

    pub fn push_into_queue(&mut self, song: Song){
        let _ = self.connection.push(song);
    }
}
