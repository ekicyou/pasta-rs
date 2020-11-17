use futures::channel::mpsc::{channel, Receiver, Sender};
use futures::sink::SinkExt;
use futures::stream::StreamExt;

#[derive(Debug)]
enum Command {
    Next,
    Cancel,
}

pub fn yield_redume<T>() -> (YieldBuilder<T>, Resume<T>) {
    let (tc, rc) = channel::<Command>(0);
    let (tr, rr) = channel::<T>(0);
    (
        YieldBuilder(Yield { rc: rc, tr: tr }),
        Resume { tc: tc, rr: rr },
    )
}

#[derive(Debug)]
pub struct YieldBuilder<T>(Yield<T>);

unsafe impl<T: Send> Send for YieldBuilder<T> {}

impl<T> YieldBuilder<T> {
    pub async fn start(self) -> Option<Yield<T>> {
        let mut yy = self.0;
        if yy.has_next().await {
            Some(yy)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Yield<T> {
    rc: Receiver<Command>,
    tr: Sender<T>,
}
impl<T> Yield<T> {
    async fn has_next(&mut self) -> bool {
        match self.rc.next().await {
            Some(Command::Next) => true,
            _ => false,
        }
    }

    pub async fn yield_async(&mut self, value: T) -> bool {
        match self.tr.send(value).await {
            Ok(_) => self.has_next().await,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Resume<T> {
    tc: Sender<Command>,
    rr: Receiver<T>,
}
impl<T> Resume<T> {
    pub async fn next(&mut self) -> Option<T> {
        match self.tc.send(Command::Next).await {
            Ok(_) => self.rr.next().await,
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::*;

    #[test]
    fn local_pool() -> PastaResult<()> {
        use futures::executor::LocalPool;
        use futures::task::LocalSpawnExt;
        let (mut yy, mut rr) = yield_redume::<isize>();

        let write_task = async move {
            if let Some(mut yy) = yy.start().await {
                if !yy.yield_async(1).await {
                    return;
                };
                if !yy.yield_async(2).await {
                    return;
                };
                if !yy.yield_async(3).await {
                    return;
                };
                if !yy.yield_async(4).await {
                    return;
                };
                if !yy.yield_async(5).await {
                    return;
                };
            }
        };
        let read_task = async move {
            let mut all = 0;
            while let Some(a) = rr.next().await {
                all += a;
            }
            all
        };

        let mut exec = LocalPool::new();
        let spawn = exec.spawner();
        spawn.spawn_local(write_task)?;
        let rc = exec.run_until(read_task);
        assert_eq!(rc, 15);
        Ok(())
    }

    #[test]
    fn thread_pool() -> PastaResult<()> {
        use futures::executor::block_on;
        use futures::executor::ThreadPool;
        use futures::task::SpawnExt;
        let (mut yy, mut rr) = yield_redume::<isize>();

        let write_task = async move {
            if let Some(mut yy) = yy.start().await {
                if !yy.yield_async(1).await {
                    return;
                };
                if !yy.yield_async(2).await {
                    return;
                };
                if !yy.yield_async(3).await {
                    return;
                };
                if !yy.yield_async(4).await {
                    return;
                };
                if !yy.yield_async(5).await {
                    return;
                };
            }
        };
        let read_task = async move {
            let mut all = 0;
            while let Some(a) = rr.next().await {
                all += a;
            }
            all
        };

        let exec = ThreadPool::new()?;
        exec.spawn_ok(write_task);
        let rc = block_on(read_task);
        assert_eq!(rc, 15);
        Ok(())
    }
}
