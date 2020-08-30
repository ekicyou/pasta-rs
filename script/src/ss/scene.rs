use crate::error::*;
use futures::channel::mpsc::{channel, Receiver, Sender};
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use futures::task::{LocalSpawn, LocalSpawnExt};
use rhai::ImmutableString;
use std::iter::Iterator;

#[derive(Debug)]
enum Command {
    Next,
    Cancel,
}

#[derive(Debug)]
pub struct SceneIter {
    tc: Sender<Command>,
    rr: Receiver<Option<ImmutableString>>,
}

impl SceneIter {
    pub async fn next(&mut self) -> PastaResult<Option<ImmutableString>> {
        self.tc.send(Command::Next).await?;
        Ok(self.rr.next().await.flatten())
    }
}

#[derive(Debug)]
pub struct ScenePlayer {
    rc: Receiver<Command>,
    tr: Sender<Option<ImmutableString>>,
    scene: Scene,
}

#[derive(Debug, Default, Clone)]
pub struct Scene {}

impl ScenePlayer {
    pub fn start<S: LocalSpawn>(spawner: &S, scene: Scene) -> PastaResult<SceneIter> {
        let (tc, rc) = channel::<Command>(0);
        let (tr, rr) = channel::<Option<ImmutableString>>(0);
        let player = ScenePlayer {
            rc: rc,
            tr: tr,
            scene: scene,
        };
        spawner.spawn_local(player.schedule())?;
        Ok(SceneIter { tc: tc, rr: rr })
    }

    async fn schedule(mut self) {
        loop {
            // 何も返さないループ
            match self.rc.next().await {
                Some(Command::Next) => {
                    self.action_scene().await;
                    self.tr.send(None).await;
                    break;
                }
                _ => {
                    break;
                }
            }
        }
    }

    /// シーンを再生します。
    async fn action_scene(&mut self) {
        return;
    }
}

impl Iterator for ScenePlayer {
    type Item = ImmutableString;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

/*
pub struct ScenePlayer {
    tx: Sender<Option<ImmutableString>>,
    rx: Receiver<Request>,
}
impl ScenePlayer {
    /// 終わるまでsinkにカットを返し続ける非同期関数
    pub fn start(
        executor: &mut LocalPool,
    ) -> PastaResult<(Sender<Request>, Receiver<Option<ImmutableString>>)> {
        let (tx1, rx) = channel::<Request>(0);
        let (tx, rx1) = channel::<Option<ImmutableString>>(0);
        let spawner = executor.spawner();
        let player = Self::new(tx, rx);
        spawner.spawn_local(player.service())?;
        executor.run_until_stalled();

        Ok((tx1, rx1))
    }

    fn new(tx: Sender<Option<ImmutableString>>, rx: Receiver<Request>) -> ScenePlayer {
        ScenePlayer { tx: tx, rx: rx }
    }

    /// 非同期タスク
    async fn service(mut self) {
        loop {
            // 何も返さないループ
            match self.rx.next().await {
                Cancel => {
                    break;
                }
                Next => {
                    self.tx.send(None).await;
                    break;
                }
            }
        }
    }
}
*/
