use core::future::Future;

/// The current status of the firmware on a device
pub struct Status<'m> {
    /// Current firmware version
    pub current_version: &'m [u8],
    /// Offset written of next firmware
    pub next_offset: usize,
    /// Next version being written
    pub next_version: Option<&'m [u8]>,
}

pub trait FirmwareDevice {
    const MTU: u32;
    type Error;

    // Future returned by status
    type StatusFuture<'m>: Future<Output = Result<Status<'m>, Self::Error>> + 'm
    where
        Self: 'm;
    /// Return the status of the currently running firmware.
    fn status<'m>(&'m mut self) -> Self::StatusFuture<'m>;

    // Future returned by start
    type StartFuture<'m>: Future<Output = Result<(), Self::Error>> + 'm
    where
        Self: 'm;
    /// Prepare for starting the firmware update process.
    fn start<'m>(&'m mut self, version: &'m [u8]) -> Self::StartFuture<'m>;

    /// Future returned by write
    type WriteFuture<'m>: Future<Output = Result<(), Self::Error>> + 'm
    where
        Self: 'm;
    /// Write a block of firmware at the expected offset.
    fn write<'m>(&'m mut self, offset: u32, data: &'m [u8]) -> Self::WriteFuture<'m>;

    /// Future returned by update
    type UpdateFuture<'m>: Future<Output = Result<(), Self::Error>> + 'm
    where
        Self: 'm;
    /// Finish the firmware write and mark device to be updated
    fn update<'m>(&'m mut self, version: &'m [u8], checksum: [u8; 32]) -> Self::UpdateFuture<'m>;

    /// Future returned by synced
    type SyncedFuture<'m>: Future<Output = Result<(), Self::Error>> + 'm
    where
        Self: 'm;
    /// Mark firmware as being in sync with the expected
    fn synced<'m>(&'m mut self) -> Self::SyncedFuture<'m>;

    /// Reset the device.
    fn reset(&mut self) -> !;
}