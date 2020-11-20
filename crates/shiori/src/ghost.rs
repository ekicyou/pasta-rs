use crate::event::*;
use log::*;
use shiori3::*;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

#[derive(Debug)]
pub struct Ghost {
    tx: mpsc::Sender<GhostEvent>,
}
impl Drop for Ghost {
    fn drop(&mut self) {
        let (tx, rx) = mpsc::sync_channel(0);
        let _ = self.tx.send(GhostEvent::Unload(tx));
        let _ = rx.recv();
    }
}

impl Ghost {
    pub fn new(hinst: usize, load_dir: PathBuf) -> ApiResult<Ghost> {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let mut ch: GhostChannel = Default::default();
            match ch.message_loop(rx) {
                Err(e) => error!("{}", e),
                _ => {}
            };
        });
        tx.send(GhostEvent::Load(hinst, load_dir))?;
        Ok(Self { tx: tx })
    }

    pub fn shiori_request(&mut self, req: GCowStr) -> ApiResult<String> {
        let args = ShioriRequestArgs::new(req)?;
        let rc = match args.req().header.method {
            ShioriRequestRule::get => {
                let (tx, rx) = mpsc::sync_channel(0);
                let ev = GhostEvent::ShioriRequest(args, tx);
                self.tx.send(ev)?;
                rx.recv()?
            }
            ShioriRequestRule::notify => {
                let ev = GhostEvent::ShioriNotify(args);
                self.tx.send(ev)?;
                Err(ApiError::NoContent)?
            }
            _ => Err(ApiError::Unimplemented)?,
        };
        rc
    }
}

#[derive(Default, Debug)]
pub struct GhostChannel {
    hinst: usize,
    load_dir: PathBuf,
}

impl GhostChannel {
    fn message_loop(&mut self, rx: mpsc::Receiver<GhostEvent>) -> ApiResult<()> {
        loop {
            match rx.recv()? {
                GhostEvent::Unload(tx) => {
                    self.unload(tx)?;
                    return Ok(());
                }
                GhostEvent::Load(hinst, load_dir) => {
                    self.load(hinst, load_dir)?;
                }
                GhostEvent::ShioriRequest(args, tx) => {
                    self.shiori_request(args, tx)?;
                }
                GhostEvent::ShioriNotify(args) => {
                    self.shiori_notify(args)?;
                }
            }
        }
    }

    fn load(&mut self, hinst: usize, load_dir: PathBuf) -> ApiResult<()> {
        self.load_impl(hinst, load_dir)
    }
    fn unload(&mut self, tx: mpsc::SyncSender<()>) -> ApiResult<()> {
        let rc = self.unload_impl();
        tx.send(())?;
        rc
    }
    fn shiori_request(
        &mut self,
        args: ShioriRequestArgs,
        tx: mpsc::SyncSender<ApiResult<String>>,
    ) -> ApiResult<()> {
        tx.send(self.shiori_request_impl(args))?;
        Ok(())
    }
    fn shiori_notify(&mut self, args: ShioriRequestArgs) -> ApiResult<()> {
        match self.shiori_notify_impl(args) {
            Err(e) => error!("{}", e),
            _ => {}
        };
        Ok(())
    }

    fn load_impl(&mut self, hinst: usize, load_dir: PathBuf) -> ApiResult<()> {
        self.hinst = hinst;
        self.load_dir = load_dir;
        Ok(())
    }
    fn unload_impl(&mut self) -> ApiResult<()> {
        Ok(())
    }
    fn shiori_request_impl(&mut self, args: ShioriRequestArgs) -> ApiResult<String> {
        Err(ApiError::NoContent)
    }
    fn shiori_notify_impl(&mut self, args: ShioriRequestArgs) -> ApiResult<()> {
        Err(ApiError::NoContent)
    }
}
